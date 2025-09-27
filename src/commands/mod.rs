use crate::config::ConfigManager;
use crate::error::{EnvMatchError, Result};
use colored::*;

#[derive(Debug)]
pub struct EnvMatchCommands {
    config_manager: ConfigManager,
}

impl EnvMatchCommands {
    pub fn new() -> Self {
        Self {
            config_manager: ConfigManager::new(),
        }
    }

    #[cfg(test)]
    pub fn with_config_manager(config_manager: ConfigManager) -> Self {
        Self { config_manager }
    }

    pub fn init_with_environment(&self, env_name: &str) -> Result<()> {
        self.config_manager.initialize()?;

        println!(
            "{}",
            "✅ envMatch initialized successfully!"
                .bright_green()
                .bold()
        );
        println!("{}", "📁 Created .envMatch directory".bright_blue());
        println!(
            "{} {}",
            "🔧 Initial environment:".bright_yellow(),
            env_name.bright_green().bold()
        );

        // Create the initial environment if it's not the default "development"
        if env_name != "development" {
            let env_config = crate::config::EnvConfig::default();
            self.config_manager
                .save_environment(env_name, &env_config)?;
            println!(
                "{} {}",
                "🆕 Created environment:".bright_cyan(),
                env_name.bright_green().bold()
            );
        }

        println!(
            "{} {}",
            "💡 Try:".bright_magenta(),
            format!("envMatch set API_KEY your-key-here -e {}", env_name).bright_cyan()
        );

        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.config_manager.is_initialized()
    }

    pub fn set_variable(&self, key: &str, value: &str, env_name: &str) -> Result<()> {
        let mut env_config = self.config_manager.load_environment(env_name)?;
        env_config
            .variables
            .insert(key.to_string(), value.to_string());
        self.config_manager
            .save_environment(env_name, &env_config)?;

        println!(
            "{} {}={} {} {}",
            "✅ Set".bright_green().bold(),
            key.bright_cyan().bold(),
            value.bright_yellow(),
            "in environment".bright_white(),
            format!("'{}'", env_name).bright_green().bold()
        );
        Ok(())
    }

    pub fn get_variable(&self, key: &str, env_name: &str) -> Result<String> {
        let env_config = self.config_manager.load_environment(env_name)?;

        match env_config.variables.get(key) {
            Some(value) => {
                println!("{}", value);
                Ok(value.clone())
            }
            None => Err(EnvMatchError::VariableNotFound {
                key: key.to_string(),
                env: env_name.to_string(),
            }),
        }
    }

    pub fn unset_variable(&self, key: &str, env_name: &str) -> Result<()> {
        let mut env_config = self.config_manager.load_environment(env_name)?;

        if env_config.variables.remove(key).is_some() {
            self.config_manager
                .save_environment(env_name, &env_config)?;
            println!(
                "{} {} {} {}",
                "✅ Removed".bright_green().bold(),
                format!("'{}'", key).bright_red().bold(),
                "from environment".bright_white(),
                format!("'{}'", env_name).bright_green().bold()
            );
            Ok(())
        } else {
            Err(EnvMatchError::VariableNotFound {
                key: key.to_string(),
                env: env_name.to_string(),
            })
        }
    }

    pub fn switch_environment(&self, env_name: &str) -> Result<()> {
        // Ensure the environment exists by loading it
        self.config_manager.load_environment(env_name)?;

        let mut config = self.config_manager.load_global_config()?;
        config.current_environment = env_name.to_string();
        self.config_manager.save_global_config(&config)?;

        println!(
            "{} {}",
            "✅ Switched to environment".bright_green().bold(),
            format!("'{}'", env_name).bright_green().bold().underline()
        );
        Ok(())
    }

    pub fn list_variables(&self, env_name: Option<&str>) -> Result<Vec<(String, String)>> {
        let config = self.config_manager.load_global_config()?;
        let env_name = env_name.unwrap_or(&config.current_environment);
        let env_config = self.config_manager.load_environment(env_name)?;

        println!(
            "{} {}",
            "📋 Environment:".bright_blue().bold(),
            env_name.bright_green().bold()
        );
        println!("{}", "─".repeat(40).bright_blue());

        if env_config.variables.is_empty() {
            println!("{}", "(no variables set)".bright_black());
            return Ok(vec![]);
        }

        let mut vars: Vec<_> = env_config.variables.iter().collect();
        vars.sort_by_key(|(k, _)| *k);

        let result: Vec<(String, String)> = vars
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        for (key, value) in &vars {
            println!("{}={}", key.bright_cyan().bold(), value.bright_green());
        }

        Ok(result)
    }

    pub fn show_current_environment(&self) -> Result<String> {
        let config = self.config_manager.load_global_config()?;
        println!("{}", config.current_environment);
        Ok(config.current_environment)
    }

