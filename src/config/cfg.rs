use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Action {
    pub action_type: String,
    pub data: String
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub regex_match: String,
    pub actions: Option<Vec<Action>>
}

#[derive(Debug, Deserialize)]
pub struct Env {
    pub name: String,
    pub value: String
}

#[derive(Debug, Deserialize)]
pub struct Process {
    pub command: String,
    pub env: Option<Vec<Env>>,
    pub rules: Vec<Rule>
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default="bool::default")]
    pub debug: bool,
    pub processes: Vec<Process>
}

pub fn load_cfg(file_name: String) -> Config {
    let json = fs::read_to_string(file_name).expect("Error reading CFG file.");

    let cfg: Config = serde_json::from_str(json.as_str()).expect("JSON format error.");

    return cfg;
}