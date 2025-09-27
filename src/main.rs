mod commands;
mod config;
mod error;
mod tui;

use clap::{Parser, Subcommand};
use commands::EnvMatchCommands;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use error::EnvMatchError;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;
use tui::{App, EventHandler};

#[derive(Parser)]
#[command(name = "envMatch")]
#[command(
    about = "Environment Variable Manager - Match your environments like Rust matches patterns"
)]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize envMatch in current directory
    Init {
        /// Initial environment to create (default: development)
        #[arg(default_value = "development")]
        environment: String,
    },
    /// Launch interactive TUI (default mode)
    Tui,
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
    Switch { environment: String },
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let commands = EnvMatchCommands::new();

    // If no command is specified, check if initialized and launch TUI
    let command = cli.command.unwrap_or_else(|| {
        if commands.is_initialized() {
            Commands::Tui
        } else {
            Commands::Init {
                environment: "development".to_string(),
            }
        }
    });

    let result = match command {
        Commands::Init { environment } => commands.init_with_environment(&environment),
        Commands::Tui => run_tui().await,
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

    Ok(())
}

async fn run_tui() -> Result<(), EnvMatchError> {
    // Setup terminal
    enable_raw_mode().map_err(|e| EnvMatchError::ConfigReadError { source: e })?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .map_err(|e| EnvMatchError::ConfigReadError { source: e })?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal =
        Terminal::new(backend).map_err(|e| EnvMatchError::ConfigReadError { source: e })?;

    // Create app and event handler
    let mut app = App::new()?;
    let mut event_handler = EventHandler::new(Duration::from_millis(100));

    // Run the main loop
    while !app.should_quit {
        terminal
            .draw(|f| tui::ui::draw(f, &mut app))
            .map_err(|e| EnvMatchError::ConfigReadError { source: e })?;

        if let Some(event) = event_handler.next().await {
            match event {
                tui::Event::Key(key_event) => {
                    if let Err(e) = app.handle_key(key_event.code) {
                        app.error_message = e.to_string();
                    }
                }
                tui::Event::Tick => {
                    // Clear old messages after some time
                    if !app.status_message.is_empty() || !app.error_message.is_empty() {
                        // You could add a timer here to clear messages after a delay
                    }
                }
                tui::Event::Resize => {
                    // Terminal was resized, redraw on next iteration
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode().map_err(|e| EnvMatchError::ConfigReadError { source: e })?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .map_err(|e| EnvMatchError::ConfigReadError { source: e })?;
    terminal
        .show_cursor()
        .map_err(|e| EnvMatchError::ConfigReadError { source: e })?;

    Ok(())
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
