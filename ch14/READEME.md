## cargo と crate.ioについて
- cargo build は opt-level = 0
- cargo build --release は opt-level = 3
- opt-levelは最適化の度合い　コンパイルの速度とトレードオフ

- `cargo doc --open`で`///`or`//!`の部分からドキュメントを作成する
- `cargo test`でコメントないのコードもテストしてくれる

- pub use で非公開構造とは異なる公開構造にできる

- cargo publishでcrates.ioに公開できる

- workspaceで複数のバイナリクレートとライブラリクレートを管理する

- `cargo install`でバイナリパッケージの外部クレートをインストールできる



