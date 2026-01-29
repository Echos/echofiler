use mlua::{Function, Lua, Result};

/// プラグインフックの種類
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HookType {
    /// ファイラー起動時
    OnStartup,
    /// ファイラー終了時
    OnExit,
    /// ファイル選択時
    OnFileSelect,
    /// ディレクトリ変更時
    OnDirChange,
    /// ファイル操作前
    BeforeFileOp,
    /// ファイル操作後
    AfterFileOp,
}

impl HookType {
    pub fn as_str(&self) -> &'static str {
        match self {
            HookType::OnStartup => "on_startup",
            HookType::OnExit => "on_exit",
            HookType::OnFileSelect => "on_file_select",
            HookType::OnDirChange => "on_dir_change",
            HookType::BeforeFileOp => "before_file_op",
            HookType::AfterFileOp => "after_file_op",
        }
    }
}

/// フックマネージャー
pub struct HookManager {
    hooks: std::collections::HashMap<HookType, Vec<String>>,
}

impl HookManager {
    pub fn new() -> Self {
        Self {
            hooks: std::collections::HashMap::new(),
        }
    }

    /// フックを登録
    pub fn register(&mut self, hook_type: HookType, handler_name: String) {
        self.hooks
            .entry(hook_type)
            .or_insert_with(Vec::new)
            .push(handler_name);
    }

    /// フックを実行
    pub fn trigger(&self, lua: &Lua, hook_type: HookType, args: mlua::MultiValue) -> Result<()> {
        if let Some(handlers) = self.hooks.get(&hook_type) {
            let globals = lua.globals();

            for handler_name in handlers {
                if let Ok(handler) = globals.get::<Function>(handler_name.as_str()) {
                    if let Err(e) = handler.call::<()>(args.clone()) {
                        eprintln!("Hook {} failed: {}", handler_name, e);
                    }
                }
            }
        }

        Ok(())
    }

    /// 登録されているフックの数を取得
    pub fn hook_count(&self, hook_type: HookType) -> usize {
        self.hooks.get(&hook_type).map_or(0, |v| v.len())
    }
}

/// フックAPIをLuaに登録
pub fn register_hook_api(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    let echofiler: mlua::Table = globals.get("echofiler")?;

    let hooks = lua.create_table()?;

    // echofiler.hooks.register(hook_type, handler)
    let register_fn = lua.create_function(|_, (hook_name, handler_name): (String, String)| {
        // 実際の登録処理はHookManagerで行う
        // ここでは関数名を保存するのみ
        println!("Registered hook: {} -> {}", hook_name, handler_name);
        Ok(())
    })?;
    hooks.set("register", register_fn)?;

    echofiler.set("hooks", hooks)?;
    globals.set("echofiler", echofiler)?;

    Ok(())
}
