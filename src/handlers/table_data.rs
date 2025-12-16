use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseEventKind};

pub fn handle_event(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.data_index > 0 {
                app.data_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.data_index < app.table_data.len().saturating_sub(1) {
                app.data_index += 1;
            }
        }
        KeyCode::Left | KeyCode::Char('h') => {
            // 切换到表列表（左侧）
            // 可以在这里实现切换焦点到左侧表列表
        }
        KeyCode::Right | KeyCode::Char('l') => {
            // 切换到数据视图（右侧）
        }
        KeyCode::Esc => {
            // 返回表列表
            app.current_table = None;
            app.table_data.clear();
            app.columns.clear();
            app.screen = crate::app::Screen::TableList;
        }
        _ => {}
    }
    false
}

pub fn handle_mouse(app: &mut App, mouse: MouseEvent) {
    match mouse.kind {
        MouseEventKind::Down(_) => {
            // 可以根据鼠标位置选择数据行
        }
        MouseEventKind::ScrollUp => {
            if app.data_index > 0 {
                app.data_index -= 1;
            }
        }
        MouseEventKind::ScrollDown => {
            if app.data_index < app.table_data.len().saturating_sub(1) {
                app.data_index += 1;
            }
        }
        _ => {}
    }
}

