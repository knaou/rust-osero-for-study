use std::fmt;
use super::player::Player;

// `Cell` 列挙型：ボードの各マスの状態を表します。
#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Occupied(Player), // 誰かの石が置いてある状態（石の所有者の情報を持ちます）
}

// マスを表示するための設定
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Occupied(player) => write!(f, "{}", player),
        }
    }
}

// `Board` 構造体：8x8 の盤面を管理します。
pub struct Board {
    // 2次元配列。Rust の配列は `[型; 長さ]` と書きます。
    cells: [[Cell; 8]; 8],
}

impl Board {
    // 新しい盤面を作成するコンストラクタ的なメソッド
    // `Self` は `Board` 型自身を指すエイリアスです。
    pub fn new() -> Self {
        let mut cells = [[Cell::Empty; 8]; 8];
        // 初期配置（オセロの中央 4 つの石）
        cells[3][3] = Cell::Occupied(Player::White);
        cells[3][4] = Cell::Occupied(Player::Black);
        cells[4][3] = Cell::Occupied(Player::Black);
        cells[4][4] = Cell::Occupied(Player::White);
        Board { cells }
    }

    // 盤面を表示する
    // `&self` は「読み取り専用の借用」を意味します。
    // このメソッドは盤面を読み取るだけで、書き換え（変更）はしないことを保証します。
    pub fn display(&self) {
        println!("  0 1 2 3 4 5 6 7");
        for r in 0..8 {
            print!("{} ", r);
            for c in 0..8 {
                // `self.cells[r][c]` を表示。Display トレイトを実装しているので `{}` で表示可能です。
                print!("{} ", self.cells[r][c]);
            }
            println!();
        }
    }

    // 指定した場所に石を置けるかチェックする
    pub fn is_valid_move(&self, r: i32, c: i32, player: Player) -> bool {
        // 盤面の範囲外なら false
        if r < 0 || r >= 8 || c < 0 || c >= 8 {
            return false;
        }
        // すでに石がある場所なら false
        // `if let` は「特定のパターンにマッチする場合のみ実行する」制御構文です。
        if let Cell::Occupied(_) = self.cells[r as usize][c as usize] {
            return false;
        }

        // 全 8 方向（上下左右＋斜め）をチェック
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                if self.can_flip_in_direction(r, c, dr, dc, player) {
                    return true;
                }
            }
        }
        false
    }

    // 特定の方向で相手の石を挟めるか判定する内部関数
    fn can_flip_in_direction(&self, r: i32, c: i32, dr: i32, dc: i32, player: Player) -> bool {
        let mut nr = r + dr;
        let mut nc = c + dc;
        let mut count = 0;

        while nr >= 0 && nr < 8 && nc >= 0 && nc < 8 {
            match self.cells[nr as usize][nc as usize] {
                Cell::Empty => return false,
                // `if p == player` はガード条件です。マッチングに追加の条件を付けます。
                Cell::Occupied(p) if p == player => return count > 0,
                Cell::Occupied(_) => {
                    count += 1;
                    nr += dr;
                    nc += dc;
                }
            }
        }
        false
    }

    // 実際に石を置く処理
    // `&mut self` は「可変の借用」を意味します。
    // Rust ではデフォルトで変数は不変（変更不可）ですが、`mut` をつけることで
    // 自身のデータ（この場合は盤面）を書き換えることができるようになります。
    pub fn make_move(&mut self, r: i32, c: i32, player: Player) -> bool {
        if !self.is_valid_move(r, c, player) {
            return false;
        }

        // 石を置く。`usize` は符号なし整数型で、配列のインデックスとして使われます。
        // C言語の `int` と違い、Rust は型に厳格なので `i32` を `usize` にキャスト（as）する必要があります。
        self.cells[r as usize][c as usize] = Cell::Occupied(player);

        // 挟んだ石を全て裏返す
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                if self.can_flip_in_direction(r, c, dr, dc, player) {
                    self.flip_in_direction(r, c, dr, dc, player);
                }
            }
        }
        true
    }

    // 石を裏返す処理（内部関数）
    fn flip_in_direction(&mut self, r: i32, c: i32, dr: i32, dc: i32, player: Player) {
        let mut nr = r + dr;
        let mut nc = c + dc;

        while nr >= 0 && nr < 8 && nc >= 0 && nc < 8 {
            match self.cells[nr as usize][nc as usize] {
                // 自分の石に到達したら終了
                Cell::Occupied(p) if p == player => return,
                // 相手の石なら自分の色に書き換える
                Cell::Occupied(_) => {
                    self.cells[nr as usize][nc as usize] = Cell::Occupied(player);
                    nr += dr;
                    nc += dc;
                }
                Cell::Empty => return,
            }
        }
    }

    // プレイヤーがどこかに置ける場所があるか確認する
    pub fn has_valid_move(&self, player: Player) -> bool {
        for r in 0..8 {
            for c in 0..8 {
                if self.is_valid_move(r, c, player) {
                    return true;
                }
            }
        }
        false
    }

    // 各色の石の数を数える
    // 戻り値の `(i32, i32)` は「タプル」という型で、複数の値を一度に返せます（Python と同様）。
    pub fn count_stones(&self) -> (i32, i32) {
        let mut black = 0;
        let mut white = 0;
        for r in 0..8 {
            for c in 0..8 {
                match self.cells[r][c] {
                    Cell::Occupied(Player::Black) => black += 1,
                    Cell::Occupied(Player::White) => white += 1,
                    Cell::Empty => {}
                }
            }
        }
        (black, white)
    }
}
