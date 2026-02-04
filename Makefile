.PHONY: all build run test clean help

# デフォルトターゲット
all: build

# ビルド
build:
	cargo build

# 実行
run:
	cargo run

# テスト
test:
	cargo test

# クリーンアップ
clean:
	cargo clean

# ヘルプ
help:
	@echo "利用可能なターゲット:"
	@echo "  make build  - プロジェクトをビルドします"
	@echo "  make run    - アプリケーションを実行します"
	@echo "  make test   - テストを実行します"
	@echo "  make clean  - ビルド成果物を削除します"
	@echo "  make help   - このヘルプを表示します"
