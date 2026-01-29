use mlua::{Lua, Result, Table};

/// プラグインAPIをLuaに登録
pub fn register_api(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    // echofilerテーブルを作成
    let echofiler = lua.create_table()?;

    // ログ関数
    register_log_api(lua, &echofiler)?;

    // ファイルシステム関数（将来的に実装）
    register_fs_api(lua, &echofiler)?;

    // UI関数（将来的に実装）
    register_ui_api(lua, &echofiler)?;

    // グローバルに登録
    globals.set("echofiler", echofiler)?;

    Ok(())
}

/// ログAPI
fn register_log_api(lua: &Lua, echofiler: &Table) -> Result<()> {
    let log = lua.create_table()?;

    // echofiler.log.info(message)
    let info_fn = lua.create_function(|_, message: String| {
        println!("[Plugin] {}", message);
        Ok(())
    })?;
    log.set("info", info_fn)?;

    // echofiler.log.warn(message)
    let warn_fn = lua.create_function(|_, message: String| {
        eprintln!("[Plugin Warning] {}", message);
        Ok(())
    })?;
    log.set("warn", warn_fn)?;

    // echofiler.log.error(message)
    let error_fn = lua.create_function(|_, message: String| {
        eprintln!("[Plugin Error] {}", message);
        Ok(())
    })?;
    log.set("error", error_fn)?;

    echofiler.set("log", log)?;
    Ok(())
}

/// ファイルシステムAPI（スタブ）
fn register_fs_api(lua: &Lua, echofiler: &Table) -> Result<()> {
    let fs = lua.create_table()?;

    // echofiler.fs.list_dir(path) - ディレクトリ一覧取得
    let list_dir_fn = lua.create_function(|_, _path: String| {
        // TODO: 実装
        Ok(mlua::Value::Nil)
    })?;
    fs.set("list_dir", list_dir_fn)?;

    // echofiler.fs.exists(path) - ファイル存在確認
    let exists_fn = lua.create_function(|_, path: String| {
        let exists = std::path::Path::new(&path).exists();
        Ok(exists)
    })?;
    fs.set("exists", exists_fn)?;

    // echofiler.fs.is_dir(path) - ディレクトリか確認
    let is_dir_fn = lua.create_function(|_, path: String| {
        let is_dir = std::path::Path::new(&path).is_dir();
        Ok(is_dir)
    })?;
    fs.set("is_dir", is_dir_fn)?;

    echofiler.set("fs", fs)?;
    Ok(())
}

/// UI API（スタブ）
fn register_ui_api(lua: &Lua, echofiler: &Table) -> Result<()> {
    let ui = lua.create_table()?;

    // echofiler.ui.notify(message) - 通知表示
    let notify_fn = lua.create_function(|_, message: String| {
        println!("[Notification] {}", message);
        Ok(())
    })?;
    ui.set("notify", notify_fn)?;

    echofiler.set("ui", ui)?;
    Ok(())
}
