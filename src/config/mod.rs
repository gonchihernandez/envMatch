use crate::error::{EnvMatchError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const ENV_MATCH_DIR: &str = ".envMatch";
const CONFIG_FILE: &str = "config.yaml";
const ENVIRONMENTS_DIR: &str = "environments";
const DEFAULT_ENVIRONMENT: &str = "development";

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct EnvConfig {
    pub variables: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct GlobalConfig {
    pub current_environment: String,
}

pub struct ConfigManager {
    base_dir: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Self {
        let base_dir = std::env::current_dir()
            .expect("Failed to get current directory")
            .join(ENV_MATCH_DIR);

        Self { base_dir }
    }

    #[cfg(test)]
    pub fn with_base_dir(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    pub fn is_initialized(&self) -> bool {
        self.base_dir.exists() && self.get_config_path().exists()
    }

    pub fn initialize(&self) -> Result<()> {
        if self.is_initialized() {
            return Err(EnvMatchError::AlreadyInitialized);
        }

        // Create directories
        fs::create_dir_all(self.get_environments_dir())?;

        // Create default config
        let config = GlobalConfig {
            current_environment: DEFAULT_ENVIRONMENT.to_string(),
        };
        self.save_global_config(&config)?;

        // Create default development environment
        let dev_env = EnvConfig::default();
        self.save_environment(DEFAULT_ENVIRONMENT, &dev_env)?;

        Ok(())
    }

    pub fn load_global_config(&self) -> Result<GlobalConfig> {
        if !self.is_initialized() {
            return Err(EnvMatchError::NotInitialized);
        }

        let content = fs::read_to_string(self.get_config_path())?;
        let config = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn save_global_config(&self, config: &GlobalConfig) -> Result<()> {
        let config_yaml = serde_yaml::to_string(config)?;
        fs::write(self.get_config_path(), config_yaml)?;
        Ok(())
    }

    pub fn load_environment(&self, env_name: &str) -> Result<EnvConfig> {
        if !self.is_initialized() {
            return Err(EnvMatchError::NotInitialized);
        }

        let env_path = self.get_env_path(env_name);

        if !env_path.exists() {
            // Create new environment if it doesn't exist
            let new_env = EnvConfig::default();
            self.save_environment(env_name, &new_env)?;
            return Ok(new_env);
        }

        let content = fs::read_to_string(env_path)?;
        let env_config = serde_yaml::from_str(&content).unwrap_or_default();
        Ok(env_config)
    }

    pub fn save_environment(&self, env_name: &str, env_config: &EnvConfig) -> Result<()> {
        self.validate_environment_name(env_name)?;

        let env_yaml = serde_yaml::to_string(env_config)?;

        // Ensure environments directory exists
        fs::create_dir_all(self.get_environments_dir())?;

        fs::write(self.get_env_path(env_name), env_yaml)?;
        Ok(())
    }

    pub fn list_environments(&self) -> Result<Vec<String>> {
        let env_dir = self.get_environments_dir();

        if !env_dir.exists() {
            return Ok(vec![]);
        }

        let mut environments = Vec::new();

        for entry in fs::read_dir(&env_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "yaml") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    environments.push(name.to_string());
                }
            }
        }

        environments.sort();
        Ok(environments)
    }

    fn get_config_path(&self) -> PathBuf {
        self.base_dir.join(CONFIG_FILE)
    }

    fn get_environments_dir(&self) -> PathBuf {
        self.base_dir.join(ENVIRONMENTS_DIR)
    }

    fn get_env_path(&self, env_name: &str) -> PathBuf {
        self.get_environments_dir()
            .join(format!("{}.yaml", env_name))
    }

    fn validate_environment_name(&self, name: &str) -> Result<()> {
        if name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            Ok(())
        } else {
            Err(EnvMatchError::InvalidEnvironmentName {
                name: name.to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_config_manager() -> (ConfigManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let base_dir = temp_dir.path().join(ENV_MATCH_DIR);
        let config_manager = ConfigManager::with_base_dir(base_dir);
        (config_manager, temp_dir)
    }

    #[test]
    fn test_initialization() {
        let (config_manager, _temp_dir) = create_test_config_manager();

        assert!(!config_manager.is_initialized());

        config_manager.initialize().unwrap();

        assert!(config_manager.is_initialized());

        // Should fail to initialize again
        assert!(matches!(
            config_manager.initialize(),
            Err(EnvMatchError::AlreadyInitialized)
        ));
    }

    #[test]
    fn test_global_config() {
        let (config_manager, _temp_dir) = create_test_config_manager();
        config_manager.initialize().unwrap();

        let config = config_manager.load_global_config().unwrap();
        assert_eq!(config.current_environment, "development");

        let new_config = GlobalConfig {
            current_environment: "production".to_string(),
        };
        config_manager.save_global_config(&new_config).unwrap();

        let loaded_config = config_manager.load_global_config().unwrap();
        assert_eq!(loaded_config.current_environment, "production");
    }

    #[test]
    fn test_environment_management() {
        let (config_manager, _temp_dir) = create_test_config_manager();
        config_manager.initialize().unwrap();

        let mut env_config = EnvConfig::default();
        env_config
            .variables
            .insert("TEST_VAR".to_string(), "test_value".to_string());

        config_manager
            .save_environment("test", &env_config)
            .unwrap();

        let loaded_env = config_manager.load_environment("test").unwrap();
        assert_eq!(
            loaded_env.variables.get("TEST_VAR"),
            Some(&"test_value".to_string())
        );

        let environments = config_manager.list_environments().unwrap();
        assert!(environments.contains(&"test".to_string()));
        assert!(environments.contains(&"development".to_string()));
    }

    #[test]
    fn test_invalid_environment_name() {
        let (config_manager, _temp_dir) = create_test_config_manager();
        config_manager.initialize().unwrap();

        let env_config = EnvConfig::default();
        let result = config_manager.save_environment("invalid name!", &env_config);

        assert!(matches!(
            result,
            Err(EnvMatchError::InvalidEnvironmentName { .. })
        ));
    }

    #[test]
    fn test_not_initialized_error() {
        let (config_manager, _temp_dir) = create_test_config_manager();

        let result = config_manager.load_global_config();
        assert!(matches!(result, Err(EnvMatchError::NotInitialized)));
    }
}