    pub fn validate_environment(&self, required: Option<&str>) -> Result<()> {
        let config = self.config_manager.load_global_config()?;
        let env_config = self
            .config_manager
            .load_environment(&config.current_environment)?;

        if let Some(required_vars) = required {
            let required_list: Vec<&str> = required_vars.split(',').map(|s| s.trim()).collect();
            let missing: Vec<String> = required_list
                .iter()
                .filter(|&&var| !env_config.variables.contains_key(var))
                .map(|&var| var.to_string())
                .collect();

            if missing.is_empty() {
                println!(
                    "✅ All required variables are set in environment '{}'",
                    config.current_environment
                );
                Ok(())
            } else {
                Err(EnvMatchError::MissingRequiredVariables {
                    env: config.current_environment,
                    variables: missing,
                })
            }
        } else {
            let var_count = env_config.variables.len();
            println!(
                "✅ Environment '{}' has {} variable(s)",
                config.current_environment, var_count
            );
            Ok(())
        }
    }

    pub fn list_environments(&self) -> Result<Vec<String>> {
        let environments = self.config_manager.list_environments()?;
        let config = self.config_manager.load_global_config()?;

        if environments.is_empty() {
            println!("No environments found.");
            return Ok(vec![]);
        }

        println!("📁 Available environments:");
        println!("{}", "─".repeat(30));

        for env in &environments {
            if env == &config.current_environment {
                println!("• {} (current)", env);
            } else {
                println!("• {}", env);
            }
        }

        Ok(environments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ConfigManager;
    use tempfile::TempDir;

    fn create_test_commands() -> (EnvMatchCommands, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let base_dir = temp_dir.path().join(".envMatch");
        let config_manager = ConfigManager::with_base_dir(base_dir);
        let commands = EnvMatchCommands::with_config_manager(config_manager);
        (commands, temp_dir)
    }

    #[test]
    fn test_init_command() {
        let (commands, _temp_dir) = create_test_commands();

        commands.init_with_environment("development").unwrap();

        // Should fail to init again
        assert!(matches!(
            commands.init_with_environment("development"),
            Err(EnvMatchError::AlreadyInitialized)
        ));
    }

    #[test]
    fn test_set_and_get_variable() {
        let (commands, _temp_dir) = create_test_commands();
        commands.init_with_environment("development").unwrap();

        commands
            .set_variable("TEST_KEY", "test_value", "development")
            .unwrap();
        let value = commands.get_variable("TEST_KEY", "development").unwrap();

        assert_eq!(value, "test_value");
    }

    #[test]
    fn test_unset_variable() {
        let (commands, _temp_dir) = create_test_commands();
        commands.init_with_environment("development").unwrap();

        commands
            .set_variable("TEST_KEY", "test_value", "development")
            .unwrap();
        commands.unset_variable("TEST_KEY", "development").unwrap();

        let result = commands.get_variable("TEST_KEY", "development");
        assert!(matches!(
            result,
            Err(EnvMatchError::VariableNotFound { .. })
        ));
    }

    #[test]
    fn test_switch_environment() {
        let (commands, _temp_dir) = create_test_commands();
        commands.init_with_environment("development").unwrap();

        commands.switch_environment("production").unwrap();
        let current = commands.show_current_environment().unwrap();

        assert_eq!(current, "production");
    }

    #[test]
    fn test_list_variables() {
        let (commands, _temp_dir) = create_test_commands();
        commands.init_with_environment("development").unwrap();

        commands
            .set_variable("KEY1", "value1", "development")
            .unwrap();
        commands
            .set_variable("KEY2", "value2", "development")
            .unwrap();

        let variables = commands.list_variables(None).unwrap();

        assert_eq!(variables.len(), 2);
        assert!(variables.contains(&("KEY1".to_string(), "value1".to_string())));
        assert!(variables.contains(&("KEY2".to_string(), "value2".to_string())));
    }

    #[test]
    fn test_validate_environment() {
        let (commands, _temp_dir) = create_test_commands();
        commands.init_with_environment("development").unwrap();

        commands
            .set_variable("REQUIRED_VAR", "value", "development")
            .unwrap();

        // Should pass validation
        commands.validate_environment(Some("REQUIRED_VAR")).unwrap();

        // Should fail validation for missing variable
        let result = commands.validate_environment(Some("MISSING_VAR"));
        assert!(matches!(
            result,
            Err(EnvMatchError::MissingRequiredVariables { .. })
        ));
    }

    #[test]
    fn test_list_environments() {
        let (commands, _temp_dir) = create_test_commands();
        commands.init_with_environment("development").unwrap();

        commands
            .set_variable("TEST", "value", "production")
            .unwrap();

        let environments = commands.list_environments().unwrap();

        assert!(environments.contains(&"development".to_string()));
        assert!(environments.contains(&"production".to_string()));
    }
}
