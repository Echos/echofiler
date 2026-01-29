use std::path::Path;
use crate::config::general::IconStyle;

/// ファイル/ディレクトリのアイコンを取得
pub fn get_icon(path: &Path, is_dir: bool, is_symlink: bool, style: IconStyle) -> &'static str {
    match style {
        IconStyle::NerdFonts => get_nerd_font_icon(path, is_dir, is_symlink),
        IconStyle::Emoji => get_emoji_icon(path, is_dir, is_symlink),
        IconStyle::Ascii => get_ascii_icon(path, is_dir, is_symlink),
    }
}

// ==================== Nerd Fonts アイコン ====================

fn get_nerd_font_icon(path: &Path, is_dir: bool, is_symlink: bool) -> &'static str {
    if is_symlink {
        return "\u{f481}"; // nf-oct-file_symlink_file
    }

    if is_dir {
        return get_nerd_font_dir_icon(path);
    }

    // ファイル名ベースのアイコン（優先）
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        if let Some(icon) = get_nerd_font_filename_icon(name) {
            return icon;
        }
    }

    // 拡張子ベースのアイコン
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if let Some(icon) = get_nerd_font_extension_icon(ext) {
            return icon;
        }
    }

    // デフォルトファイルアイコン
    "\u{f15b}" // nf-fa-file_o
}

fn get_nerd_font_dir_icon(path: &Path) -> &'static str {
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        match name {
            ".git" => "\u{f1d3}", // nf-dev-git
            ".github" => "\u{f408}", // nf-oct-mark_github
            ".config" => "\u{e5fc}", // nf-dev-config
            "Downloads" => "\u{f498}", // nf-oct-download
            "Documents" => "\u{f02d}", // nf-fa-book
            "Pictures" | "Images" => "\u{f03e}", // nf-fa-picture_o
            "Music" => "\u{f001}", // nf-fa-music
            "Videos" => "\u{f03d}", // nf-fa-video
            "Desktop" => "\u{f108}", // nf-fa-desktop
            "src" | "source" => "\u{e796}", // nf-dev-code
            "test" | "tests" => "\u{f0668}", // nf-mdi-test_tube
            "docs" => "\u{f02d}", // nf-fa-book
            "node_modules" => "\u{e718}", // nf-dev-nodejs_small
            ".vscode" => "\u{e70c}", // nf-dev-visualstudio
            ".idea" => "\u{e7b5}", // nf-dev-intellij
            "target" => "\u{f140}", // nf-fa-folder_open_o
            "build" => "\u{f0c7}", // nf-fa-build
            "dist" => "\u{f413}", // nf-oct-package
            _ => "\u{f115}", // nf-dev-folder
        }
    } else {
        "\u{f115}" // nf-dev-folder
    }
}

