use super::config_env::ModalEnvs;
use super::config_list::ModalConfig;
use super::config_model::ModalModel;
use super::config_providers::ModalProviders;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::disable_raw_mode;
use futures::StreamExt;
use ratatui::{
    layout::Rect, prelude::CrosstermBackend, Frame, Terminal, TerminalOptions, Viewport,
};
use std::collections::HashMap;
use std::io;
use wake_core::config::config::WakeConfig;
use wake_llm::client::LlmClient;
use wake_llm::provider::ProviderInfo;

pub enum NavAction {
    Back,
    Next,
    Done,
    None,
}

#[derive(Debug)]
pub enum AuthState {
    Start(ModalConfig),
    SelectProvider(ModalProviders),
    EnvConfig(ModalEnvs),
    ModelSelection(ModalModel),
    Done,
}

pub struct AppAuth {
    terminal: Option<Terminal<CrosstermBackend<io::Stdout>>>,
    state: AuthState,
    height: u16,
    exit: bool,
}

impl AppAuth {
    pub fn new() -> Self {
        let config_list = ModalConfig::new();

        Self {
            terminal: None,
            state: AuthState::Start(config_list),
            exit: false,
            height: 0 as u16,
        }
    }

    pub async fn run(&mut self) {
        let result = self.try_run().await;
        let _ = disable_raw_mode();

        if let Err(e) = result {
            println!();
            eprintln!("\x1b[2m{}\x1b[0m\r\n", e);
        }

        println!();
    }

    pub async fn try_run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize terminal with current modal height
        self.height = self.height();
        self.terminal = Some(ratatui::init_with_options(TerminalOptions {
            viewport: Viewport::Inline(self.height),
        }));

        let mut reader = crossterm::event::EventStream::new();

