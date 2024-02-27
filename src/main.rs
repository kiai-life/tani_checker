use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

pub mod config;
pub mod pattern;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  /// ex: "coins22"
  #[clap(short, long)]
  name: String,

  /// sets a config file
  #[clap(value_name = "FILE")]
  config: PathBuf,
}

fn main() -> Result<()> {
  let args = Args::parse();

  let config = config::make_config_data(&args.config)?;

  match &*args.name {
    "coins22" => {
      let v = pattern::coins22::check(&config)?;
      for c in v.iter() {
        println!("{}", &*c.msg())
      }
      Ok(())
    }
    _ => {
      todo!()
    }
  }
}
