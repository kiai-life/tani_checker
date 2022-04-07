# tani_checker

筑波大学での単位数の計算を行います。

現在のところ情報学群情報科学類のみに対応しています。

対応学類を増やしたい場合は、その学類に対応した`check`関数を追加するプルリクを送ってください。

# インストール方法

## Rust, Cargoの環境構築が済んでいる場合

```sh
cargo install --git "https://github.com/kiai-life/tani_checker.git"
```

によってインストールすることができます。


## Rust, Cargoの環境構築からしたい方


[Rust公式ホームページのインストールについてのページ](https://www.rust-lang.org/ja/tools/install)を見てインストールしてください。

**2022/04/07時点ではWindows利用者はVisual Studio C++ Build toolsをインストールする必要があります。**


環境構築が済んだら

```sh
cargo install --git "https://github.com/kiai-life/tani_checker.git"
```

インストールできます。


# 使用方法

自分の履修計画を反映した以下のようなTOMLファイルを作ります。

```toml
[[class]]
  id = "31HG122"
  name = "English Reading Skills I"
  credits = 1.0


[[class]]
  id = "GA18212"
  name = "プログラミング入門A"
  credits = 2.0
  get = false
```

- `id`: 教科コード
- `name`: 授業タイトル
- `credits`: 認定される単位（小数）
- `get`: 単位を取れたかどうかの真偽値（書かないと`true`）


このTOMLファイルを使って

```sh
tani_checker --name <name> <toml file path>
```

のように起動します。

`<name>`には`coins`のような学類の名前を入れます。現在は`coins`蚤の対応です。

`<toml file path>`には保存したTOMLファイルへのpathを与えます。


例えば

```sh
tani_checker --name coins ./tsukuba/tani.toml
```

のようにです。

# ライセンス(LICENSE)

MITライセンスのもと、公開・配布されます。

This package released under [the MIT license](https://github.com/kiai-life/tani_checker/blob/master/LICENSE).

---

(c) 2022 Naoki Kaneko (a.k.a. "puripuri2100")