fn get_nerd_font_extension_icon(ext: &str) -> Option<&'static str> {
    let ext_lower = ext.to_lowercase();
    match ext_lower.as_str() {
        // Programming languages
        "rs" => Some("\u{e7a8}"), // nf-dev-rust
        "py" => Some("\u{e73c}"), // nf-dev-python
        "js" | "mjs" | "cjs" => Some("\u{e74e}"), // nf-dev-javascript
        "ts" | "tsx" => Some("\u{e628}"), // nf-seti-typescript
        "jsx" => Some("\u{e7ba}"), // nf-dev-react
        "go" => Some("\u{e626}"), // nf-seti-go
        "c" | "h" => Some("\u{e61e}"), // nf-seti-c
        "cpp" | "cc" | "cxx" | "hpp" => Some("\u{e61d}"), // nf-seti-cpp
        "java" => Some("\u{e738}"), // nf-dev-java
        "rb" => Some("\u{e739}"), // nf-dev-ruby
        "php" => Some("\u{e73d}"), // nf-dev-php
        "swift" => Some("\u{e755}"), // nf-dev-swift
        "kt" | "kts" => Some("\u{e634}"), // nf-seti-kotlin
        "lua" => Some("\u{e620}"), // nf-seti-lua
        "vim" => Some("\u{e62b}"), // nf-seti-vim
        "sh" | "bash" | "zsh" | "fish" => Some("\u{f489}"), // nf-oct-terminal
        "r" => Some("\u{f25d}"), // nf-fae-r
        "dart" => Some("\u{e798}"), // nf-dev-dart
        "scala" => Some("\u{e737}"), // nf-dev-scala
        "cs" => Some("\u{f031b}"), // nf-mdi-language_csharp
        "ex" | "exs" => Some("\u{e62d}"), // nf-seti-elixir
        "erl" | "hrl" => Some("\u{e7b1}"), // nf-dev-erlang

        // Web
        "html" | "htm" => Some("\u{e736}"), // nf-dev-html5
        "css" | "scss" | "sass" | "less" => Some("\u{e749}"), // nf-dev-css3
        "json" => Some("\u{e60b}"), // nf-seti-json
        "xml" => Some("\u{f05c0}"), // nf-mdi-xml
        "yaml" | "yml" => Some("\u{f481}"), // nf-oct-file_code
        "toml" => Some("\u{e615}"), // nf-seti-config
        "ini" | "conf" | "config" => Some("\u{e615}"), // nf-seti-config

        // Documents
        "md" | "markdown" => Some("\u{e609}"), // nf-seti-markdown
        "txt" => Some("\u{f15c}"), // nf-fa-file_text_o
        "pdf" => Some("\u{f1c1}"), // nf-fa-file_pdf_o
        "doc" | "docx" => Some("\u{f1c2}"), // nf-fa-file_word_o
        "xls" | "xlsx" => Some("\u{f1c3}"), // nf-fa-file_excel_o
        "ppt" | "pptx" => Some("\u{f1c4}"), // nf-fa-file_powerpoint_o
        "odt" | "ods" | "odp" => Some("\u{f15c}"), // nf-fa-file_text_o
        "tex" => Some("\u{e600}"), // nf-seti-tex
        "rtf" => Some("\u{f15c}"), // nf-fa-file_text_o

        // Images
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "ico" => Some("\u{f1c5}"), // nf-fa-file_image_o
        "svg" => Some("\u{e64d}"), // nf-seti-svg
        "psd" => Some("\u{e7b8}"), // nf-dev-photoshop
        "ai" => Some("\u{e7b4}"), // nf-dev-illustrator

        // Audio
        "mp3" | "wav" | "ogg" | "flac" | "aac" | "m4a" | "wma" => Some("\u{f1c7}"), // nf-fa-file_audio_o

        // Video
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" => Some("\u{f1c8}"), // nf-fa-file_video_o

        // Archives
        "zip" | "rar" | "7z" => Some("\u{f410}"), // nf-oct-file_zip
        "tar" | "gz" | "bz2" | "xz" | "tgz" => Some("\u{f410}"), // nf-oct-file_zip
        "deb" | "rpm" | "apk" | "dmg" => Some("\u{f487}"), // nf-oct-package

        // Executables
        "exe" | "dll" | "so" | "dylib" => Some("\u{f17a}"), // nf-fa-windows
        "app" => Some("\u{f179}"), // nf-fa-apple
        "bin" => Some("\u{f489}"), // nf-oct-terminal

        // Database
        "db" | "sqlite" | "sqlite3" => Some("\u{f1c0}"), // nf-fa-database
        "sql" => Some("\u{f1c0}"), // nf-fa-database

        // Lock files
        "lock" => Some("\u{f023}"), // nf-fa-lock

        // Fonts
        "ttf" | "otf" | "woff" | "woff2" | "eot" => Some("\u{f031}"), // nf-fa-font

        // Other
        "iso" | "img" => Some("\u{f1c0}"), // nf-fa-disk
        "log" => Some("\u{f18d}"), // nf-fa-file_text
        "csv" => Some("\u{f1c3}"), // nf-fa-file_excel_o

        _ => None,
    }
}

