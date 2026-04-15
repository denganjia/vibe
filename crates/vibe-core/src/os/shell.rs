use std::collections::HashMap;
use regex::Regex;

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

    pub fn build_cd_command(&self, path: &str) -> String {
        match self.shell_type {
            ShellType::Bash => format!("cd '{}'", path.replace("'", "'\\''")),
            ShellType::Pwsh => format!("Set-Location '{}'", path.replace("'", "''")),
            ShellType::Cmd => format!("cd /d \"{}\"", path),
        }
    }

    pub fn build_full_command(&self, cmd: &str, cwd: Option<&str>, envs: &HashMap<String, String>) -> String {
        let mut parts = Vec::new();
        
        for (k, v) in envs {
            parts.push(self.build_env_command(k, v));
        }
        
        if let Some(path) = cwd {
            parts.push(self.build_cd_command(path));
        }
        
        parts.push(cmd.to_string());
        
        match self.shell_type {
            ShellType::Bash => parts.join(" && "),
            ShellType::Pwsh => parts.join("; "),
            ShellType::Cmd => parts.join(" && "),
        }
    }
}

pub fn strip_ansi(input: &str) -> String {
    let re = Regex::new(r"[\u001b\u009b][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]").unwrap();
    re.replace_all(input, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ansi_stripping() {
        let input = "\x1b[31mHello\x1b[0m World";
        assert_eq!(strip_ansi(input), "Hello World");

        let input2 = "\x1b[1;32mGreen Bold\x1b[0m";
        assert_eq!(strip_ansi(input2), "Green Bold");
    }

    #[test]
    fn test_shell_adapter_env_bash() {
        let adapter = ShellAdapter { shell_type: ShellType::Bash };
        assert_eq!(adapter.build_env_command("KEY", "VALUE"), "export KEY='VALUE'");
        assert_eq!(adapter.build_env_command("KEY", "O'Reilly"), "export KEY='O'\\''Reilly'");
    }

    #[test]
    fn test_shell_adapter_full_bash() {
        let adapter = ShellAdapter { shell_type: ShellType::Bash };
        let mut envs = HashMap::new();
        envs.insert("A".to_string(), "1".to_string());
        let full = adapter.build_full_command("ls", Some("/tmp"), &envs);
        assert!(full.contains("export A='1'"));
        assert!(full.contains("cd '/tmp'"));
        assert!(full.ends_with("ls"));
    }
}
