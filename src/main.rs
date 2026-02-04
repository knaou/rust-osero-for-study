// `mod game;` は、`game` という名前のモジュール（フォルダやファイル）があることを宣言します。
mod game;

// `use` は他のモジュールの機能を現在のスコープに持ち込みます（Python の import と同様）。
use std::io::{self, Write};
use crate::game::player::Player;
use crate::game::board::Board;

// プログラムの実行開始点（エントリポイント）です。
fn main() {
    // `mut` キーワードは重要です。Rust では変数はデフォルトで不変（immutable）です。
    // 値を変更する必要がある場合は、明示的に `mut` をつけて可変にする必要があります。
    let mut board = Board::new();
    let mut current_player = Player::Black;

    // `loop` は無限ループを作成します。
    loop {
        board.display();
        let (black, white) = board.count_stones();
        println!("Black (○): {}, White (●): {}", black, white);

        // パス（置ける場所がない）の判定
        if !board.has_valid_move(current_player) {
            // 相手も置けないならゲーム終了
            if !board.has_valid_move(current_player.opponent()) {
                println!("No moves left for both players. Game over.");
                break; // ループを抜ける
            }
            println!("No moves left for {}. Skipping turn.", current_player);
            current_player = current_player.opponent(); // 交代
            continue; // 次のループ（相手の番）へ
        }

        println!("Current player: {} ({}'s turn)", current_player, if current_player == Player::Black { "Black" } else { "White" });
        print!("Enter coordinates (row col), e.g., '3 2': ");
        
        // 標準出力を即座に表示させるためのフラッシュ
        // `unwrap()` は、エラーが発生した時にプログラムを強制終了させる Rust の「パニック」機能です。
        // 本来は適切にエラー処理をすべきですが、確実な場所では簡略化のために使われることがあります。
        io::stdout().flush().unwrap();

        let mut input = String::new();
        // 標準入力から 1 行読み取ります。`&mut input` は「可変の参照」を渡しています。
        // C言語のポインタに似ていますが、Rust は安全性が保証されています。
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // 入力文字列を数値に変換する処理
        // 関数型プログラミングのようなメソッドチェーン（split -> filter_map -> collect）を使っています。
        let coords: Vec<i32> = input
            .split_whitespace() // 空白で区切る
            .filter_map(|s| s.parse().ok()) // 数値に変換。失敗したものは除外
            .collect(); // 結果を Vec（可変長配列）に集約

        if coords.len() != 2 {
            println!("Invalid input. Please enter two numbers separated by space.");
            continue;
        }

        let (r, c) = (coords[0], coords[1]);
        // `make_move` は成功すると true を返します。
        // ボードの状態を書き換えるため、`mut` で宣言された `board` が必要です。
        if board.make_move(r, c, current_player) {
            current_player = current_player.opponent(); // 成功したら次のプレイヤーへ
        } else {
            println!("Invalid move. Try again.");
        }
    }

    // ゲーム終了後の結果表示
    let (black, white) = board.count_stones();
    board.display();
    println!("Final score - Black: {}, White: {}", black, white);
    if black > white {
        println!("Black wins!");
    } else if white > black {
        println!("White wins!");
    } else {
        println!("It's a tie!");
    }
}
