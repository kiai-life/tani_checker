use anyhow::{Context, Result};
use csv::ReaderBuilder;
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

pub fn make_config_data_toml(path: &Path) -> Result<Config> {
  let content = fs::read_to_string(path)?;
  let config: Config = toml::from_str(&content).with_context(|| "toml parser error")?;
  Ok(config)
}

pub fn make_config_data_csv(path: &Path) -> Result<Config> {
  let content = fs::read_to_string(path)?;
  let mut rdr = ReaderBuilder::new().from_reader(content.as_bytes());
  let mut class = Vec::new();
  while let Some(Ok(result)) = rdr.records().next() {
    let record = result;
    let id = record.get(2).unwrap().trim().to_string();
    let name = record.get(3).unwrap().trim().to_string();
    let credits_f = record.get(4).unwrap().trim().parse::<f64>().unwrap();
    let credits = credits_f as usize;
    let result = record.get(7).unwrap().trim().to_string();
    let get = match result.as_str() {
      "A+" | "A" | "B" | "C" | "P" => Some(true),
      "D" | "F" => Some(false),
      _ => None,
    };
    let c = Class {
      id,
      name,
      credits,
      get,
    };
    class.push(c);
  }
  Ok(Config { class })
}
