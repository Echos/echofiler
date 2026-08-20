# echofiler
#
# インストール処理の実体は install.sh にある。ここでは薄いラッパーのみ提供する。
# 例:
#   make install                       # ~/.local/bin へインストール
#   make install FEATURES=all          # 全機能を有効化してインストール
#   make install PREFIX=/usr/local     # システム全体へインストール (sudo が必要)
#   make uninstall
#   make purge

PREFIX   ?= $(HOME)/.local
FEATURES ?=
INSTALL_FLAGS ?=

CARGO_FEATURE_FLAG := $(if $(FEATURES),--features $(FEATURES),)
INSTALL_FEATURE_FLAG := $(if $(FEATURES),--features $(FEATURES),)

.PHONY: all build release run test fmt lint clean install install-dry uninstall purge help

all: build

build:
	cargo build $(CARGO_FEATURE_FLAG)

release:
	cargo build --release $(CARGO_FEATURE_FLAG)

run:
	cargo run $(CARGO_FEATURE_FLAG)

test:
	cargo test $(CARGO_FEATURE_FLAG)

fmt:
	cargo fmt

lint:
	cargo clippy $(CARGO_FEATURE_FLAG) -- -D warnings

clean:
	cargo clean

install:
	./install.sh --prefix "$(PREFIX)" $(INSTALL_FEATURE_FLAG) $(INSTALL_FLAGS)

install-dry:
	./install.sh --prefix "$(PREFIX)" $(INSTALL_FEATURE_FLAG) --dry-run $(INSTALL_FLAGS)

uninstall:
	./install.sh --uninstall --prefix "$(PREFIX)" $(INSTALL_FLAGS)

purge:
	./install.sh --uninstall --purge --prefix "$(PREFIX)" $(INSTALL_FLAGS)

help:
	@./install.sh --help
