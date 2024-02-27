use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use toml;

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Config {
  pub class: Vec<Class>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Class {
  pub id: String,
  pub name: String,
  pub credits: usize,
  pub get: Option<bool>,
}

pub fn make_config_data(path: &Path) -> Result<Config> {
  let content = fs::read_to_string(path)?;
  let config: Config = toml::from_str(&content).with_context(|| "toml parser error")?;
  Ok(config)
}
