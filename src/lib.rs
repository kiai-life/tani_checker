extern crate wasm_bindgen;
use anyhow::Context;
use wasm_bindgen::prelude::*;

pub mod config;
pub mod pattern;

use config::Config;

#[wasm_bindgen]
pub fn tani(config: &str) {
  let config: Config = toml::from_str(config)
    .with_context(|| "toml parser error")
    .unwrap();

  let v = pattern::coins::check(&config).unwrap();
  for (name, msg) in v.iter() {
    println!("{}: {}", name, msg)
  }
}
