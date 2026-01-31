# echofiler

Linux CLI用の二画面ファイラー。TOMLファイルによる高度なカスタマイズ性を持つファイルマネージャーです。

## 特徴

- 二画面レイアウト
- Vimライクなキーバインド
- TOMLベースの設定システム（ホットリロード対応）
- 高速なファイル一覧表示
- 画像プレビュー（PNG, JPEG, GIF等）
- アーカイブ操作（ZIP, TAR, TAR.GZ）
- Lua/Luauプラグインシステム
- ファイル監視（自動リロード）
- ブックマーク機能
- 検索・フィルター機能
- タブ機能

## 技術スタック

- **TUI**: ratatui + crossterm
- **非同期**: tokio
- **ファイルシステム**: walkdir, ignore, notify
- **設定**: toml + serde
- **言語**: Rust

## ビルド方法

```bash
# 基本ビルド
cargo build --release

# 画像プレビュー機能を有効化してビルド
cargo build --release --features preview

# すべての機能を有効化してビルド
cargo build --release --features preview,archive,plugin
```

## インストール

### Makeを使用したインストール（推奨）

```bash
# すべての機能を有効化してインストール
make install

# インストール先を変更する場合
make install PREFIX=/usr/local

# アンインストール
make uninstall
```

デフォルトでは、以下にインストールされます：
- バイナリ: `~/.local/bin/echofiler`
- 設定ファイル: `~/.config/echofiler/`

`~/.local/bin` がPATHに含まれていない場合は、以下を `~/.bashrc` または `~/.zshrc` に追加してください：

```bash
export PATH="$HOME/.local/bin:$PATH"
```

### Cargoを使用したインストール

```bash
# すべての機能を有効化してインストール
cargo install --path . --features archive,preview,plugin

# インストール後、設定ファイルを手動でコピー
mkdir -p ~/.config/echofiler
cp config/default/*.toml ~/.config/echofiler/
```

### install.shを使用したインストール（Makeがない環境）

```bash
# デフォルトインストール
./install.sh

# インストール先を変更
PREFIX=/usr/local ./install.sh

# デバッグビルドをインストール
BUILD_TYPE=debug ./install.sh
```

## 実行方法

```bash
# インストール済みの場合
echofiler

# または開発時
cargo run --features archive,preview,plugin
```

## 設定

設定ファイルは `~/.config/echofiler/echofiler.toml` に配置します。

設定ファイルが存在しない場合、デフォルト設定が使用されます。

### アイコン表示の設定

echofilerは3種類のアイコンスタイルをサポートしています：

- **nerd-fonts**: Nerd Fonts（1文字幅、推奨）
- **emoji**: 絵文字（2文字幅、表示がずれる可能性あり）
- **ascii**: ASCII文字（1文字幅、シンプル）

#### Nerd Fontsのインストール（推奨）

アイコンを正しく表示するには、ターミナルにNerd Fontsをインストールする必要があります。

```bash
# Arch Linux
sudo pacman -S ttf-nerd-fonts-symbols

# Ubuntu/Debian
# https://github.com/ryanoasis/nerd-fonts/releases から手動ダウンロード

# macOS (Homebrew)
brew tap homebrew/cask-fonts
brew install font-hack-nerd-font

# または、任意のNerd Fontsフォント
brew install font-fira-code-nerd-font
```

インストール後、ターミナルの設定でNerd Fontsを選択してください。

### 設定例

```toml
[general]
show_hidden = false          # 隠しファイルを表示するか
confirm_delete = true        # 削除時に確認するか
confirm_overwrite = true     # 上書き時に確認するか
show_icons = true            # ファイル/ディレクトリアイコンを表示するか
icon_style = "nerd-fonts"    # アイコンスタイル: "nerd-fonts" | "emoji" | "ascii"
icon_spacing = 1             # アイコン後のスペース数（0-9）

[layout]
style = "dual"               # レイアウトスタイル
ratio = [1, 1]               # 左右ペインの比率

[sort]
method = "natural"           # ソート方法
directories_first = true     # ディレクトリを最初に表示
reverse = false              # 逆順ソート

[preview]
max_size = "10MB"            # プレビュー最大サイズ
syntax_highlight = true      # シンタックスハイライト
image_protocol = "auto"      # 画像プロトコル

[log]
level = "info"               # ログレベル
file = "~/.local/share/echofiler/echofiler.log"
```