fn get_nerd_font_filename_icon(name: &str) -> Option<&'static str> {
    let name_lower = name.to_lowercase();
    match name_lower.as_str() {
        "readme" | "readme.md" | "readme.txt" => Some("\u{f48a}"), // nf-oct-book
        "license" | "license.md" | "license.txt" | "copying" => Some("\u{e60a}"), // nf-seti-license
        "makefile" | "cmake" | "cmakelists.txt" => Some("\u{e615}"), // nf-seti-config
        "dockerfile" | ".dockerignore" => Some("\u{f308}"), // nf-linux-docker
        "docker-compose.yml" | "docker-compose.yaml" => Some("\u{f308}"), // nf-linux-docker
        "cargo.toml" | "cargo.lock" => Some("\u{e7a8}"), // nf-dev-rust
        "package.json" | "package-lock.json" => Some("\u{e718}"), // nf-dev-nodejs_small
        "go.mod" | "go.sum" => Some("\u{e626}"), // nf-seti-go
        "gemfile" | "gemfile.lock" => Some("\u{e739}"), // nf-dev-ruby
        ".env" | ".env.local" | ".env.example" => Some("\u{f023}"), // nf-fa-lock
        ".gitignore" | ".gitattributes" | ".gitmodules" => Some("\u{f1d3}"), // nf-dev-git
        ".editorconfig" => Some("\u{e615}"), // nf-seti-config
        ".bashrc" | ".zshrc" | ".vimrc" | ".tmux.conf" => Some("\u{e615}"), // nf-seti-config
        "gruntfile.js" | "gulpfile.js" | "webpack.config.js" => Some("\u{e74e}"), // nf-dev-javascript
        "tsconfig.json" => Some("\u{e628}"), // nf-seti-typescript
        ".pylintrc" | ".flake8" | "pyproject.toml" => Some("\u{e73c}"), // nf-dev-python
        _ => None,
    }
}

// ==================== Emoji アイコン ====================

fn get_emoji_icon(path: &Path, is_dir: bool, is_symlink: bool) -> &'static str {
    if is_symlink {
        return "🔗";
    }

    if is_dir {
        return get_emoji_dir_icon(path);
    }

    // ファイル名ベースのアイコン（優先）
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        if let Some(icon) = get_emoji_filename_icon(name) {
            return icon;
        }
    }

    // 拡張子ベースのアイコン
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if let Some(icon) = get_emoji_extension_icon(ext) {
            return icon;
        }
    }

    // デフォルトファイルアイコン
    "📄"
}

fn get_emoji_dir_icon(path: &Path) -> &'static str {
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        match name {
            ".git" => "",
            ".config" => "⚙️",
            "Downloads" => "📥",
            "Documents" => "📚",
            "Pictures" | "Images" => "🖼️",
            "Music" => "🎵",
            "Videos" => "🎬",
            "Desktop" => "🖥️",
            "src" | "source" => "📦",
            "test" | "tests" => "🧪",
            "docs" => "📖",
            "node_modules" => "📦",
            ".vscode" | ".idea" => "💡",
            _ => "📁",
        }
    } else {
        "📁"
    }
}

fn get_emoji_extension_icon(ext: &str) -> Option<&'static str> {
    let ext_lower = ext.to_lowercase();
    match ext_lower.as_str() {
        // Programming languages
        "rs" => Some("🦀"),
        "py" => Some("🐍"),
        "js" | "mjs" | "cjs" => Some("📜"),
        "ts" | "tsx" => Some("💙"),
        "go" => Some("🐹"),
        "c" | "h" => Some("©️"),
        "cpp" | "cc" | "cxx" | "hpp" => Some("⚙️"),
        "java" => Some("☕"),
        "rb" => Some("💎"),
        "php" => Some("🐘"),
        "swift" => Some("🦅"),
        "kt" | "kts" => Some("🅺"),
        "lua" => Some("🌙"),
        "vim" => Some("🟢"),
        "sh" | "bash" | "zsh" | "fish" => Some("🐚"),

        // Web
        "html" | "htm" => Some("🌐"),
        "css" | "scss" | "sass" | "less" => Some("🎨"),
        "json" => Some("{}"),
        "xml" => Some("📋"),
        "yaml" | "yml" => Some("📋"),
        "toml" => Some("⚙️"),
        "ini" | "conf" | "config" => Some("⚙️"),

        // Documents
        "md" | "markdown" => Some("📝"),
        "txt" => Some("📄"),
        "pdf" => Some("📕"),
        "doc" | "docx" => Some("📘"),
        "xls" | "xlsx" => Some("📊"),
        "ppt" | "pptx" => Some("📽️"),
        "odt" | "ods" | "odp" => Some("📓"),

        // Images
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg" | "ico" => Some("🖼️"),

        // Audio
        "mp3" | "wav" | "ogg" | "flac" | "aac" | "m4a" => Some("🎵"),

        // Video
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" => Some("🎬"),

        // Archives
        "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" => Some("📦"),

        // Executables
        "exe" | "dll" | "so" | "dylib" | "app" => Some("⚡"),
        "deb" | "rpm" | "apk" | "dmg" => Some("📦"),

        // Git
        "git" | "gitignore" | "gitattributes" | "gitmodules" => Some(""),

        // Lock files
        "lock" => Some("🔒"),

        // Database
        "db" | "sqlite" | "sqlite3" => Some("🗄️"),
        "sql" => Some("🗃️"),

        _ => None,
    }
}

