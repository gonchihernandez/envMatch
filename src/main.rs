mod commands;
mod config;
mod error;

use clap::{Parser, Subcommand};
use commands::EnvMatchCommands;
use error::EnvMatchError;

#[derive(Parser)]
#[command(name = "envMatch")]
#[command(about = "Environment Variable Manager - Match your environments like Rust matches patterns")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize envMatch in current directory
    Init,
    /// Set an environment variable
    Set {
        key: String,
        value: String,
        #[arg(short, long, default_value = "development")]
        env: String,
    },
    /// Get an environment variable
    Get {
        key: String,
        #[arg(short, long, default_value = "development")]
        env: String,
    },
    /// Remove an environment variable
    Unset {
        key: String,
        #[arg(short, long, default_value = "development")]
        env: String,
    },
    /// Switch to a different environment
    Switch {
        environment: String,
    },
    /// List all variables in current environment
    List {
        #[arg(short, long)]
        env: Option<String>,
    },
    /// Show current active environment
    Current,
    /// Validate environment setup
    Validate {
        #[arg(short, long)]
        required: Option<String>,
    },
    /// Show available environments
    Envs,
}

fn main() {
    let cli = Cli::parse();
    let commands = EnvMatchCommands::new();

    let result = match cli.command {
        Commands::Init => commands.init(),
        Commands::Set { key, value, env } => commands.set_variable(&key, &value, &env),
        Commands::Get { key, env } => commands.get_variable(&key, &env).map(|_| ()),
        Commands::Unset { key, env } => commands.unset_variable(&key, &env),
        Commands::Switch { environment } => commands.switch_environment(&environment),
        Commands::List { env } => commands.list_variables(env.as_deref()).map(|_| ()),
        Commands::Current => commands.show_current_environment().map(|_| ()),
        Commands::Validate { required } => commands.validate_environment(required.as_deref()),
        Commands::Envs => commands.list_environments().map(|_| ()),
    };

    if let Err(error) = result {
        handle_error(error);
    }
}

fn handle_error(error: EnvMatchError) {
    match &error {
        EnvMatchError::MissingRequiredVariables { env, variables } => {
            eprintln!("❌ Missing required variables in environment '{}':", env);
            for var in variables {
                eprintln!("  - {}", var);
            }
        }
        _ => {
            eprintln!("❌ {}", error);
        }
    }
    std::process::exit(1);
}