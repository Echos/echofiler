#!/usr/bin/env bash
#
# echofiler インストーラー
#
# バイナリと初期設定ファイルをXDG Base Directoryに従って配置する。
# 使い方は ./install.sh --help を参照。
#
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
APP_NAME="echofiler"
CONFIG_SRC_DIR="$SCRIPT_DIR/config/default"

# アプリが読み込む設定ファイル (src/config/mod.rs のパス解決と対応)
CONFIG_FILES=(echofiler.toml theme.toml keymap.toml opener.toml)

# ---------------------------------------------------------------------------
# 既定値 (環境変数で上書き可能)
# ---------------------------------------------------------------------------
if [ "$(id -u)" -eq 0 ]; then
    PREFIX="${PREFIX:-/usr/local}"
else
    PREFIX="${PREFIX:-$HOME/.local}"
fi
CONFIG_DIR="${CONFIG_DIR:-${XDG_CONFIG_HOME:-$HOME/.config}/$APP_NAME}"
DATA_DIR="${DATA_DIR:-${XDG_DATA_HOME:-$HOME/.local/share}/$APP_NAME}"

FEATURES=""
BIN_SRC=""
DO_BUILD=1
FORCE=0
DRY_RUN=0
MODE="install"
PURGE=0
ASSUME_YES=0
WITH_EXAMPLE_PLUGIN=0

# ---------------------------------------------------------------------------
# ユーティリティ
# ---------------------------------------------------------------------------
if [ -t 1 ]; then
    C_INFO=$'\033[36m'; C_OK=$'\033[32m'; C_WARN=$'\033[33m'; C_ERR=$'\033[31m'; C_OFF=$'\033[0m'
else
    C_INFO=""; C_OK=""; C_WARN=""; C_ERR=""; C_OFF=""
fi

info()  { printf '%s==>%s %s\n' "$C_INFO" "$C_OFF" "$*"; }
ok()    { printf '%s  ok%s %s\n' "$C_OK" "$C_OFF" "$*"; }
skip()  { printf '  -- %s\n' "$*"; }
warn()  { printf '%s警告:%s %s\n' "$C_WARN" "$C_OFF" "$*" >&2; }
die()   { printf '%sエラー:%s %s\n' "$C_ERR" "$C_OFF" "$*" >&2; exit 1; }

# dry-run 対応の実行ラッパー
run() {
    if [ "$DRY_RUN" -eq 1 ]; then
        printf '  [dry-run] %s\n' "$*"
    else
        "$@"
    fi
}

app_version() {
    awk -F'"' '/^version[[:space:]]*=/ { print $2; exit }' "$SCRIPT_DIR/Cargo.toml"
}

