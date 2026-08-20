//! 終了時ディレクトリの復元の結合テスト
//!
//! `XDG_CONFIG_HOME` / `XDG_DATA_HOME` を一時ディレクトリに向けて
//! App::new() が session.toml のディレクトリから開始することを確認する。
//! 環境変数はプロセス全体に影響するため、このファイルは1テストのみに保つ
//! (tests/ 配下は1ファイルが1バイナリとして実行される)。

use echofiler::core::{PaneSide, Session};
use echofiler::App;
use std::fs;
use std::path::PathBuf;

#[test]
fn app_starts_from_saved_directories() {
    let base = std::env::temp_dir().join("echofiler-session-test");
    let config_home = base.join("config");
    let data_home = base.join("data");
    let left = base.join("left");
    let right = base.join("right");

    for dir in [&config_home, &data_home, &left, &right] {
        fs::create_dir_all(dir).expect("テスト用ディレクトリの作成に失敗");
    }

    std::env::set_var("XDG_CONFIG_HOME", &config_home);
    std::env::set_var("XDG_DATA_HOME", &data_home);

    // 前回終了時の状態を書き込む
    Session::from_dirs(left.clone(), right.clone(), PaneSide::Right)
        .save()
        .expect("session.toml の保存に失敗");

    let app = App::new().expect("App の初期化に失敗");

    assert_eq!(app.left_pane.current_tab().cwd, left);
    assert_eq!(app.right_pane.current_tab().cwd, right);
    assert_eq!(app.active_pane, PaneSide::Right);

    // 保存されたディレクトリが無くなっていた場合はカレントディレクトリにフォールバックする
    fs::remove_dir_all(&right).expect("テスト用ディレクトリの削除に失敗");
    let app = App::new().expect("App の初期化に失敗");
    assert_eq!(app.left_pane.current_tab().cwd, left);
    assert_eq!(
        app.right_pane.current_tab().cwd,
        std::env::current_dir().unwrap()
    );

    // session.toml が無い場合も起動できる
    fs::remove_file(data_home.join("echofiler/session.toml")).expect("session.toml の削除に失敗");
    let app = App::new().expect("App の初期化に失敗");
    let cwd: PathBuf = std::env::current_dir().unwrap();
    assert_eq!(app.left_pane.current_tab().cwd, cwd);
    assert_eq!(app.right_pane.current_tab().cwd, cwd);
    assert_eq!(app.active_pane, PaneSide::Left);

    fs::remove_dir_all(&base).ok();
}
