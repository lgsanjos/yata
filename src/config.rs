use serde::Deserialize;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub db_path: String,
    pub status: Option<Vec<Status>>,
    pub project: Option<Vec<Project>>,
}

#[derive(Deserialize)]
pub struct Status {
    pub name: String,
    pub shorten: String,
    pub order: u32,
}

#[derive(Deserialize)]
pub struct Project {
    pub name: String,
    pub shorten: String,
    pub order: u32,
}

pub fn default_config() -> String {
    r#"
    db_path = "~/.yat.db"

    [[status]]
    name = "TODO"
    shorten = "T"
    order = 0

    [[status]]
    name = "IN PROGRESS"
    shorten = "IP"
    order = 1

    [[status]]
    name = "DONE"
    shorten = "DD"
    order = 2

    [[project]]
    name = "Personal"
    shorten = "P"
    order = 0

    [[project]]
    name = "Work"
    shorten = "W"
    order = 1
[[project]]
    name = "Other"
    shorten = "O"
    order = 2
    "#
    .to_string()
}

pub fn parse_config(config: String) -> Result<Config, toml::de::Error> {
    toml::from_str(&config)
}

pub fn load_config_file(filepath: String) -> Result<Config, toml::de::Error> {
    match std::fs::read_to_string(filepath) {
        Ok(config) => parse_config(config),
        Err(_) => parse_config(default_config()),
    }
}

#[cfg(test)]
#[test]
fn test_load_default_config_into_objects() {
    let config: Config = parse_config(default_config()).unwrap();

    assert_eq!(config.db_path, "~/.yat/sqlite.db");
    let status = config.status.unwrap();
    assert_eq!(status.len(), 3);
    assert_eq!(status[0].name, "TODO");
    assert_eq!(status[0].shorten, "T");
    assert_eq!(status[0].order, 0);
    assert_eq!(status[2].name, "DONE");
    assert_eq!(status[2].shorten, "DD");
    assert_eq!(status[2].order, 2);

    assert_eq!(config.project.unwrap().len(), 3);
}

#[test]
fn test_load_config_missing_status_and_projects() {
    let result: Result<Config, toml::de::Error> =
        parse_config("db_path = \"~/.yat.db\"\n".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_load_invalid_config() {
    let result: Result<Config, toml::de::Error> = parse_config("invalid toml file".to_string());
    assert_eq!(result.err().unwrap().to_string(), "TOML parse error at line 1, column 9\n  |\n1 | invalid toml file\n  |         ^\nexpected `.`, `=`\n");

    let result: Result<Config, toml::de::Error> = parse_config("invalid=123".to_string());
    assert_eq!(result.err().unwrap().to_string(), "TOML parse error at line 1, column 1\n  |\n1 | invalid=123\n  | ^^^^^^^^^^^\nmissing field `db_path`\n");
}
