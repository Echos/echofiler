-- echofiler サンプルプラグイン

-- プラグイン情報を定義
plugin = {
    name = "example",
    version = "0.1.0",
    author = "echofiler contributors",
    description = "Example plugin demonstrating the plugin system"
}

-- 起動時フック
function on_startup()
    echofiler.log.info("Example plugin loaded!")
    echofiler.ui.notify("Welcome to echofiler!")
end

-- ディレクトリ変更時フック
function on_dir_change(path)
    echofiler.log.info("Directory changed to: " .. (path or "unknown"))
end

-- ファイル選択時フック
function on_file_select(file)
    echofiler.log.info("File selected: " .. (file or "unknown"))
end

-- フックを登録
echofiler.hooks.register("on_startup", "on_startup")
echofiler.hooks.register("on_dir_change", "on_dir_change")
echofiler.hooks.register("on_file_select", "on_file_select")

-- カスタム関数の例
function hello_world()
    echofiler.log.info("Hello from Lua plugin!")
    return "Hello, World!"
end

-- ファイルシステムAPI使用例
function check_file(path)
    if echofiler.fs.exists(path) then
        if echofiler.fs.is_dir(path) then
            return "Directory exists"
        else
            return "File exists"
        end
    else
        return "Does not exist"
    end
end
