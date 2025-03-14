# sherbet

A simple web server that serves static files wrritten in Rust.

avaiceがRustの勉強用に開発している静的ファイルを配信するWebサーバです。Rust難しいよお

## Usage

PORTは環境変数で指定できます。デフォルトは8080です。

```sh
$ sharbet
```

## Build
```sh
$ cargo build --release
```

## sherbet script

sherbet scriptでは、API開発に必要な機能を提供します。

- ユーザー認証
- DB操作
- HTTP通信
- ログ出力
- JSON操作
- 簡単な処理系
  - if文
  - for文
  - 変数