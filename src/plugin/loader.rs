use crate::plugin::api::register_api;
use crate::plugin::hooks::{register_hook_api, HookManager};
use anyhow::{Context, Result};
use mlua::{Lua, Table};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// プラグイン情報
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
}

/// プラグインマネージャー
pub struct PluginManager {
    lua: Lua,
    plugins: HashMap<String, PluginInfo>,
    plugin_dir: PathBuf,
    hook_manager: HookManager,
}

impl PluginManager {
    /// 新しいプラグインマネージャーを作成
    pub fn new() -> Result<Self> {
        let lua = Lua::new();
        let plugin_dir = Self::get_plugin_dir()?;

        // APIを登録
        register_api(&lua)
            .map_err(|e| anyhow::anyhow!("Failed to register plugin API: {}", e))?;

        // フックAPIを登録
        register_hook_api(&lua)
            .map_err(|e| anyhow::anyhow!("Failed to register hook API: {}", e))?;

        Ok(Self {
            lua,
            plugins: HashMap::new(),
            plugin_dir,
            hook_manager: HookManager::new(),
        })
    }

    /// プラグインディレクトリのパスを取得
    fn get_plugin_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Failed to get config directory")?;
        Ok(config_dir.join("echofiler").join("plugins"))
    }

    /// すべてのプラグインを読み込み
    pub fn load_all(&mut self) -> Result<()> {
        if !self.plugin_dir.exists() {
            fs::create_dir_all(&self.plugin_dir)?;
            return Ok(());
        }

        for entry in fs::read_dir(&self.plugin_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("lua") {
                if let Err(e) = self.load_plugin(&path) {
                    eprintln!("Failed to load plugin {:?}: {}", path, e);
                }
            }
        }

        Ok(())
    }

    /// 個別のプラグインを読み込み
    pub fn load_plugin(&mut self, path: &Path) -> Result<()> {
        let script = fs::read_to_string(path)
            .with_context(|| format!("Failed to read plugin file: {:?}", path))?;

        // プラグインスクリプトを実行
        self.lua.load(&script).exec()
            .map_err(|e| anyhow::anyhow!("Failed to execute plugin {:?}: {}", path, e))?;

        // プラグイン情報を取得
        let info = self.extract_plugin_info()?;

        let plugin_name = info.name.clone();
        self.plugins.insert(plugin_name.clone(), info);

        println!("Loaded plugin: {}", plugin_name);

        Ok(())
    }

    /// プラグイン情報をLuaグローバルから抽出
    fn extract_plugin_info(&self) -> Result<PluginInfo> {
        let globals = self.lua.globals();
        let plugin_table: Table = globals.get("plugin")
            .map_err(|e| anyhow::anyhow!("Plugin must define 'plugin' table: {}", e))?;

        let name = plugin_table.get::<String>("name")
            .map_err(|e| anyhow::anyhow!("Plugin must have 'name' field: {}", e))?;
        let version = plugin_table.get::<String>("version")
            .unwrap_or_else(|_| "0.1.0".to_string());
        let author = plugin_table.get::<String>("author")
            .unwrap_or_else(|_| "Unknown".to_string());
        let description = plugin_table.get::<String>("description")
            .unwrap_or_else(|_| "No description".to_string());

        Ok(PluginInfo {
            name,
            version,
            author,
            description,
        })
    }

    /// 読み込まれたプラグインの一覧を取得
    pub fn list_plugins(&self) -> Vec<&PluginInfo> {
        self.plugins.values().collect()
    }

    /// Luaインスタンスへの参照を取得
    pub fn lua(&self) -> &Lua {
        &self.lua
    }

    /// プラグインディレクトリのパスを取得
    pub fn plugin_dir(&self) -> &Path {
        &self.plugin_dir
    }

    /// フックマネージャーへの参照を取得
    pub fn hook_manager(&self) -> &HookManager {
        &self.hook_manager
    }

    /// フックマネージャーへの可変参照を取得
    pub fn hook_manager_mut(&mut self) -> &mut HookManager {
        &mut self.hook_manager
    }
}
