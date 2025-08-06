use std::env;
use clap::ValueEnum;
use std::process::Command;

pub static MAGIC_COOKIE: &str = ">>>WAKE_HOOKS_INJECTED<<<";

#[derive(Debug, Clone, ValueEnum)]
pub enum ShellType {
    #[value(name = "sh")]
    Sh,
    #[value(name = "bash")]
    Bash,
    #[value(name = "zsh")]
    Zsh,
    #[value(name = "fish")]
    Fish,
    #[value(name = "powershell", alias = "pwsh")]
    Powershell,
}

#[derive(Debug, Clone)]
pub struct Shell {
    pub shell_type: ShellType,
    pub path: String,
}

impl Shell {
    pub fn new(shell_type: ShellType, path: String) -> Self {
        Self { shell_type, path }
    }


    pub fn generate_rc_content(&self) -> String {
        let wake_binary = get_wake_binary_path();
    
        match self.shell_type {
            ShellType::Sh | ShellType::Bash => {
                format!(r#"# Wake hook for POSIX sh
# Shell path: {}
# Pre-command hook (captures command before execution)
wake_precmd() {{
    case "$BASH_COMMAND" in
        wake*|*wake_precmd*|*wake_postcmd*) return ;;
    esac
    WAKE_CURRENT_CMD="$BASH_COMMAND"
    "{}" precmd "$WAKE_CURRENT_CMD"
}}

# Post-command hook (captures exit status after execution)
wake_postcmd() {{
    exit_code="$?"
    if [ -n "$WAKE_CURRENT_CMD" ]; then
        "{}" postcmd "$exit_code" "$WAKE_CURRENT_CMD"
        unset WAKE_CURRENT_CMD
    fi
    # Reload history to pick up any new entries
    history -r 2>/dev/null || true
}}

# Set up the prompt command to run the hook
export PROMPT_COMMAND="${{PROMPT_COMMAND:+$PROMPT_COMMAND; }}wake_postcmd"

echo "{}"
# Capture command in DEBUG trap (before execution)
trap 'wake_precmd' DEBUG
    "#, self.path, wake_binary, wake_binary, MAGIC_COOKIE)
            }

            ShellType::Zsh => {
                format!(r#"# Wake hook for zsh
# Shell path: {}
# Capture command before execution
wake_preexec_hook() {{
    WAKE_CURRENT_CMD="$1"
    "{}" precmd "$1"
}}

# Pre-command hook (captures after execution)
wake_precmd_hook() {{
    exit_code=$?
    if [ -n "$WAKE_CURRENT_CMD" ]; then
        case "$WAKE_CURRENT_CMD" in
            wake*|*wake_preexec_hook*|*wake_precmd_hook*) ;;
            *) "{}" postcmd "$exit_code" "$WAKE_CURRENT_CMD" ;;
        esac
    fi
    unset WAKE_CURRENT_CMD
    # Reload history to pick up any new entries
    fc -R 2>/dev/null || true
}}

# Hook into zsh's command execution cycle
autoload -Uz add-zsh-hook
add-zsh-hook preexec wake_preexec_hook
add-zsh-hook precmd  wake_precmd_hook
echo "{}"
    "#, self.path, wake_binary, wake_binary, MAGIC_COOKIE)
                }
    
            ShellType::Fish => {
                format!(r#"# Wake hook for fish
# Shell path: {}
# Pre-command hook (captures command before execution)
function wake_precmd --on-event fish_preexec
    set cmd $argv[1]
    # Skip wake-related commands
    if string match -q "wake*" $cmd; or string match -q "*wake_precmd*" $cmd; or string match -q "*wake_postcmd*" $cmd
        return
    end
    set -g WAKE_CURRENT_CMD $cmd
    "{}" precmd "$WAKE_CURRENT_CMD"
end

# Post-command hook (captures exit status after execution)
function wake_postcmd --on-event fish_postexec
    set exit_code $status
    if set -q WAKE_CURRENT_CMD
        "{}" postcmd $exit_code "$WAKE_CURRENT_CMD"
        set -e WAKE_CURRENT_CMD
    end
    # Reload history to pick up any new entries
    history --merge 2>/dev/null; or true
end

echo "{}"
    "#, self.path, wake_binary, wake_binary, MAGIC_COOKIE)
                }
    
            ShellType::Powershell => {
                format!(r#"# Wake hook for PowerShell
# Shell path: {}
# Initialize command variable
$global:WAKE_CURRENT_CMD = $null

# Function to check if command should be filtered
function Test-WakeCommand {{
    param([string]$Command)
    if ([string]::IsNullOrEmpty($Command)) {{ return $true }}
    
    $patterns = @("wake*", "*wake_precmd*", "*wake_postcmd*", "*Invoke-Wake*", "*Set-WakeCommand*", "*Test-WakeCommand*")
    foreach ($pattern in $patterns) {{
        if ($Command -like $pattern) {{ return $true }}
    }}
    return $false
}}

# Pre-command hook
function Invoke-WakePrecmd {{
    if ($global:WAKE_CURRENT_CMD -and -not (Test-WakeCommand $global:WAKE_CURRENT_CMD)) {{
        "{}" precmd $global:WAKE_CURRENT_CMD
    }}
}}

# Post-command hook
function Invoke-WakePostcmd {{
    if ($global:WAKE_CURRENT_CMD -and -not (Test-WakeCommand $global:WAKE_CURRENT_CMD)) {{
        "{}" postcmd $LASTEXITCODE $global:WAKE_CURRENT_CMD
    }}
    $global:WAKE_CURRENT_CMD = $null
}}

# Try to use PSReadLine for automatic command capture if available
if (Get-Module -ListAvailable -Name PSReadLine) {{
    try {{
        Import-Module PSReadLine -ErrorAction Stop
        
        # Set up PSReadLine to capture commands
        Set-PSReadLineOption -AddToHistoryHandler {{
            param($command)
            if (-not (Test-WakeCommand $command)) {{
                $global:WAKE_CURRENT_CMD = $command
            }}
            return $true
        }}
        
        Write-Host "Wake: Using PSReadLine for automatic command capture" -ForegroundColor Green
    }} catch {{
        Write-Host "Wake: PSReadLine available but failed to configure. Using manual mode." -ForegroundColor Yellow
    }}
}} else {{
    Write-Host "Wake: PSReadLine not available. Commands must be set manually using Set-WakeCommand." -ForegroundColor Yellow
}}

# Manual command setting function for when PSReadLine is not available
function Set-WakeCommand {{
    param([string]$Command)
    if (-not (Test-WakeCommand $Command)) {{
        $global:WAKE_CURRENT_CMD = $Command
    }}
}}

# Override the prompt function to include our hooks
function prompt {{
    # Run pre-command hook first
    Invoke-WakePrecmd
    
    # Run post-command hook
    Invoke-WakePostcmd
    
    # Return the prompt string (customize as needed)
    "PS $($executionContext.SessionState.Path.CurrentLocation)$('>' * ($nestedPromptLevel + 1)) "
}}

# Usage examples:
# With PSReadLine (automatic): Just run commands normally
# Without PSReadLine (manual): Set-WakeCommand "your-command"; your-command
    "#, self.path, wake_binary, wake_binary)
            }
        }
    }

}

fn get_wake_binary_path() -> String {
    match env::current_exe() {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(_) => {
            // Fallback strategies if current_exe() fails
            if let Some(path) = find_bin_path("wake") {
                path
            } else {
                "wake".to_string() // Last resort
            }
        }
    }
}

pub fn get_shell(shell_type: Option<ShellType>) -> Result<Shell, Box<dyn std::error::Error>> {
    return match shell_type {
        Some(s) => find_shell_by_type(s),
        None => detect_shell(),
    };
}

pub fn find_shell_by_type(shell_type: ShellType) -> Result<Shell, Box<dyn std::error::Error>> {
    let shell_name = match shell_type {
        ShellType::Sh => "sh",
        ShellType::Bash => "bash",
        ShellType::Zsh => "zsh",
        ShellType::Fish => "fish",
        ShellType::Powershell => "pwsh"
    };

    if let Some(path) = find_bin_path(shell_name) {
        Ok(Shell::new(shell_type, path))
    } else {
        Err(format!("Shell '{}' not found in PATH", shell_name).into())
    }
}

pub fn detect_shell() -> Result<Shell, Box<dyn std::error::Error>> {
    // Try SHELL environment variable first
    if let Ok(shell_path) = env::var("SHELL") {
        let shell_name = shell_path
            .split('/')
            .last()
            .unwrap_or("")
            .to_lowercase();
        
        let detected_shell_type = match shell_name.as_str() {
            "sh" => Some(ShellType::Sh),
            "bash" => Some(ShellType::Bash),
            "zsh" => Some(ShellType::Zsh),
            "fish" => Some(ShellType::Fish),
            "pwsh" | "powershell" => Some(ShellType::Powershell),
            _ => None,
        };

        if let Some(shell_type) = detected_shell_type {
            if std::path::Path::new(&shell_path).exists() {
                return Ok(Shell::new(shell_type, shell_path));
            }
        }
    }

    // Fallback: try to detect from environment variables and find their paths
    if env::var("ZSH_VERSION").is_ok() {
        if let Some(path) = find_bin_path("zsh") {
            return Ok(Shell::new(ShellType::Zsh, path));
        }
    }
    if env::var("BASH_VERSION").is_ok() {
        if let Some(path) = find_bin_path("bash") {
            return Ok(Shell::new(ShellType::Bash, path));
        }
    }
    if env::var("FISH_VERSION").is_ok() {
        if let Some(path) = find_bin_path("fish") {
            return Ok(Shell::new(ShellType::Fish, path));
        }
    }

    // Final fallback: check if sh exists
    if let Some(path) = find_bin_path("sh") {
        return Ok(Shell::new(ShellType::Sh, path));
    }

    // No shell found
    Err("Could not detect shell and no compatible shell is available. Please specify shell explicitly.".into())
}

fn find_bin_path(shell_name: &str) -> Option<String> {
    // First try using 'which' command
    if let Ok(output) = Command::new("which")
        .arg(shell_name)
        .output()
    {
        if output.status.success() {
            if let Ok(path) = String::from_utf8(output.stdout) {
                return Some(path.trim().to_string());
            }
        }
    }

    // Fallback: manually search PATH
    if let Ok(path_var) = env::var("PATH") {
        let path_separator = if cfg!(windows) { ';' } else { ':' };
        
        for path_dir in path_var.split(path_separator) {
            let shell_path = if cfg!(windows) {
                // On Windows, check for .exe extension
                let with_exe = std::path::Path::new(path_dir).join(format!("{}.exe", shell_name));
                let without_exe = std::path::Path::new(path_dir).join(shell_name);
                if with_exe.exists() && with_exe.is_file() {
                    with_exe
                } else if without_exe.exists() && without_exe.is_file() {
                    without_exe
                } else {
                    continue;
                }
            } else {
                let path = std::path::Path::new(path_dir).join(shell_name);
                if path.exists() && path.is_file() {
                    path
                } else {
                    continue;
                }
            };
            
            if let Some(path_str) = shell_path.to_str() {
                return Some(path_str.to_string());
            }
        }
    }
    
    // Also check common shell locations on Unix-like systems
    if !cfg!(windows) {
        let common_locations = [
            format!("/bin/{}", shell_name),
            format!("/usr/bin/{}", shell_name),
            format!("/usr/local/bin/{}", shell_name),
        ];
        
        for location in &common_locations {
            if std::path::Path::new(location).exists() {
                return Some(location.clone());
            }
        }
    }
    
    None
}

pub fn write_to_shell_history(command: &str) {
    use std::env;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::process::Command;
    
    // Try to detect the shell and write to its history file
    if let Ok(shell) = env::var("SHELL") {
        let shell_type = if shell.contains("zsh") {
            ShellType::Zsh
        } else if shell.contains("bash") {
            ShellType::Bash
        } else if shell.contains("fish") {
            ShellType::Fish
        } else if shell.contains("sh") {
            ShellType::Sh
        } else {
            return; // Unknown shell
        };
        
        let history_file = get_history_file_path(&shell_type);
        
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&history_file) {
            let formatted_command = format_command_for_shell(&shell_type, command);
            if let Err(e) = file.write_all(formatted_command.as_bytes()) {
                //eprintln!("Failed to write to history: {}", e);
                return;
            }
            if let Err(e) = file.flush() {
                //eprintln!("Failed to flush history: {}", e);
                return;
            }
            //eprintln!("Added to history: {}", command.trim());
        }
    }
}

