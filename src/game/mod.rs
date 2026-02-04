// Rust のモジュールシステムでは、フォルダ構造とモジュール構造が対応しています。
// この `mod.rs` は、`game` というフォルダ（モジュール）の入り口となるファイルです。

// `pub mod` を使うことで、他のファイル（例えば main.rs）から
// このモジュール内の子モジュール（player や board）にアクセスできるようになります。

pub mod player; // player.rs ファイルの内容を `game::player` モジュールとして公開します
pub mod board;  // board.rs ファイルの内容を `game::board` モジュールとして公開します
