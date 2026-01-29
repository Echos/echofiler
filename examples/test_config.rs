use echofiler::config::Config;

fn main() -> anyhow::Result<()> {
    let config = Config::load()?;

    println!("設定ファイル読み込み成功");
    println!();
    println!("一般設定:");
    println!("  show_hidden: {}", config.general.show_hidden);
    println!("  confirm_delete: {}", config.general.confirm_delete);
    println!("  confirm_overwrite: {}", config.general.confirm_overwrite);
    println!("  use_trash: {}", config.general.use_trash);
    println!();
    println!("レイアウト設定:");
    println!("  style: {}", config.layout.style);
    println!("  ratio: {:?}", config.layout.ratio);
    println!("  show_preview: {}", config.layout.show_preview);
    println!("  preview_ratio: {}", config.layout.preview_ratio);
    println!();
    println!("ソート設定:");
    println!("  method: {}", config.sort.method);
    println!("  directories_first: {}", config.sort.directories_first);
    println!("  reverse: {}", config.sort.reverse);
    println!();
    println!("プレビュー設定:");
    println!("  max_size: {}", config.preview.max_size);
    println!("  syntax_highlight: {}", config.preview.syntax_highlight);
    println!("  image_protocol: {}", config.preview.image_protocol);
    println!();
    println!("ログ設定:");
    println!("  level: {}", config.log.level);
    println!("  file: {}", config.log.file);

    Ok(())
}