fn get_history_file_path(shell_type: &ShellType) -> String {
    use std::env;
    
    match shell_type {
        ShellType::Zsh => {
            env::var("HISTFILE").unwrap_or_else(|_| 
                format!("{}/.zsh_history", env::var("HOME").unwrap_or_default())
            )
        }
        ShellType::Bash | ShellType::Sh => {
            env::var("HISTFILE").unwrap_or_else(|_| 
                format!("{}/.bash_history", env::var("HOME").unwrap_or_default())
            )
        }
        ShellType::Fish => {
            format!("{}/.local/share/fish/fish_history", env::var("HOME").unwrap_or_default())
        }
        ShellType::Powershell => {
            // PowerShell history is more complex, skip for now
            String::new()
        }
    }
}

fn format_command_for_shell(shell_type: &ShellType, command: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    match shell_type {
        ShellType::Zsh => {
            // ZSH extended history format: : timestamp:duration;command
            format!(": {}:0;{}\n", timestamp, command.trim())
        }
        ShellType::Fish => {
            // Fish YAML-like format
            format!("- cmd: {}\n  when: {}\n", command.trim(), timestamp)
        }
        ShellType::Bash | ShellType::Sh => {
            // Simple format: just the command
            format!("{}\n", command.trim())
        }
        ShellType::Powershell => {
            // PowerShell history is complex, return empty for now
            String::new()
        }
    }
}