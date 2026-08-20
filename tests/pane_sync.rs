//! o / O キー（ペイン間のディレクトリ同期）の結合テスト
//!
//! 環境変数はプロセス全体に影響するため、このファイルは1テストのみに保つ
//! (tests/ 配下は1ファイルが1バイナリとして実行される)。

use echofiler::app::action::Action;
use echofiler::core::PaneSide;
use echofiler::App;
use std::fs;

#[test]
fn o_and_shift_o_sync_directories_between_panes() {
    let base = std::env::temp_dir().join("echofiler-pane-sync-test");
    let config_home = base.join("config");
    let data_home = base.join("data");
    let left = base.join("left");
    let right = base.join("right");

    for dir in [&config_home, &data_home, &left, &right] {
        fs::create_dir_all(dir).expect("テスト用ディレクトリの作成に失敗");
    }
    std::env::set_var("XDG_CONFIG_HOME", &config_home);
    std::env::set_var("XDG_DATA_HOME", &data_home);

    let mut app = App::new().expect("App の初期化に失敗");
    app.left_pane.current_tab_mut().navigate_to(left.clone());
    app.right_pane.current_tab_mut().navigate_to(right.clone());
    app.active_pane = PaneSide::Left;

    // o: もう一方（右）のディレクトリをアクティブ（左）に反映する
    app.update(Action::SyncDirFromOther);
    assert_eq!(app.left_pane.current_tab().cwd, right);
    assert_eq!(app.right_pane.current_tab().cwd, right);

    // O: アクティブ（左）のディレクトリをもう一方（右）に反映する
    app.left_pane.current_tab_mut().navigate_to(left.clone());
    app.update(Action::SyncDirToOther);
    assert_eq!(app.left_pane.current_tab().cwd, left);
    assert_eq!(app.right_pane.current_tab().cwd, left);

    // 同じディレクトリなら何も起きず、その旨を伝える
    app.update(Action::SyncDirToOther);
    assert!(
        app.status_message.contains("already"),
        "想定外のメッセージ: {}",
        app.status_message
    );

    // アクティブペインが右のときは向きが逆になる
    app.active_pane = PaneSide::Right;
    app.right_pane.current_tab_mut().navigate_to(right.clone());
    app.update(Action::SyncDirToOther);
    assert_eq!(app.left_pane.current_tab().cwd, right);

    fs::remove_dir_all(&base).ok();
}