        while !self.exit && !matches!(self.state, AuthState::Done) {
            self.draw_ui()?;

            tokio::select! {
                crossterm_event = reader.next() => {
                    if let Some(Ok(event)) = crossterm_event {
                        self.handle_crossterm_event(event).await?;
                    }
                }

                _ = tokio::time::sleep(std::time::Duration::from_millis(100)), if matches!(&self.state, AuthState::EnvConfig(modal) if modal.is_fetching()) => {
                    if let AuthState::EnvConfig(ref mut modal_envs) = &mut self.state {
                        if let Some(Ok(models)) = modal_envs.poll_fetch() {
                            if let AuthState::EnvConfig(modal_envs) = std::mem::replace(&mut self.state, AuthState::Done) {
                                let (config, providers, provider, env_values) = modal_envs.extract_state();
                                let modal_model = ModalModel::new(models, config.clone(), providers, provider, env_values);
                                self.state = AuthState::ModelSelection(modal_model);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn handle_crossterm_event(&mut self, event: Event) -> io::Result<()> {
        match event {
            Event::Resize(..) => {
                if let Some(ref mut terminal) = self.terminal {
                    terminal.autoresize()?;
                }
            }
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event).await?;
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_key_event(&mut self, key_event: KeyEvent) -> io::Result<()> {
        // Global key handling
        if matches!(key_event.code, KeyCode::Char('c'))
            && key_event
                .modifiers
                .contains(crossterm::event::KeyModifiers::CONTROL)
        {
            self.exit = true;
            return Ok(());
        }

        // Handle Esc in the Start state as exit
        if matches!(key_event.code, KeyCode::Esc) && matches!(self.state, AuthState::Start(_)) {
            self.exit = true;
            return Ok(());
        }

        // Handle current state event
        match &mut self.state {
            AuthState::Start(ref mut modal_config) => {
                match modal_config.handle_event(key_event).await {
                    NavAction::Back | NavAction::Done => {
                        self.exit = true;
                    }
                    NavAction::Next => {
                        // Load fresh config and start provider selection
                        let config = WakeConfig::load().unwrap_or_default();
                        let providers = LlmClient::list_providers();
                        let modal_providers = ModalProviders::new(config, providers);
                        self.state = AuthState::SelectProvider(modal_providers);
                    }
                    NavAction::None => {}
                }
            }
            AuthState::SelectProvider(ref mut modal_providers) => {
                match modal_providers.handle_event(key_event).await {
                    NavAction::Back => {
                        let modal_config = ModalConfig::new();
                        self.state = AuthState::Start(modal_config);
                    }
                    NavAction::Next => {
                        if let AuthState::SelectProvider(modal_providers) =
                            std::mem::replace(&mut self.state, AuthState::Done)
                        {
                            let (config, providers, selected_provider) =
                                modal_providers.extract_state();
                            let modal_envs = ModalEnvs::new(config, providers, selected_provider);
                            self.state = AuthState::EnvConfig(modal_envs);
                        }
                    }
                    _ => {}
                }
            }
            AuthState::EnvConfig(ref mut modal_envs) => {
                match modal_envs.handle_event(key_event).await {
                    NavAction::Back => {
                        if let AuthState::EnvConfig(modal_envs) =
                            std::mem::replace(&mut self.state, AuthState::Done)
                        {
                            let (config, providers, _provider, _env_values) =
                                modal_envs.extract_state();
                            let modal_providers = ModalProviders::new(config, providers);
                            self.state = AuthState::SelectProvider(modal_providers);
                        }
                    }
                    _ => {}
                }
            }
            AuthState::ModelSelection(ref mut modal_model) => {
                match modal_model.handle_event(key_event).await {
                    NavAction::Back => {
                        if let AuthState::ModelSelection(modal_model) =
                            std::mem::replace(&mut self.state, AuthState::Done)
                        {
                            let modal_envs = ModalEnvs::new(
                                modal_model.config,
                                modal_model.providers,
                                modal_model.provider,
                            );
                            self.state = AuthState::EnvConfig(modal_envs);
                        }
                    }
                    NavAction::Done => {
                        if let AuthState::ModelSelection(modal_model) =
                            std::mem::replace(&mut self.state, AuthState::Done)
                        {
                            if let Err(e) = self
                                .add_provider_to_config(
                                    &modal_model.config,
                                    &modal_model.provider.name,
                                    &modal_model.env_values,
                                    &modal_model,
                                )
                                .await
                            {
                                eprintln!("Failed to add provider: {}", e);
                            }

                            let modal_config = ModalConfig::new();
                            self.state = AuthState::Start(modal_config);
                        }
                    }
                    _ => {}
                }
            }
            AuthState::Done => {}
        }

        Ok(())
    }

    async fn add_provider_to_config(
        &self,
        config: &WakeConfig,
        provider_name: &str,
        env_values: &HashMap<String, String>,
        modal_model: &ModalModel,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = config.clone();

        // Add the new provider and get its index
        let new_provider_index = config.add_provider(
            provider_name.to_string(),
            env_values.clone(),
            modal_model.selected_model(),
        );

        // Set the newly added provider as selected
        config.set_selected_provider(new_provider_index)?;

        // Set environment variables for immediate use
        config.set_env_vars();

        // Save the updated config
        config.save()?;

        Ok(())
    }

    fn height(&self) -> u16 {
        match &self.state {
            AuthState::Start(modal_config) => modal_config.height() as u16,
            AuthState::SelectProvider(modal_providers) => modal_providers.height() as u16,
            AuthState::EnvConfig(modal_envs) => modal_envs.height() as u16,
            AuthState::ModelSelection(modal_model) => modal_model.height() as u16,
            AuthState::Done => 5,
        }
    }

    fn draw_ui(&mut self) -> io::Result<()> {
        let height = self.height();
        if let Some(ref mut terminal) = self.terminal {
            if height != self.height {
                self.height = height;
                // terminal.set_viewport_height(height)?; // Method not available in current ratatui version
            }

            terminal.draw(|frame| {
                let area = frame.area();
                match &mut self.state {
                    AuthState::Start(modal_config) => modal_config.draw(frame, area),
                    AuthState::SelectProvider(modal_providers) => modal_providers.draw(frame, area),
                    AuthState::EnvConfig(modal_envs) => modal_envs.draw(frame, area),
                    AuthState::ModelSelection(modal_model) => modal_model.draw(frame, area),
                    AuthState::Done => {}
                }
            })?;
        }
        Ok(())
    }
}
