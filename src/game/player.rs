use std::fmt;

// `enum`（列挙型）は、いくつかの選択肢のうちの一つを表す型です。
// C言語の enum に似ていますが、Rust の enum はもっと強力で、各バリアントにデータを持たせることもできます。
// `#[derive(...)]` は「自動実装」の機能です。
// - Clone, Copy: 値をコピー可能にします（Cの単純な構造体や整数のように扱えます）
// - PartialEq: `==` で比較可能にします
#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    Black, // 黒番
    White, // 白番
}

// `impl` ブロックを使って、特定の型にメソッドを定義します。
impl Player {
    // `self` は Python の `self` と同様、インスタンス自身を指します。
    // ここでは `Copy` トレイトを実装しているため、所有権の移動ではなく値のコピーが発生します。
    pub fn opponent(self) -> Player {
        // `match` は強力なパターンマッチングです。Cの switch-case に似ていますが、
        // 全てのケースを網羅していることをコンパイラがチェックしてくれます（漏れがあるとエラーになります）。
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

// `fmt::Display` トレイトを実装することで、`println!("{}", player)` のように
// 文字列として表示した時の挙動を定義できます（Python の __str__ に相当します）。
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // `write!` マクロは、指定されたバッファ（f）に書き込みます。
            Player::Black => write!(f, "○"), // 黒は○
            Player::White => write!(f, "●"), // 白は●
        }
    }
}
