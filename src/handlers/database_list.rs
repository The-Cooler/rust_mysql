use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseEventKind};

pub fn handle_event(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.database_index > 0 {
                app.database_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.database_index < app.databases.len().saturating_sub(1) {
                app.database_index += 1;
            }
        }
        KeyCode::Enter => {
            if !app.databases.is_empty() && app.database_index < app.databases.len() {
                let db_name = app.databases[app.database_index].clone();
                app.current_database = Some(db_name.clone());
                
                // 获取表列表
                if let Some(ref conn) = app.connection {
                    match conn.get_tables(&db_name) {
                        Ok(tables) => {
                            app.tables = tables;
                            app.table_index = 0;
                            app.screen = crate::app::Screen::TableList;
                        }
                        Err(_) => {
                            // 获取表失败
                        }
                    }
                }
            }
        }
        KeyCode::Esc => {
            // 返回登录界面
            app.screen = crate::app::Screen::Login;
        }
        _ => {}
    }
    false
}

pub fn handle_mouse(app: &mut App, mouse: MouseEvent) {
    match mouse.kind {
        MouseEventKind::Down(_) => {
            // 可以根据鼠标位置选择数据库
            // 这里简化处理，主要使用键盘
        }
        MouseEventKind::ScrollUp => {
            if app.database_index > 0 {
                app.database_index -= 1;
            }
        }
        MouseEventKind::ScrollDown => {
            if app.database_index < app.databases.len().saturating_sub(1) {
                app.database_index += 1;
            }
        }
        _ => {}
    }
}

