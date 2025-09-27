use crate::commands::EnvMatchCommands;
use crate::error::{EnvMatchError, Result};
use crossterm::event::KeyCode;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum AppState {
    #[default]
    EnvironmentList,
    VariableList,
    AddVariable,
    EditVariable,
    ConfirmDelete,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub struct App {
    pub state: AppState,
    pub commands: EnvMatchCommands,
    pub current_environment: String,
    pub environments: Vec<String>,
    pub variables: Vec<Variable>,
    pub selected_env_index: usize,
    pub selected_var_index: usize,
    pub input_buffer: String,
    pub input_key: String,
    pub should_quit: bool,
    pub status_message: String,
    pub error_message: String,
    pub show_help: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let commands = EnvMatchCommands::new();

        // Check if initialized
        if !commands.is_initialized() {
            return Err(EnvMatchError::NotInitialized);
        }

        let current_environment = commands.show_current_environment()?;
        let environments = commands.list_environments()?;
        let variables = Self::load_variables(&commands, &current_environment)?;

        let selected_env_index = environments
            .iter()
            .position(|env| env == &current_environment)
            .unwrap_or(0);

        Ok(Self {
            state: AppState::EnvironmentList,
            commands,
            current_environment,
            environments,
            variables,
            selected_env_index,
            selected_var_index: 0,
            input_buffer: String::new(),
            input_key: String::new(),
            should_quit: false,
            status_message: String::new(),
            error_message: String::new(),
            show_help: false,
        })
    }

    fn load_variables(commands: &EnvMatchCommands, env_name: &str) -> Result<Vec<Variable>> {
        let vars = commands.list_variables(Some(env_name))?;
        Ok(vars
            .into_iter()
            .map(|(key, value)| Variable { key, value })
            .collect())
    }

    pub fn handle_key(&mut self, key: KeyCode) -> Result<()> {
        match self.state {
            AppState::EnvironmentList => self.handle_env_list_key(key)?,
            AppState::VariableList => self.handle_var_list_key(key)?,
            AppState::AddVariable => self.handle_add_var_key(key)?,
            AppState::EditVariable => self.handle_edit_var_key(key)?,
            AppState::ConfirmDelete => self.handle_confirm_delete_key(key)?,
        }
        Ok(())
    }

    fn handle_env_list_key(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('h') | KeyCode::F(1) => self.show_help = !self.show_help,
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_env_index > 0 {
                    self.selected_env_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_env_index < self.environments.len().saturating_sub(1) {
                    self.selected_env_index += 1;
                }
            }
            KeyCode::Enter => {
                if let Some(env) = self.environments.get(self.selected_env_index) {
                    self.switch_environment(env.clone())?;
                }
                self.state = AppState::VariableList;
            }
            KeyCode::Tab => self.state = AppState::VariableList,
            _ => {}
        }
        Ok(())
    }

    fn handle_var_list_key(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('h') | KeyCode::F(1) => self.show_help = !self.show_help,
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_var_index > 0 {
                    self.selected_var_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_var_index < self.variables.len().saturating_sub(1) {
                    self.selected_var_index += 1;
                }
            }
            KeyCode::Char('a') => {
                self.input_key.clear();
                self.input_buffer.clear();
                self.state = AppState::AddVariable;
            }
            KeyCode::Char('e') => {
                if let Some(var) = self.variables.get(self.selected_var_index) {
                    self.input_key = var.key.clone();
                    self.input_buffer = var.value.clone();
                    self.state = AppState::EditVariable;
                }
            }
            KeyCode::Char('d') | KeyCode::Delete => {
                if !self.variables.is_empty() {
                    self.state = AppState::ConfirmDelete;
                }
            }
            KeyCode::Tab => self.state = AppState::EnvironmentList,
            KeyCode::F(5) => self.refresh_variables()?,
            _ => {}
        }
        Ok(())
    }

    fn handle_add_var_key(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Esc => {
                self.state = AppState::VariableList;
                self.input_key.clear();
                self.input_buffer.clear();
            }
            KeyCode::Enter => {
                if self.input_key.is_empty() {
                    // We're entering the key
                    if !self.input_buffer.is_empty() {
                        self.input_key = self.input_buffer.clone();
                        self.input_buffer.clear();
                    }
                } else {
                    // We're entering the value, save the variable
                    self.add_variable()?;
                }
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_edit_var_key(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Esc => {
                self.state = AppState::VariableList;
                self.input_key.clear();
                self.input_buffer.clear();
            }
            KeyCode::Enter => {
                self.edit_variable()?;
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_confirm_delete_key(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                self.delete_variable()?;
                self.state = AppState::VariableList;
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                self.state = AppState::VariableList;
            }
            _ => {}
        }
        Ok(())
    }

    fn switch_environment(&mut self, env_name: String) -> Result<()> {
        self.commands.switch_environment(&env_name)?;
        self.current_environment = env_name.clone();
        self.variables = Self::load_variables(&self.commands, &env_name)?;
        self.selected_var_index = 0;
        self.status_message = format!("Switched to environment: {}", env_name);
        self.error_message.clear();
        Ok(())
    }

    fn add_variable(&mut self) -> Result<()> {
        if self.input_key.is_empty() || self.input_buffer.is_empty() {
            self.error_message = "Both key and value are required".to_string();
            return Ok(());
        }

        self.commands.set_variable(
            &self.input_key,
            &self.input_buffer,
            &self.current_environment,
        )?;
        self.refresh_variables()?;
        self.state = AppState::VariableList;
        self.status_message = format!("Added variable: {}={}", self.input_key, self.input_buffer);
        self.input_key.clear();
        self.input_buffer.clear();
        self.error_message.clear();
        Ok(())
    }

    fn edit_variable(&mut self) -> Result<()> {
        if self.input_buffer.is_empty() {
            self.error_message = "Value cannot be empty".to_string();
            return Ok(());
        }

        self.commands.set_variable(
            &self.input_key,
            &self.input_buffer,
            &self.current_environment,
        )?;
        self.refresh_variables()?;
        self.state = AppState::VariableList;
        self.status_message = format!("Updated variable: {}={}", self.input_key, self.input_buffer);
        self.input_key.clear();
        self.input_buffer.clear();
        self.error_message.clear();
        Ok(())
    }

    fn delete_variable(&mut self) -> Result<()> {
        if let Some(var) = self.variables.get(self.selected_var_index).cloned() {
            self.commands
                .unset_variable(&var.key, &self.current_environment)?;
            self.refresh_variables()?;
            self.status_message = format!("Deleted variable: {}", var.key);
            self.error_message.clear();

            // Adjust selected index if necessary
            if self.selected_var_index >= self.variables.len() && !self.variables.is_empty() {
                self.selected_var_index = self.variables.len() - 1;
            }
        }
        Ok(())
    }

    fn refresh_variables(&mut self) -> Result<()> {
        self.variables = Self::load_variables(&self.commands, &self.current_environment)?;
        Ok(())
    }
}
