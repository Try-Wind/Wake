use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{collections::HashMap, path::PathBuf};
use wake_llm::{LlmClient, ToolCallMethod};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider: String,
    pub env_vars: std::collections::HashMap<String, String>,
    pub model: String,
    pub tool_method: ToolCallMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeConfig {
    pub providers: Vec<ProviderConfig>,
    pub selected_provider: usize,
}

impl WakeConfig {
    pub async fn pull_from_url(url: Url) -> Result<Self, Box<dyn std::error::Error>> {
        let response = reqwest::get(url).await?;
        let config_json = response.text().await?;
        let config: WakeConfig = serde_json::from_str(&config_json)?;
        Ok(config)
    }

    pub fn add_provider(
        &mut self,
        provider: String,
        env_vars: std::collections::HashMap<String, String>,
        model: String,
    ) -> usize {
        let provider_config = ProviderConfig {
            provider,
            env_vars,
            model,
            tool_method: ToolCallMethod::FunctionCall,
        };

        self.providers.push(provider_config);
        self.providers.len() - 1
    }

    pub fn is_duplicate_config(
        &self,
        provider_name: &str,
        env_vars: &std::collections::HashMap<String, String>,
        model: &str,
    ) -> bool {
        self.providers.iter().any(|provider_config| {
            provider_config.provider == provider_name
                && provider_config.env_vars == *env_vars
                && provider_config.model.eq(model)
        })
    }

    pub fn get_selected_provider(&self) -> Option<&ProviderConfig> {
        self.providers.get(self.selected_provider)
    }

    pub fn get_selected_provider_mut(&mut self) -> Option<&mut ProviderConfig> {
        self.providers.get_mut(self.selected_provider)
    }

    pub fn set_selected_provider(&mut self, index: usize) -> Result<(), String> {
        if index < self.providers.len() {
            self.selected_provider = index;
            Ok(())
        } else {
            Err(format!(
                "Provider index {} out of bounds (have {} providers)",
                index,
                self.providers.len()
            ))
        }
    }

    pub fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = std::env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|_| {
                dirs::home_dir()
                    .map(|home| home.join(".config"))
                    .ok_or("Could not find home directory")
            })?;

        let wake_config_dir = config_dir.join("wake");
        std::fs::create_dir_all(&wake_config_dir)?;

        Ok(wake_config_dir.join("auth.config"))
    }

    pub fn load() -> Result<WakeConfig, Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Err("config file does not exist".into());
        }

        let content = fs::read_to_string(config_path)?;
        let mut config: WakeConfig = serde_json::from_str(&content)?;

        // Validate selected_provider index
        if config.providers.is_empty() {
            config.selected_provider = 0;
        } else if config.selected_provider >= config.providers.len() {
            config.selected_provider = 0; // Reset to first provider if index is invalid
        }

        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, content)?;

        // Set file permissions to 600 (user read/write only) on Unix systems
        #[cfg(unix)]
        {
            let mut perms = fs::metadata(&config_path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&config_path, perms)?;
        }

        Ok(())
    }

    pub fn exists() -> bool {
        Self::config_path()
            .map(|path| path.exists())
            .unwrap_or(false)
    }

    /// Set environment variables from the currently selected provider
    pub fn set_env_vars(&self) {
        if let Some(provider_config) = self.get_selected_provider() {
            for (name, value) in &provider_config.env_vars {
                std::env::set_var(name, value);
            }
            // Set model-specific environment variable if model is specified
            std::env::set_var("WAKE_MODEL", &provider_config.model);
            // Set provider-specific environment variable
            std::env::set_var("WAKE_PROVIDER", &provider_config.provider);
        }
    }

    pub fn remove_provider(&mut self, index: usize) -> Result<ProviderConfig, String> {
        if index >= self.providers.len() {
            return Err(format!(
                "Provider index {} out of bounds (have {} providers)",
                index,
                self.providers.len()
            ));
        }

        if self.providers.len() == 1 {
            return Err("Cannot remove the last provider".to_string());
        }

        let removed = self.providers.remove(index);

        // Adjust selected_provider if needed
        if self.selected_provider >= self.providers.len() {
            self.selected_provider = self.providers.len() - 1;
        } else if self.selected_provider > index {
            self.selected_provider -= 1;
        }

        Ok(removed)
    }

    pub fn list_providers(&self) -> Vec<(usize, &str, &str)> {
        self.providers
            .iter()
            .enumerate()
            .map(|(i, config)| (i, config.provider.as_str(), config.model.as_str()))
            .collect()
    }

    pub fn find_providers_by_type(&self, provider_type: &str) -> Vec<usize> {
        self.providers
            .iter()
            .enumerate()
            .filter_map(|(i, config)| {
                if config.provider == provider_type {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for WakeConfig {
    fn default() -> Self {
        Self {
            // default to ovhcloud qwen3 in anonymous mode
            providers: vec![ProviderConfig {
                provider: "ovhcloud".to_string(),
                env_vars: HashMap::from([(
                    String::from("OVH_BASE_URL"),
                    String::from(
                        "https://qwen-3-32b.endpoints.kepler.ai.cloud.ovh.net/api/openai_compat/v1",
                    ),
                )]),
                model: "Qwen3-32B".to_string(),
                tool_method: ToolCallMethod::FunctionCall,
            }],
            selected_provider: 0,
        }
    }
}

impl WakeConfig {
    pub async fn get_llm() -> Result<(LlmClient, String), Box<dyn std::error::Error>> {
        let config = WakeConfig::load().unwrap_or_else(|_| WakeConfig::default());

        config.set_env_vars();

        let llm = if let Some(provider_config) = config.get_selected_provider() {
            LlmClient::create_provider(&provider_config.provider, &provider_config.env_vars)
                .map_err(|e| {
                    format!(
                        "Failed to create {} client: {}",
                        provider_config.provider, e
                    )
                })?
        } else {
            return Err("No provider configured".into());
        };

        let model = llm
            .default_model()
            .await
            .map_err(|_| "no Model available")?;
        Ok((llm, model))
    }
}
