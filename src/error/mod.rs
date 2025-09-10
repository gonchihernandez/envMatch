use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvMatchError {
    #[error("envMatch not initialized. Run 'envMatch init' first")]
    NotInitialized,

    #[error("envMatch already initialized in this directory")]
    AlreadyInitialized,

    #[error("Variable '{key}' not found in environment '{env}'")]
    VariableNotFound { key: String, env: String },

    #[error("Failed to read config file: {source}")]
    ConfigReadError {
        #[from]
        source: std::io::Error,
    },

    #[error("Failed to parse YAML: {source}")]
    YamlParseError {
        #[from]
        source: serde_yaml::Error,
    },

    #[error("Missing required variables in environment '{env}': {variables:?}")]
    MissingRequiredVariables { env: String, variables: Vec<String> },

    #[error("Invalid environment name: '{name}'. Environment names must be alphanumeric")]
    InvalidEnvironmentName { name: String },
}

pub type Result<T> = std::result::Result<T, EnvMatchError>;