デフォルト設定ファイルは `config/default/echofiler.toml` を参照してください。

## 基本操作

### ナビゲーション

- `j` / `↓`: カーソル下移動
- `k` / `↑`: カーソル上移動
- `Enter`: ディレクトリに入る / ファイルを開く
- `Backspace`: 親ディレクトリへ戻る

### ペイン操作

- `Tab`: アクティブペイン切り替え
- `←` / `Ctrl+h`: 左ペインにフォーカス
- `→` / `Ctrl+l`: 右ペインにフォーカス

### ファイル選択

- `Space`: ファイル選択切替（複数選択可能）
- `v`: Visual mode（移動で連続選択）
- `Esc`: Normal modeへ戻る

### ファイル操作

- `y`: コピー（選択ファイルまたはカーソル下のファイル）
- `d`: 切り取り（選択ファイルまたはカーソル下のファイル）
- `p`: 貼り付け（逆ペインに、上書き時は確認ダイアログ表示）
- `D`: 削除（選択ファイルまたはカーソル下のファイル、確認ダイアログ表示）
- `C`: 別のペインに直接コピー（選択ファイルまたはカーソル下のファイル、上書き時は確認ダイアログ表示）
- `M`: 別のペインに直接移動（選択ファイルまたはカーソル下のファイル、上書き時は確認ダイアログ表示）
- `R`: リネーム（コマンド入力モード）
- `a`: 新規作成（コマンド入力モード、末尾に/でディレクトリ）
- `e`: アーカイブ展開（archiveフィーチャー有効時、確認ダイアログ表示）
- `z`: ZIPに圧縮（選択ファイル、archiveフィーチャー有効時）

### ファイルを開く

- `Enter`: ディレクトリに入る / ファイルを開く
- `o`: ファイルを開く（デフォルトアプリケーション）
- `E`: ファイルをエディタで開く（$EDITOR または設定ファイル）
- `w`: ファイルをページャで開く（$PAGER または設定ファイル）
- `X`: 実行可能ファイルを実行

### 検索・フィルター

- `/`: 検索モード（インクリメンタル検索）
- `n`: 次の検索結果へ
- `N`: 前の検索結果へ
- `f`: フィルターモード（ファイル名で絞り込み）

### タブ操作

- `Ctrl+t`: 新規タブ作成
- `Ctrl+w`: タブを閉じる
- `h`: 前のタブへ
- `l`: 次のタブへ
- `[` / `]`: 前/次のタブへ（代替）

### プレビュー

- `P`: プレビューモード切替（フォーカスペインの逆側にファイル内容を表示）
  - テキストファイル: ファイル内容をそのまま表示
  - 画像ファイル: Unicodeブロック文字で画像を表示（PNG, JPEG, GIF, BMP, WebP等に対応）
  - アーカイブファイル: アーカイブ内容を一覧表示（archiveフィーチャー有効時）

### ソート

- `s`: ソート方法切替（Name → Size → Modified → Extension）
- `S`: ソート順反転（昇順/降順）

### サイドバー

- `i`: サイドバー表示切替（ファイル/ディレクトリの詳細情報を表示）

### ブックマーク（プレフィックスキー: g）

echofilerは`g`キーをブックマーク操作のプレフィックスキーとして使用します。プレフィックスキーとその後のキーマップは`keymap.toml`でカスタマイズ可能です。

#### ブックマーク操作

- `g`: ブックマークプレフィックスモードに入る
  - `g b`: ブックマーク追加（名前とキーを入力）
  - `g B`: 全ブックマーク一覧を表示
  - `g m`: キー付きブックマーク一覧を表示
  - `g + 任意のキー`: そのキーに登録されたブックマークへジャンプ
  - `Esc`: キャンセル

#### キーマップのカスタマイズ

`~/.config/echofiler/keymap.toml`でプレフィックスキーとその後のキーをカスタマイズできます：

