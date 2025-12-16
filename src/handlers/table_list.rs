use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseEventKind};

pub fn handle_event(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.table_index > 0 {
                app.table_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.table_index < app.tables.len().saturating_sub(1) {
                app.table_index += 1;
            }
        }
        KeyCode::Enter => {
            if !app.tables.is_empty() && app.table_index < app.tables.len() {
                let table_name = app.tables[app.table_index].clone();
                app.current_table = Some(table_name.clone());
                
                // 获取表数据
                if let Some(ref conn) = app.connection {
                    if let Some(ref db_name) = app.current_database {
                        match conn.get_table_data(db_name, &table_name) {
                            Ok((columns, data)) => {
                                app.columns = columns;
                                app.table_data = data;
                                app.data_index = 0;
                                app.screen = crate::app::Screen::TableData;
                            }
                            Err(_) => {
                                // 获取数据失败
                            }
                        }
                    }
                }
            }
        }
        KeyCode::Esc => {
            // 返回数据库列表
            app.current_database = None;
            app.tables.clear();
            app.screen = crate::app::Screen::DatabaseList;
        }
        _ => {}
    }
    false
}

pub fn handle_mouse(app: &mut App, mouse: MouseEvent) {
    match mouse.kind {
        MouseEventKind::Down(_) => {
            // 可以根据鼠标位置选择表
        }
        MouseEventKind::ScrollUp => {
            if app.table_index > 0 {
                app.table_index -= 1;
            }
        }
        MouseEventKind::ScrollDown => {
            if app.table_index < app.tables.len().saturating_sub(1) {
                app.table_index += 1;
            }
        }
        _ => {}
    }
}

