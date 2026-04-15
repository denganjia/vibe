use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellType {
    Bash,
    Pwsh,
    Cmd,
}

pub struct ShellAdapter {
    pub shell_type: ShellType,
}

impl ShellAdapter {
    pub fn detect() -> Self {
        #[cfg(windows)]
        {
            let comspec = std::env::var("COMSPEC").unwrap_or_default().to_lowercase();
            if comspec.contains("powershell") || comspec.contains("pwsh") {
                ShellAdapter { shell_type: ShellType::Pwsh }
            } else if comspec.contains("cmd.exe") {
                ShellAdapter { shell_type: ShellType::Cmd }
            } else {
                // Fallback to Pwsh as it's more modern on Windows
                ShellAdapter { shell_type: ShellType::Pwsh }
            }
        }

        #[cfg(not(windows))]
        {
            let shell = std::env::var("SHELL").unwrap_or_default().to_lowercase();
            if shell.contains("zsh") || shell.contains("bash") || shell.contains("sh") {
                ShellAdapter { shell_type: ShellType::Bash }
            } else {
                ShellAdapter { shell_type: ShellType::Bash }
            }
        }
    }

    pub fn build_env_command(&self, key: &str, value: &str) -> String {
        match self.shell_type {
            ShellType::Bash => format!("export {}='{}'", key, value.replace("'", "'\\''")),
            ShellType::Pwsh => format!("$env:{}='{}'", key, value.replace("'", "''")),
            ShellType::Cmd => format!("set {}={}", key, value),
        }
    }

    pub fn build_all_env_commands(&self, envs: &HashMap<String, String>) -> Vec<String> {
        envs.iter()
            .map(|(k, v)| self.build_env_command(k, v))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_adapter_env_bash() {
        let adapter = ShellAdapter { shell_type: ShellType::Bash };
        assert_eq!(adapter.build_env_command("KEY", "VALUE"), "export KEY='VALUE'");
        assert_eq!(adapter.build_env_command("KEY", "O'Reilly"), "export KEY='O'\\''Reilly'");
    }

    #[test]
    fn test_shell_adapter_env_pwsh() {
        let adapter = ShellAdapter { shell_type: ShellType::Pwsh };
        assert_eq!(adapter.build_env_command("KEY", "VALUE"), "$env:KEY='VALUE'");
        assert_eq!(adapter.build_env_command("KEY", "O'Reilly"), "$env:KEY='O''Reilly'");
    }

    #[test]
    fn test_shell_adapter_env_cmd() {
        let adapter = ShellAdapter { shell_type: ShellType::Cmd };
        assert_eq!(adapter.build_env_command("KEY", "VALUE"), "set KEY=VALUE");
    }
}