```toml
[normal]
# ブックマークプレフィックスキー（特殊ワード: <bm_prefix>）
"g" = "<bm_prefix>"
"<bm_prefix> b" = "add_bookmark"
"<bm_prefix> B" = "show_bookmarks"
"<bm_prefix> m" = "show_bookmark_select"

# 例: Spaceキーをプレフィックスに変更
# "Space" = "<bm_prefix>"
```

特殊ワード（`<...>`形式）を使うことで、プレフィックスキーとその後のキーを柔軟に定義できます。

#### ブックマーク追加（g b）

現在のディレクトリをブックマークに追加します：

- 入力形式1: `名前 キー` （例: `Home h`）
  - 名前とショートカットキーの両方を登録
  - 以降、`g h`で即座にジャンプ可能
- 入力形式2: `名前` （例: `Documents`）
  - 名前のみを登録（ショートカットキーなし）
  - `g B`の一覧から選択して移動

#### ブックマーク一覧

**全ブックマーク一覧（g B）:**
- `j/k`: カーソル移動
- `Enter`: カーソル位置のブックマークへジャンプ
- `キー`: そのキーに登録されたブックマークへ直接ジャンプ
- `d`: ブックマーク削除
- `q/Esc`: 閉じる

**キー付きブックマーク一覧（g m）:**
- キーが登録されたブックマークのみを表示
- 任意のキーを押すとそのブックマークへジャンプ
- `Esc`: キャンセル

#### 使用例

```bash
# ブックマーク追加（名前 "Home" とキー "h" で登録）
g → b → 入力: "Home h" → Enter

# キー "h" でジャンプ
g → h

# 全ブックマーク一覧を表示
g → B

# キー付きブックマーク一覧を表示
g → m → h  # キー "h" のブックマークへジャンプ

# 名前のみ登録（ショートカットキーなし）
g → b → 入力: "Projects" → Enter
```

### 確認ダイアログ

ファイル削除や上書き操作時に確認ダイアログが表示されます：

- `y`: 実行
- `n` / `Esc`: キャンセル

確認ダイアログは設定ファイルで無効化できます：

```toml
[general]
confirm_delete = false     # 削除時の確認を無効化
confirm_overwrite = false  # 上書き時の確認を無効化
```

### 設定編集

アプリケーション内から設定ファイルを直接編集できます：

- `:config`: echofiler.toml を編集（一般設定）
- `:keymap`: keymap.toml を編集（キーバインド）
- `:theme`: theme.toml を編集（カラーテーマ）
- `:opener`: opener.toml を編集（ファイルオープナー）

`:` を押してコマンドモードに入り、コマンドを入力してEnterで実行します。

**コマンド補完**: `Tab`キーで入力中のコマンドを補完できます。
- 候補が1つの場合: 自動補完
- 候補が複数の場合: ステータスラインに候補を表示

設定ファイルが存在しない場合は、デフォルト設定から自動的に作成されます。

### その他

- `.`: 隠しファイル表示切替
- `r`: 表示更新（ファイル監視機能により自動更新）
- `?`: ヘルプ表示（ショートカットキー一覧）
- `q`: 終了

### キーマップカスタマイズ

`~/.config/echofiler/keymap.toml` でキーバインドをカスタマイズできます。詳細は `config/default/keymap.toml` を参照してください。

### プラグインシステム

`~/.config/echofiler/plugins/` ディレクトリにLuaスクリプト（.lua）を配置することでプラグインを追加できます。

プラグイン例（`~/.config/echofiler/plugins/example.lua`）:

```lua
-- プラグイン情報を定義
plugin = {
    name = "example",
    version = "0.1.0",
    author = "Your Name",
    description = "Example plugin"
}

-- 起動時フック
function on_startup()
    echofiler.log.info("Plugin loaded!")
    echofiler.ui.notify("Hello!")
end

-- フックを登録
echofiler.hooks.register("on_startup", "on_startup")
```

利用可能なAPI:
- `echofiler.log.info(message)`: ログ出力
- `echofiler.log.warn(message)`: 警告出力
- `echofiler.log.error(message)`: エラー出力
- `echofiler.fs.exists(path)`: ファイル存在確認
- `echofiler.fs.is_dir(path)`: ディレクトリか確認
- `echofiler.ui.notify(message)`: 通知表示

