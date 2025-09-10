use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::TempDir;

struct TestEnv {
    _temp_dir: TempDir,
    work_dir: std::path::PathBuf,
}

impl TestEnv {
    fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let work_dir = temp_dir.path().to_path_buf();
        Self {
            _temp_dir: temp_dir,
            work_dir,
        }
    }

    fn cmd(&self) -> Command {
        let mut cmd = Command::cargo_bin("envMatch").unwrap();
        cmd.current_dir(&self.work_dir);
        cmd
    }
}

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("envMatch").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Environment Variable Manager"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("envMatch").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("envMatch 0.1.0"));
}

#[test]
fn test_init_command() {
    let test_env = TestEnv::new();
    
    test_env.cmd()
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ envMatch initialized successfully!"));

    // Verify .envMatch directory was created
    assert!(test_env.work_dir.join(".envMatch").exists());
    assert!(test_env.work_dir.join(".envMatch/config.yaml").exists());
    assert!(test_env.work_dir.join(".envMatch/environments/development.yaml").exists());
}

#[test]
fn test_init_already_initialized() {
    let test_env = TestEnv::new();
    
    // Initialize first time
    test_env.cmd().arg("init").assert().success();
    
    // Try to initialize again
    test_env.cmd()
        .arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::contains("envMatch already initialized"));
}

#[test]
fn test_set_and_get_variable() {
    let test_env = TestEnv::new();
    
    // Initialize
    test_env.cmd().arg("init").assert().success();
    
    // Set a variable
    test_env.cmd()
        .args(&["set", "TEST_VAR", "test_value"])
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Set TEST_VAR=test_value"));
    
    // Get the variable
    test_env.cmd()
        .args(&["get", "TEST_VAR"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test_value"));
}

#[test]
fn test_set_variable_in_specific_environment() {
    let test_env = TestEnv::new();
    
    test_env.cmd().arg("init").assert().success();
    
    test_env.cmd()
        .args(&["set", "PROD_VAR", "prod_value", "--env", "production"])
        .assert()
        .success()
        .stdout(predicate::str::contains("production"));
    
    test_env.cmd()
        .args(&["get", "PROD_VAR", "--env", "production"])
        .assert()
        .success()
        .stdout(predicate::str::contains("prod_value"));
}

#[test]
fn test_get_nonexistent_variable() {
    let test_env = TestEnv::new();
    
    test_env.cmd().arg("init").assert().success();
    
    test_env.cmd()
        .args(&["get", "NONEXISTENT"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Variable 'NONEXISTENT' not found"));
}

#[test]
fn test_unset_variable() {
    let test_env = TestEnv::new();
    
    test_env.cmd().arg("init").assert().success();
    
    // Set a variable
    test_env.cmd()
        .args(&["set", "TEMP_VAR", "temp_value"])
        .assert()
        .success();
    
    // Unset the variable
    test_env.cmd()
        .args(&["unset", "TEMP_VAR"])
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Removed 'TEMP_VAR'"));
    
    // Verify it's gone
    test_env.cmd()
        .args(&["get", "TEMP_VAR"])
        .assert()
        .failure();
}

#[test]
fn test_switch_environment() {
    let test_env = TestEnv::new();
    
    test_env.cmd().arg("init").assert().success();
    
    // Switch to production
    test_env.cmd()
        .args(&["switch", "production"])
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Switched to environment 'production'"));
    
    // Check current environment
    test_env.cmd()
        .arg("current")
        .assert()
        .success()
        .stdout(predicate::str::contains("production"));
}

#[test]
fn test_list_variables() {
    let test_env = TestEnv::new();
    
    test_env.cmd().arg("init").assert().success();
    
    // Set some variables
    test_env.cmd().args(&["set", "VAR1", "value1"]).assert().success();
    test_env.cmd().args(&["set", "VAR2", "value2"]).assert().success();
    
    // List variables
    test_env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("VAR1=value1"))
        .stdout(predicate::str::contains("VAR2=value2"));
}

#[test]
fn test_list_environments() {
    let test_env = TestEnv::new();
    
    test_env.cmd().arg("init").assert().success();
    
    // Create a variable in production to ensure it exists
    test_env.cmd()
        .args(&["set", "PROD_VAR", "value", "--env", "production"])
        .assert()
        .success();
    
    // List environments
    test_env.cmd()
        .arg("envs")
        .assert()
        .success()
        .stdout(predicate::str::contains("development (current)"))
        .stdout(predicate::str::contains("production"));
}

#[test]
fn test_validate_environment() {
    let test_env = TestEnv::new();
    
    test_env.cmd().arg("init").assert().success();
    
    // Set required variables
    test_env.cmd().args(&["set", "DATABASE_URL", "postgres://localhost"]).assert().success();
    test_env.cmd().args(&["set", "API_KEY", "secret123"]).assert().success();
    
    // Validate with all required variables present
    test_env.cmd()
        .args(&["validate", "--required", "DATABASE_URL,API_KEY"])
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ All required variables are set"));
    
    // Validate with missing variable
    test_env.cmd()
        .args(&["validate", "--required", "DATABASE_URL,API_KEY,MISSING_VAR"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Missing required variables"));
}

#[test]
fn test_commands_without_init() {
    let test_env = TestEnv::new();
    
    // Try to set variable without init
    test_env.cmd()
        .args(&["set", "VAR", "value"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("envMatch not initialized"));
    
    // Try to get variable without init
    test_env.cmd()
        .args(&["get", "VAR"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("envMatch not initialized"));
}