fn get_emoji_filename_icon(name: &str) -> Option<&'static str> {
    let name_lower = name.to_lowercase();
    match name_lower.as_str() {
        "readme" | "readme.md" | "readme.txt" => Some("📖"),
        "license" | "license.md" | "license.txt" => Some("📜"),
        "makefile" | "cmake" | "cmakelists.txt" => Some("⚙️"),
        "dockerfile" | "docker-compose.yml" | "docker-compose.yaml" => Some("🐳"),
        "cargo.toml" | "cargo.lock" => Some("📦"),
        "package.json" | "package-lock.json" => Some("📦"),
        "go.mod" | "go.sum" => Some("🐹"),
        ".env" | ".env.local" | ".env.example" => Some("🔐"),
        ".gitignore" | ".gitattributes" | ".gitmodules" => Some(""),
        ".editorconfig" => Some("⚙️"),
        ".bashrc" | ".zshrc" | ".vimrc" => Some("⚙️"),
        _ => None,
    }
}

// ==================== ASCII アイコン ====================

fn get_ascii_icon(path: &Path, is_dir: bool, is_symlink: bool) -> &'static str {
    if is_symlink {
        return "@";
    }

    if is_dir {
        return get_ascii_dir_icon(path);
    }

    // 拡張子ベースのアイコン
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if let Some(icon) = get_ascii_extension_icon(ext) {
            return icon;
        }
    }

    // デフォルトファイルアイコン
    "f"
}

fn get_ascii_dir_icon(_path: &Path) -> &'static str {
    "d"
}

