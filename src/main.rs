use crossterm::{
    event::Event,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use echofiler::{event::EventHandler, App};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let mut terminal = setup_terminal()?;
    let result = run_app(&mut terminal);
    restore_terminal(&mut terminal)?;

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn setup_terminal() -> anyhow::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> anyhow::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> anyhow::Result<()> {
    let mut app = App::new()?;
    let event_handler = EventHandler::new();

    loop {
        // 画面をクリアする必要がある場合
        if app.screen_needs_clear {
            terminal.clear()?;
            app.screen_needs_clear = false;
        }

        terminal.draw(|f| ui(f, &mut app))?;

        if app.should_quit {
            break;
        }

        // ファイル変更をチェック
        if app.check_file_changes() {
            app.left_pane.current_tab_mut().reload();
            app.right_pane.current_tab_mut().reload();
        }

        // 設定ファイル変更をチェック
        app.check_config_changes();

        // コマンドを起動する必要がある場合
        if let Some((command, path)) = app.suspend_for_command.take() {
            // TUIを一時停止
            restore_terminal(terminal)?;

            // コマンドを起動（フォアグラウンド）
            let result = echofiler::fs::opener::edit_file_foreground(&path, &command);

            // TUIを再開
            *terminal = setup_terminal()?;

            // 結果をメッセージダイアログで表示
            match result {
                Ok(_) => {
                    // 成功時はメッセージなし（ファイルが編集されたことは明らか）
                }
                Err(e) => {
                    app.show_error(&format!("Command failed:\n{}", e));
                }
            }

            // 次のループで画面を再描画
            continue;
        }

        if let Some(event) = event_handler.next()? {
            match event {
                Event::Key(key_event) => {
                    let key = key_event.into();
                    let action = app.handle_key(key);
                    app.update(action);
                    // ディレクトリ移動時に監視対象を更新
                    app.update_watch_path();
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }

    Ok(())
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    use echofiler::core::PaneSide;
    use echofiler::ui::{bookmark::BookmarkWidget, bookmark_select::BookmarkSelectDialog, commandline::CommandLine, confirm::ConfirmWidget, help::HelpWidget, message_dialog::MessageDialog, pane::PaneWidget, preview::PreviewWidget, sidebar::SidebarWidget, statusline::Statusline, Layout};
    use echofiler::input::InputMode;

    // Helpモードの場合はヘルプ画面を表示
    if app.mode == InputMode::Help {
        let help_widget = HelpWidget::new(&app.config.theme);
        f.render_widget(help_widget, f.area());
        return;
    }

    // Bookmarkモードの場合は専用画面を表示
    if app.mode == InputMode::Bookmark {
        let bookmark_widget = BookmarkWidget::new(&app.bookmarks, app.bookmark_cursor, &app.config.theme);
        f.render_widget(bookmark_widget, f.area());
        return;
    }

    // MessageDialogモードの場合はメッセージダイアログを表示
    if app.mode == InputMode::MessageDialog {
        let message_dialog = MessageDialog::new(&app.dialog_message, app.is_error_dialog);
        f.render_widget(message_dialog, f.area());
        return;
    }

    let show_command_line = matches!(app.mode, InputMode::Command | InputMode::Search);
    let main_chunks = if show_command_line {
        use ratatui::layout::{Constraint, Direction};
        ratatui::layout::Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(f.area())
            .to_vec()
    } else {
        Layout::split_horizontal(f.area())
    };

    // サイドバー表示の判定
    let show_sidebar = app.config.sidebar.enabled && !app.show_preview;
    let sidebar_position = &app.config.sidebar.position;
    let sidebar_width = app.config.sidebar.width;

    // サイドバーが有効な場合、3カラムレイアウト
    let (panes, sidebar_area) = if show_sidebar {
        use ratatui::layout::{Constraint, Direction};
        let is_left = sidebar_position == "left";

        let chunks = ratatui::layout::Layout::default()
            .direction(Direction::Horizontal)
            .constraints(if is_left {
                vec![
                    Constraint::Length(sidebar_width),
                    Constraint::Min(1),
                ]
            } else {
                vec![
                    Constraint::Min(1),
                    Constraint::Length(sidebar_width),
                ]
            })
            .split(main_chunks[0]);

        let pane_area = if is_left { chunks[1] } else { chunks[0] };
        let sidebar = if is_left { chunks[0] } else { chunks[1] };
        let panes = Layout::split_dual_pane_with_ratio(pane_area, &app.config.layout.ratio);
        (panes, Some(sidebar))
    } else {
        let panes = Layout::split_dual_pane_with_ratio(main_chunks[0], &app.config.layout.ratio);
        (panes, None)
    };

    // ペインの高さを記録（スクロール計算用）
    app.last_pane_height = panes[0].height;

    // プレビューモードONの場合、アクティブペインとは逆のペインにプレビューを表示
    if app.show_preview {
        let current_entry = app.active_pane().current_tab().current_entry();
        let preview_path = current_entry.map(|e| e.path.as_path());
        let preview_widget = PreviewWidget::new(preview_path, &app.config.theme, app.preview_scroll);

        let show_icons = app.config.general.show_icons;
        let icon_style = app.config.general.icon_style;
        let icon_spacing = app.config.general.icon_spacing;
        match app.active_pane {
            PaneSide::Left => {
                // 左ペインがアクティブ → 左にファイル一覧、右にプレビュー
                let left_widget = PaneWidget::new(&app.left_pane, true, &app.config.theme, show_icons, icon_style, icon_spacing);
                f.render_widget(left_widget, panes[0]);
                f.render_widget(preview_widget, panes[1]);
            }
            PaneSide::Right => {
                // 右ペインがアクティブ → 左にプレビュー、右にファイル一覧
                f.render_widget(preview_widget, panes[0]);
                let right_widget = PaneWidget::new(&app.right_pane, true, &app.config.theme, show_icons, icon_style, icon_spacing);
                f.render_widget(right_widget, panes[1]);
            }
        }
    } else {
        // プレビューOFFの場合は通常の二画面表示
        let show_icons = app.config.general.show_icons;
        let icon_style = app.config.general.icon_style;
        let icon_spacing = app.config.general.icon_spacing;
        let left_widget = PaneWidget::new(&app.left_pane, app.active_pane == PaneSide::Left, &app.config.theme, show_icons, icon_style, icon_spacing);
        let right_widget = PaneWidget::new(&app.right_pane, app.active_pane == PaneSide::Right, &app.config.theme, show_icons, icon_style, icon_spacing);
        f.render_widget(left_widget, panes[0]);
        f.render_widget(right_widget, panes[1]);
    }

    // サイドバーをレンダリング
    if let Some(sidebar_area) = sidebar_area {
        let current_entry = app.active_pane().current_tab().current_entry();
        let sidebar_widget = SidebarWidget::new(current_entry, &app.config.theme);
        f.render_widget(sidebar_widget, sidebar_area);
    }

    let statusline = Statusline::new(app);
    f.render_widget(statusline, main_chunks[1]);

    if show_command_line {
        let commandline = CommandLine::new(&app.command_prompt, &app.command_input);
        f.render_widget(commandline, main_chunks[2]);
    }

    // Confirmモードの場合は確認ダイアログを表示
    if app.mode == InputMode::Confirm {
        let confirm_widget = ConfirmWidget::new(&app.confirm_message, &app.config.theme);
        f.render_widget(confirm_widget, f.area());
    }

    // BookmarkSelectモードの場合はブックマーク一覧ダイアログを表示
    if app.mode == InputMode::BookmarkSelect {
        let bookmark_select = BookmarkSelectDialog::new(&app.bookmarks);
        f.render_widget(bookmark_select, f.area());
    }

    // BookmarkPrefixモードの場合はインジケーターを表示
    if app.mode == InputMode::BookmarkPrefix {
        use ratatui::text::Span;
        use ratatui::widgets::{Block, Borders, Paragraph};
        use ratatui::style::{Color, Modifier, Style};
        use ratatui::layout::Alignment;

        let prefix_str = app.current_prefix.as_deref().unwrap_or("<prefix>");
        let title = format!("Bookmark Prefix: {}", prefix_str);

        let message = vec![
            ratatui::text::Line::from(vec![Span::styled(
                "Waiting for next key...",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]),
            ratatui::text::Line::from(""),
            ratatui::text::Line::from(vec![Span::styled(
                "Mapped keys: b (add) | B (show all) | m (show key list)",
                Style::default().fg(Color::White),
            )]),
            ratatui::text::Line::from(vec![Span::styled(
                "Other key: Jump to bookmark | Esc: Cancel",
                Style::default().fg(Color::DarkGray),
            )]),
        ];

        let dialog_width = 65.min(f.area().width.saturating_sub(4));
        let dialog_height = 9;
        let dialog_area = ratatui::layout::Rect {
            x: f.area().x + (f.area().width.saturating_sub(dialog_width)) / 2,
            y: f.area().y + (f.area().height.saturating_sub(dialog_height)) / 2,
            width: dialog_width,
            height: dialog_height,
        };

        let paragraph = Paragraph::new(message)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan))
                    .title(format!(" {} ", title)),
            )
            .alignment(Alignment::Center);

        f.render_widget(paragraph, dialog_area);
    }
}
