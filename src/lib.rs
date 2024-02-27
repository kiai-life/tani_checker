extern crate wasm_bindgen;
use anyhow::Context;
use wasm_bindgen::prelude::*;

pub mod config;
pub mod pattern;

use config::Config;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub fn tani(config: &str) {
  log(config);
  let config: Config = toml::from_str(config)
    .with_context(|| "toml parser error")
    .unwrap();

  let v = pattern::coins22::check(&config).unwrap();
  for c in v.iter() {
    log(&c.msg());
  }
}