利用可能なフック:
- `on_startup`: 起動時
- `on_exit`: 終了時
- `on_file_select`: ファイル選択時
- `on_dir_change`: ディレクトリ変更時
- `before_file_op`: ファイル操作前
- `after_file_op`: ファイル操作後

詳細は `config/default/example_plugin.lua` を参照してください。

### ファイルオープナー設定

`~/.config/echofiler/opener.toml` でファイルを開くアプリケーションをカスタマイズできます。

```toml
# デフォルトのオープナー
default = "xdg-open"

# エディタとページャ
editor = "vi"    # 環境変数$EDITORが優先される
pager = "less"   # 環境変数$PAGERが優先される

# 拡張子ごとのカスタムオープナー
[extension]
txt = "vi"
md = "vi"
png = "xdg-open"
pdf = "xdg-open"
mp4 = "mpv"
```

詳細は `config/default/opener.toml` を参照してください。

## 開発ロードマップ

### Phase 1: 基盤構築（MVP）✓ 完了

- プロジェクト初期化
- 基本的なTUIフレームワーク
- イベントループ実装
- 二画面ファイル一覧表示
- 基本的なナビゲーション（h/j/k/l）

### Phase 2: 設定システム ✓ 完了

- TOMLベース設定ファイル
- デフォルト設定の組み込み
- ~/.config/echofiler/からの設定読み込み
- 一般設定の適用（show_hidden等）
- レイアウト設定の適用（ペイン比率等）

### Phase 3: ファイル操作 ✓ 完了

- ファイル選択（Space, Visual mode）
- コピー/移動/削除操作
- 内部クリップボード
- 選択状態の視覚的表示
- ディレクトリの再帰的コピー・削除

### Phase 4: 入力プロンプトと設定拡張 ✓ 完了

- Command mode実装
- リネーム機能（R）
- 新規作成機能（a）
- コマンドライン入力UI
- テーマ設定パース実装

### Phase 5: 高度な機能 ✓ 完了

- 検索機能（/、インクリメンタル検索）
- 検索ナビゲーション（n/N）
- フィルター機能（f、ファイル名絞り込み）
- タブ機能（新規作成、閉じる、切り替え）
- タブ一覧表示

### Phase 6: プレビュー・テーマ・ソート ✓ 完了

- テーマ設定の適用（UI全体へのテーマ適用）
- テキストプレビュー（Pキーで切替）
- ソート機能拡張（動的ソート変更）
- ファイルタイプ別カラー表示（ディレクトリ、実行ファイル、シンボリックリンク等）

### Phase 7: キーマップ・サイドバー ✓ 完了

- キーマップ設定の完全実装（TOMLからのキーバインド読み込み）
- サイドバー機能（ファイル/ディレクトリの詳細情報表示）
- プレビューモード改善（フォーカスペインの逆側に表示）

### Phase 8: ファイル監視・ブックマーク ✓ 完了

- ファイル監視（notifyクレートによる自動リロード）
- ブックマーク機能（ディレクトリのブックマーク管理）

### Phase 9: 画像プレビュー・プラグイン ✓ 完了

- 画像プレビュー ✓ 完了
  - PNG, JPEG, GIF, BMP, WebP等に対応
  - Unicodeブロック文字によるフルカラー表示
- Luaプラグインシステム ✓ 完了
  - Lua/Luauによるプラグインシステム
  - プラグインAPI（ログ、ファイルシステム、UI）
  - フックシステム（起動時、終了時、ファイル操作等）
  - サンプルプラグイン付属

### Phase 10: アーカイブ操作・設定ホットリロード ✓ 完了

- アーカイブ操作 ✓ 完了
  - ZIP, TAR, TAR.GZ の展開・圧縮
  - アーカイブファイルの内容プレビュー
  - `e`キーで展開、`z`キーで圧縮
- 設定ホットリロード ✓ 完了
  - 設定ファイルの変更を自動検出
  - echofiler.toml, theme.toml, keymap.toml の監視
  - 変更時に自動で設定を再読み込み

## ライセンス

MIT

## 貢献

Issue、Pull Requestを歓迎します。
