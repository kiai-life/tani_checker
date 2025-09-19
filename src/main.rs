//! [![Build Status][ci-badge]][ci]
//! [![source badge][source-badge]][source]
//! [![license badge][license-badge]][license]
//! [![msrv badge]][msrv link]
//!
//! [ci]: https://github.com/kiai-life/tani_checker/actions/workflows/rust.yml
//! [ci-badge]: https://github.com/kiai-life/tani_checker/actions/workflows/rust.yml/badge.svg
//! [source]: https://github.com/kiai-life/tani_checker
//! [source-badge]: https://img.shields.io/badge/source-github-blue
//! [license]: https://github.com/kiai-life/tani_checker/blob/master/LICENSE
//! [license-badge]: https://img.shields.io/badge/license-MIT-blue
//! [msrv link]: https://blog.rust-lang.org/2023/08/03/Rust-1.71.1
//! [msrv badge]: https://img.shields.io/badge/rustc-1.71.1+-93450a
//!
//! # tani_checker
//!
//! 筑波大学での単位数の計算を行います。
//!
//! 現在のところ情報学群情報科学類のみに対応しています。
//!
//! 対応学類を増やしたい場合は、その学類に対応した`check`関数を追加するプルリクを送ってください。
//!
//! # インストール方法
//!
//! ## Rust, Cargoの環境構築が済んでいる場合
//!
//! ```sh
//! cargo install --git "https://github.com/kiai-life/tani_checker.git"
//! ```
//!
//! によってインストールすることができます。
//!
//!
//! ## Rust, Cargoの環境構築からしたい方
//!
//!
//! [Rust公式ホームページのインストールについてのページ](https://www.rust-lang.org/ja/tools/install)を見てインストールしてください。
//!
//! **2022/04/07時点ではWindows利用者はVisual Studio C++ Build toolsをインストールする必要があります。**
//!
//!
//! 環境構築が済んだら
//!
//! ```sh
//! cargo install --git "https://github.com/kiai-life/tani_checker.git"
//! ```
//!
//! インストールできます。
//!
//!
//! # 使用方法
//!
//! ## twinsからダウンロードしたCSVファイルを用いる場合
//!
//! twinsの成績ページからダウンロードページに行き、ファイル形式を「CSV」に、文字コードを「Unicode(UTF-8)」に指定してファイルをダウンロードしてくだ//! さい。
//!
//!
//! このCSVファイルを使って
//!
//! ```sh
//! tani_checker --name <name> <csv file path>
//! ```
//!
//! のように起動します。
//!
//! `<name>`には`coins22`のような学類の名前を入れます。現在は`coins22`のみの対応です。
//!
//! `<csv file path>`にはダウンロードしたCSVファイルへのpathを与えます。
//!
//!
//! ## TOMLファイルを作る場合
//!
//! 自分の履修計画を反映した以下のようなTOMLファイルを作ります。
//!
//! ```toml
//! [[class]]
//!   id = "31HG122"
//!   name = "English Reading Skills I"
//!   credits = 1
//!
//!
//! [[class]]
//!   id = "GA18212"
//!   name = "プログラミング入門A"
//!   credits = 2
//!   get = false
//! ```
//!
//! - `id`: 教科コード
//! - `name`: 授業タイトル
//! - `credits`: 認定される単位
//! - `get`: 単位を取れたかどうかの真偽値（書かないと`true`）
//!
//!
//! 詳しい実例は[`examples/example.toml`](./examples/example.toml)を参照してください
//!
//!
//! このTOMLファイルを使って
//!
//! ```sh
//! tani_checker --name <name> <toml file path>
//! ```
//!
//! のように起動します。
//!
//! `<name>`には`coins22`のような学類の名前を入れます。現在は`coins22`のみの対応です。
//!
//! `<toml file path>`には保存したTOMLファイルへのpathを与えます。
//!
//!
//! 例えば
//!
//! ```sh
//! tani_checker --name coins22 ./tsukuba/tani.toml
//! ```
//!
//! のようにです。
//!
//! # ライセンス(LICENSE)
//!
//! MITライセンスのもと、公開・配布されます。
//!
//! This package released under [the MIT license](https://github.com/kiai-life/tani_checker/blob/master/LICENSE).
//!
//! ---
//!
//! (c) 2022 Naoki Kitano(Kaneko) (a.k.a. "puripuri2100")
//!

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

pub mod config;
pub mod pattern;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  /// 科目名まで出力するようにする
  #[clap(short, long)]
  verbose: bool,

  /// 履修中のものを含めず、現在の取得状況を確認する
  #[clap(short, long)]
  current: bool,

  /// ex: "coins22"
  #[clap(short, long)]
  name: String,

  /// sets a config file
  #[clap(value_name = "FILE")]
  config: PathBuf,
}

fn main() -> Result<()> {
  let args = Args::parse();

  let file_name = &args.config;
  let config = match file_name.extension().map(|s| s.to_str().unwrap()) {
    Some("toml") => config::make_config_data_toml(file_name)?,
    Some("csv") => config::make_config_data_csv(file_name)?,
    _ => todo!(),
  };

  match &*args.name {
    "coins22" => {
      let v = pattern::coins22::check(&config, args.current)?;
      for c in v.iter() {
        println!("{}", &*c.msg(args.verbose))
      }
      Ok(())
    }
    _ => {
      todo!()
    }
  }
}