fn get_ascii_extension_icon(ext: &str) -> Option<&'static str> {
    let ext_lower = ext.to_lowercase();
    match ext_lower.as_str() {
        // Programming languages
        "rs" | "py" | "js" | "ts" | "go" | "c" | "cpp" | "java" | "rb" | "php" => Some("*"),

        // Documents
        "md" | "txt" | "pdf" | "doc" | "docx" => Some("t"),

        // Images
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg" => Some("i"),

        // Audio/Video
        "mp3" | "wav" | "ogg" | "mp4" | "mkv" | "avi" => Some("m"),

        // Archives
        "zip" | "tar" | "gz" | "bz2" | "7z" | "rar" => Some("z"),

        // Executables
        "exe" | "dll" | "so" | "dylib" | "app" => Some("x"),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nerd_fonts_directory() {
        let path = Path::new("/home/user/Documents");
        let icon = get_icon(path, true, false, IconStyle::NerdFonts);
        assert_eq!(icon, "\u{f02d}"); // nf-fa-book

        let path = Path::new("/home/user/.git");
        let icon = get_icon(path, true, false, IconStyle::NerdFonts);
        assert_eq!(icon, "\u{f1d3}"); // nf-dev-git
    }

    #[test]
    fn test_nerd_fonts_file_extension() {
        let path = Path::new("/tmp/test.rs");
        let icon = get_icon(path, false, false, IconStyle::NerdFonts);
        assert_eq!(icon, "\u{e7a8}"); // nf-dev-rust

        let path = Path::new("/tmp/test.py");
        let icon = get_icon(path, false, false, IconStyle::NerdFonts);
        assert_eq!(icon, "\u{e73c}"); // nf-dev-python

        let path = Path::new("/tmp/test.js");
        let icon = get_icon(path, false, false, IconStyle::NerdFonts);
        assert_eq!(icon, "\u{e74e}"); // nf-dev-javascript
    }

    #[test]
    fn test_nerd_fonts_symlink() {
        let path = Path::new("/tmp/link");
        let icon = get_icon(path, false, true, IconStyle::NerdFonts);
        assert_eq!(icon, "\u{f481}"); // nf-oct-file_symlink_file
    }

    #[test]
    fn test_emoji_directory() {
        let path = Path::new("/home/user/Downloads");
        let icon = get_icon(path, true, false, IconStyle::Emoji);
        assert_eq!(icon, "📥");
    }

    #[test]
    fn test_emoji_file_extension() {
        let path = Path::new("/tmp/test.rs");
        let icon = get_icon(path, false, false, IconStyle::Emoji);
        assert_eq!(icon, "🦀");

        let path = Path::new("/tmp/test.py");
        let icon = get_icon(path, false, false, IconStyle::Emoji);
        assert_eq!(icon, "🐍");
    }

    #[test]
    fn test_emoji_symlink() {
        let path = Path::new("/tmp/link");
        let icon = get_icon(path, false, true, IconStyle::Emoji);
        assert_eq!(icon, "🔗");
    }

    #[test]
    fn test_ascii_icons() {
        let path = Path::new("/tmp/dir");
        let icon = get_icon(path, true, false, IconStyle::Ascii);
        assert_eq!(icon, "d");

        let path = Path::new("/tmp/file.txt");
        let icon = get_icon(path, false, false, IconStyle::Ascii);
        assert_eq!(icon, "t");

        let path = Path::new("/tmp/link");
        let icon = get_icon(path, false, true, IconStyle::Ascii);
        assert_eq!(icon, "@");
    }

    #[test]
    fn test_default_file_icon() {
        let path = Path::new("/tmp/unknown.xyz");

        let icon = get_icon(path, false, false, IconStyle::NerdFonts);
        assert_eq!(icon, "\u{f15b}"); // nf-fa-file_o

        let icon = get_icon(path, false, false, IconStyle::Emoji);
        assert_eq!(icon, "📄");

        let icon = get_icon(path, false, false, IconStyle::Ascii);
        assert_eq!(icon, "f");
    }

    #[test]
    fn test_special_filenames() {
        // ファイル名ベースが優先される
        let path = Path::new("/tmp/README.md");
        let icon = get_icon(path, false, false, IconStyle::NerdFonts);
        assert_eq!(icon, "\u{f48a}"); // nf-oct-book (ファイル名優先)

        let path = Path::new("/tmp/Dockerfile");
        let icon = get_icon(path, false, false, IconStyle::NerdFonts);
        assert_eq!(icon, "\u{f308}"); // nf-linux-docker

        let path = Path::new("/tmp/Cargo.toml");
        let icon = get_icon(path, false, false, IconStyle::NerdFonts);
        assert_eq!(icon, "\u{e7a8}"); // nf-dev-rust (ファイル名優先)

        // 他のtomlファイルは拡張子マッチ
        let path = Path::new("/tmp/config.toml");
        let icon = get_icon(path, false, false, IconStyle::NerdFonts);
        assert_eq!(icon, "\u{e615}"); // nf-seti-config
    }

    #[test]
    fn test_archive_files() {
        let path = Path::new("/tmp/archive.zip");
        let icon = get_icon(path, false, false, IconStyle::NerdFonts);
        assert_eq!(icon, "\u{f410}"); // nf-oct-file_zip

        let path = Path::new("/tmp/archive.tar.gz");
        let icon = get_icon(path, false, false, IconStyle::NerdFonts);
        assert_eq!(icon, "\u{f410}"); // nf-oct-file_zip
    }

    #[test]
    fn test_image_files() {
        let path = Path::new("/tmp/image.png");
        let icon = get_icon(path, false, false, IconStyle::NerdFonts);
        assert_eq!(icon, "\u{f1c5}"); // nf-fa-file_image_o

        let path = Path::new("/tmp/image.svg");
        let icon = get_icon(path, false, false, IconStyle::NerdFonts);
        assert_eq!(icon, "\u{e64d}"); // nf-seti-svg
    }

    #[test]
    fn test_case_insensitive_extensions() {
        let path_lower = Path::new("/tmp/test.rs");
        let path_upper = Path::new("/tmp/test.RS");

        let icon_lower = get_icon(path_lower, false, false, IconStyle::NerdFonts);
        let icon_upper = get_icon(path_upper, false, false, IconStyle::NerdFonts);

        assert_eq!(icon_lower, icon_upper);
    }
}