usage() {
    cat <<'EOS'
echofiler インストーラー

使い方:
  ./install.sh [オプション]              インストール
  ./install.sh --uninstall [--purge]    アンインストール

インストール先:
  バイナリ      $PREFIX/bin/echofiler          (既定: ~/.local/bin, rootなら /usr/local/bin)
  設定ファイル  $CONFIG_DIR/*.toml             (既定: ${XDG_CONFIG_HOME:-~/.config}/echofiler)
  プラグイン    $CONFIG_DIR/plugins/
  ログ・データ  $DATA_DIR/                     (既定: ${XDG_DATA_HOME:-~/.local/share}/echofiler)

オプション:
  --prefix DIR          バイナリのインストール先プレフィックス
  --config-dir DIR      設定ディレクトリ
  --data-dir DIR        データ(ログ)ディレクトリ
  --features LIST       cargo のfeature指定 (例: preview,archive,plugin / all)
  --no-build            ビルドせず target/release の既存バイナリを使う
  --bin PATH            指定したビルド済みバイナリをインストールする (ビルド不要)
  --with-example-plugin サンプルプラグインを有効な状態で配置する
  --force               既存の設定ファイルを上書きする (旧ファイルは .bak-<日時> に退避)
  --uninstall           バイナリを削除する (設定は残す)
  --purge               --uninstall と併用し設定・データも削除する
  -y, --yes             確認プロンプトを省略する
  -n, --dry-run         実際には変更せず、実行内容のみ表示する
  -h, --help            このヘルプを表示する

環境変数 PREFIX / CONFIG_DIR / DATA_DIR でも既定値を変更できる。
EOS
}

# ---------------------------------------------------------------------------
# 引数解析
# ---------------------------------------------------------------------------
while [ $# -gt 0 ]; do
    case "$1" in
        --prefix)       [ $# -ge 2 ] || die "--prefix に値が必要です";       PREFIX="$2"; shift 2 ;;
        --prefix=*)     PREFIX="${1#*=}"; shift ;;
        --config-dir)   [ $# -ge 2 ] || die "--config-dir に値が必要です";   CONFIG_DIR="$2"; shift 2 ;;
        --config-dir=*) CONFIG_DIR="${1#*=}"; shift ;;
        --data-dir)     [ $# -ge 2 ] || die "--data-dir に値が必要です";     DATA_DIR="$2"; shift 2 ;;
        --data-dir=*)   DATA_DIR="${1#*=}"; shift ;;
        --features)     [ $# -ge 2 ] || die "--features に値が必要です";     FEATURES="$2"; shift 2 ;;
        --features=*)   FEATURES="${1#*=}"; shift ;;
        --bin)          [ $# -ge 2 ] || die "--bin に値が必要です";          BIN_SRC="$2"; DO_BUILD=0; shift 2 ;;
        --bin=*)        BIN_SRC="${1#*=}"; DO_BUILD=0; shift ;;
        --no-build)     DO_BUILD=0; shift ;;
        --with-example-plugin) WITH_EXAMPLE_PLUGIN=1; shift ;;
        --force)        FORCE=1; shift ;;
        --uninstall)    MODE="uninstall"; shift ;;
        --purge)        PURGE=1; shift ;;
        -y|--yes)       ASSUME_YES=1; shift ;;
        -n|--dry-run)   DRY_RUN=1; shift ;;
        -h|--help)      usage; exit 0 ;;
        *)              die "不明なオプション: $1 (--help を参照)" ;;
    esac
done

[ "$FEATURES" = "all" ] && FEATURES="preview,archive,plugin"

BIN_DIR="$PREFIX/bin"
BIN_PATH="$BIN_DIR/$APP_NAME"
PLUGIN_DIR="$CONFIG_DIR/plugins"

confirm() {
    [ "$ASSUME_YES" -eq 1 ] && return 0
    [ "$DRY_RUN" -eq 1 ] && return 0
    local answer
    printf '%s [y/N]: ' "$1"
    read -r answer </dev/tty || answer=""
    case "$answer" in [yY]|[yY][eE][sS]) return 0 ;; *) return 1 ;; esac
}

# ---------------------------------------------------------------------------
# アンインストール
# ---------------------------------------------------------------------------
uninstall() {
    info "echofiler をアンインストールします"

    if [ -e "$BIN_PATH" ]; then
        run rm -f "$BIN_PATH"
        ok "削除: $BIN_PATH"
    else
        skip "バイナリが見つかりません: $BIN_PATH"
    fi

    if [ "$PURGE" -eq 1 ]; then
        for dir in "$CONFIG_DIR" "$DATA_DIR"; do
            if [ -d "$dir" ]; then
                if confirm "$dir を完全に削除しますか?"; then
                    run rm -rf "$dir"
                    ok "削除: $dir"
                else
                    skip "保持: $dir"
                fi
            else
                skip "存在しません: $dir"
            fi
        done
    else
        [ -d "$CONFIG_DIR" ] && skip "設定を保持: $CONFIG_DIR (削除するには --purge)"
        [ -d "$DATA_DIR" ]   && skip "データを保持: $DATA_DIR (削除するには --purge)"
    fi

    info "完了"
}

# ---------------------------------------------------------------------------
# インストール
# ---------------------------------------------------------------------------
build_binary() {
    command -v cargo >/dev/null 2>&1 \
        || die "cargo が見つかりません。Rustツールチェインを入れるか --no-build / --bin を使ってください"

    local args=(build --release --manifest-path "$SCRIPT_DIR/Cargo.toml")
    if [ -n "$FEATURES" ]; then
        args+=(--features "$FEATURES")
        info "ビルド中 (features: $FEATURES)"
    else
        info "ビルド中 (features: なし)"
    fi
    run cargo "${args[@]}"
}

install_binary() {
    run install -d -m 755 "$BIN_DIR"
    run install -m 755 "$BIN_SRC" "$BIN_PATH"
    ok "バイナリ: $BIN_PATH"
}

# 設定ファイルを配置する。既存ファイルは --force が無ければ触らない
install_config_file() {
    local src="$1" dest="$2"
    if [ -e "$dest" ]; then
        if [ "$FORCE" -eq 1 ]; then
            local backup="$dest.bak-$(date +%Y%m%d%H%M%S)"
            run cp -p "$dest" "$backup"
            run install -m 644 "$src" "$dest"
            ok "上書き: $dest (退避: $(basename "$backup"))"
        else
            skip "既存のためスキップ: $dest"
        fi
    else
        run install -m 644 "$src" "$dest"
        ok "作成: $dest"
    fi
}

install_configs() {
    run install -d -m 755 "$CONFIG_DIR"
    for f in "${CONFIG_FILES[@]}"; do
        [ -f "$CONFIG_SRC_DIR/$f" ] || die "既定設定が見つかりません: $CONFIG_SRC_DIR/$f"
        install_config_file "$CONFIG_SRC_DIR/$f" "$CONFIG_DIR/$f"
    done
}

install_plugins() {
    local sample="$CONFIG_SRC_DIR/example_plugin.lua"
    run install -d -m 755 "$PLUGIN_DIR"
    ok "プラグインディレクトリ: $PLUGIN_DIR"

    [ -f "$sample" ] || return 0

    # examples/ 配下はアプリの走査対象外 (トップレベルの *.lua のみ読み込まれる)
    run install -d -m 755 "$PLUGIN_DIR/examples"
    install_config_file "$sample" "$PLUGIN_DIR/examples/example_plugin.lua"

    if [ "$WITH_EXAMPLE_PLUGIN" -eq 1 ]; then
        install_config_file "$sample" "$PLUGIN_DIR/example_plugin.lua"
    fi
}

install_data_dir() {
    run install -d -m 755 "$DATA_DIR"
    ok "データディレクトリ: $DATA_DIR"
}

check_path() {
    case ":${PATH}:" in
        *":$BIN_DIR:"*) : ;;
        *)
            warn "$BIN_DIR は PATH に含まれていません。シェル設定に以下を追加してください:"
            printf '\n    export PATH="%s:$PATH"\n\n' "$BIN_DIR"
            ;;
    esac
}

do_install() {
    info "echofiler v$(app_version) をインストールします"
    printf '    バイナリ  : %s\n' "$BIN_PATH"
    printf '    設定      : %s\n' "$CONFIG_DIR"
    printf '    データ    : %s\n' "$DATA_DIR"
    [ "$DRY_RUN" -eq 1 ] && info "dry-run モード: 実際の変更は行いません"

    # アプリ側は XDG_CONFIG_HOME (未設定なら ~/.config) から設定を探すため、
    # そこ以外に置いた場合は読み込まれないことを知らせる
    local expected_config_dir="${XDG_CONFIG_HOME:-$HOME/.config}/$APP_NAME"
    if [ "$CONFIG_DIR" != "$expected_config_dir" ]; then
        warn "echofiler は $expected_config_dir から設定を読み込みます。"
        warn "$CONFIG_DIR を使うには XDG_CONFIG_HOME=$(dirname "$CONFIG_DIR") を設定してください。"
    fi

    if [ "$DO_BUILD" -eq 1 ]; then
        build_binary
        BIN_SRC="$SCRIPT_DIR/target/release/$APP_NAME"
    fi
    [ -n "$BIN_SRC" ] || BIN_SRC="$SCRIPT_DIR/target/release/$APP_NAME"

    if [ ! -f "$BIN_SRC" ]; then
        die "バイナリが見つかりません: $BIN_SRC (--no-build を外してビルドするか --bin で指定してください)"
    fi

    info "バイナリを配置"
    install_binary

    info "設定ファイルを配置"
    install_configs
    install_plugins
    install_data_dir

    check_path
    info "完了。'$APP_NAME' で起動できます"
}

case "$MODE" in
    install)   do_install ;;
    uninstall) uninstall ;;
esac
