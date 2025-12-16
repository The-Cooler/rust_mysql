use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use crate::database::DatabaseConnection;
use crate::config;

pub fn handle_event(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Tab => {
            app.input_field = (app.input_field + 1) % 5; // 0-3: 输入字段, 4: 连接按钮
            match app.input_field {
                0 => app.input_cursor = app.host.len(),
                1 => app.input_cursor = 0, // port 字段不需要光标
                2 => app.input_cursor = app.username.len(),
                3 => app.input_cursor = app.password.len(),
                _ => {}
            }
        }
        KeyCode::BackTab => {
            app.input_field = match app.input_field {
                0 => 4,
                n => n - 1,
            };
            match app.input_field {
                0 => app.input_cursor = app.host.len(),
                1 => app.input_cursor = 0,
                2 => app.input_cursor = app.username.len(),
                3 => app.input_cursor = app.password.len(),
                _ => {}
            }
        }
        KeyCode::Enter => {
            match app.input_field {
                4 => return handle_connect(app), // 连接按钮
                1 => {
                    // port 字段，跳转到下一个
                    app.input_field = 2;
                    app.input_cursor = app.username.len();
                }
                _ => {
                    // 其他字段，跳转到下一个
                    app.input_field = (app.input_field + 1) % 5;
                    match app.input_field {
                        0 => app.input_cursor = app.host.len(),
                        1 => app.input_cursor = 0,
                        2 => app.input_cursor = app.username.len(),
                        3 => app.input_cursor = app.password.len(),
                        _ => {}
                    }
                }
            }
        }
        KeyCode::Up => {
            // 向上切换输入框
            app.input_field = match app.input_field {
                0 => 4, // 从主机跳到连接按钮
                1 => 0, // 从端口跳到主机
                2 => 1, // 从用户名跳到端口
                3 => 2, // 从密码跳到用户名
                4 => 3, // 从连接按钮跳到密码
                _ => app.input_field,
            };
            match app.input_field {
                0 => app.input_cursor = app.host.len(),
                1 => app.input_cursor = 0,
                2 => app.input_cursor = app.username.len(),
                3 => app.input_cursor = app.password.len(),
                _ => {}
            }
        }
        KeyCode::Down => {
            // 向下切换输入框
            app.input_field = match app.input_field {
                0 => 1, // 从主机跳到端口
                1 => 2, // 从端口跳到用户名
                2 => 3, // 从用户名跳到密码
                3 => 4, // 从密码跳到连接按钮
                4 => 0, // 从连接按钮跳到主机
                _ => app.input_field,
            };
            match app.input_field {
                0 => app.input_cursor = app.host.len(),
                1 => app.input_cursor = 0,
                2 => app.input_cursor = app.username.len(),
                3 => app.input_cursor = app.password.len(),
                _ => {}
            }
        }
        KeyCode::Left => {
            match app.input_field {
                0 | 2 | 3 => {
                    if app.input_cursor > 0 {
                        app.input_cursor -= 1;
                    }
                }
                _ => {}
            }
        }
        KeyCode::Right => {
            match app.input_field {
                0 => {
                    if app.input_cursor < app.host.len() {
                        app.input_cursor += 1;
                    }
                }
                2 => {
                    if app.input_cursor < app.username.len() {
                        app.input_cursor += 1;
                    }
                }
                3 => {
                    if app.input_cursor < app.password.len() {
                        app.input_cursor += 1;
                    }
                }
                _ => {}
            }
        }
        KeyCode::Home => {
            match app.input_field {
                0 | 2 | 3 => app.input_cursor = 0,
                _ => {}
            }
        }
        KeyCode::End => {
            match app.input_field {
                0 => app.input_cursor = app.host.len(),
                2 => app.input_cursor = app.username.len(),
                3 => app.input_cursor = app.password.len(),
                _ => {}
            }
        }
        KeyCode::Char(c) => {
            match app.input_field {
                4 => {
                    // 连接按钮
                    if c == ' ' || c == '\n' || c == '\r' {
                        return handle_connect(app);
                    }
                }
                1 => {
                    // port 字段
                    app.error_message = None; // 清除错误消息
                    if c.is_ascii_digit() {
                        let port_str = app.port.to_string();
                        if let Ok(new_port) = format!("{}{}", port_str, c).parse::<u16>() {
                            app.port = new_port;
                        }
                    } else if c == '\x08' || c == '\x7f' {
                        // Backspace
                        app.port = app.port / 10;
                    }
                }
                0 => {
                    app.error_message = None; // 清除错误消息
                    if c == '\x08' || c == '\x7f' {
                        if app.input_cursor > 0 {
                            app.host.remove(app.input_cursor - 1);
                            app.input_cursor -= 1;
                        }
                    } else if !c.is_control() {
                        app.host.insert(app.input_cursor, c);
                        app.input_cursor += 1;
                    }
                }
                2 => {
                    app.error_message = None; // 清除错误消息
                    if c == '\x08' || c == '\x7f' {
                        if app.input_cursor > 0 {
                            app.username.remove(app.input_cursor - 1);
                            app.input_cursor -= 1;
                        }
                    } else if !c.is_control() {
                        app.username.insert(app.input_cursor, c);
                        app.input_cursor += 1;
                    }
                }
                3 => {
                    app.error_message = None; // 清除错误消息
                    if c == '\x08' || c == '\x7f' {
                        if app.input_cursor > 0 {
                            app.password.remove(app.input_cursor - 1);
                            app.input_cursor -= 1;
                        }
                    } else if !c.is_control() {
                        app.password.insert(app.input_cursor, c);
                        app.input_cursor += 1;
                    }
                }
                _ => {}
            }
        }
        KeyCode::Backspace => {
            match app.input_field {
                1 => app.port = app.port / 10,
                0 => {
                    if app.input_cursor > 0 {
                        app.host.remove(app.input_cursor - 1);
                        app.input_cursor -= 1;
                    }
                }
                2 => {
                    if app.input_cursor > 0 {
                        app.username.remove(app.input_cursor - 1);
                        app.input_cursor -= 1;
                    }
                }
                3 => {
                    if app.input_cursor > 0 {
                        app.password.remove(app.input_cursor - 1);
                        app.input_cursor -= 1;
                    }
                }
                _ => {}
            }
        }
        KeyCode::Delete => {
            match app.input_field {
                0 => {
                    if app.input_cursor < app.host.len() {
                        app.host.remove(app.input_cursor);
                    }
                }
                2 => {
                    if app.input_cursor < app.username.len() {
                        app.username.remove(app.input_cursor);
                    }
                }
                3 => {
                    if app.input_cursor < app.password.len() {
                        app.password.remove(app.input_cursor);
                    }
                }
                _ => {}
            }
        }
        KeyCode::Esc => {
            return true; // 退出程序
        }
        _ => {}
    }
    false
}

pub fn handle_mouse(app: &mut App, mouse: MouseEvent) {
    use crossterm::event::{MouseButton, MouseEventKind};
    
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            // 根据鼠标点击位置切换输入框
            if let Some(ref chunks_info) = app.chunks_info {
                let mouse_y = mouse.row;
                // 检查鼠标点击了哪个输入框
                for (field_idx, (chunk_y, chunk_height)) in chunks_info.iter().enumerate() {
                    if mouse_y >= *chunk_y && mouse_y < *chunk_y + *chunk_height {
                        // 点击了这个输入框
                        app.input_field = field_idx;
                        match field_idx {
                            0 => app.input_cursor = app.host.len(),
                            1 => app.input_cursor = 0,
                            2 => app.input_cursor = app.username.len(),
                            3 => app.input_cursor = app.password.len(),
                            _ => {}
                        }
                        break;
                    }
                }
            }
        }
        _ => {}
    }
}

fn handle_connect(app: &mut App) -> bool {
    // 清除之前的错误消息
    app.error_message = None;
    
    // 验证输入
    if app.host.is_empty() {
        app.error_message = Some("错误: 主机地址不能为空".to_string());
        return false;
    }
    if app.username.is_empty() {
        app.error_message = Some("错误: 用户名不能为空".to_string());
        return false;
    }
    
    // 尝试连接
    match DatabaseConnection::new(&app.host, app.port, &app.username, &app.password) {
        Ok(conn) => {
            // 保存配置
            let config = config::Config::new(
                app.host.clone(),
                app.port,
                app.username.clone(),
                app.password.clone(),
            );
            let _ = config::save_config(&config);
            
            app.connection = Some(conn);
            
            // 获取数据库列表
            if let Some(ref conn) = app.connection {
                match conn.get_databases() {
                    Ok(databases) => {
                        app.databases = databases;
                        app.database_index = 0;
                        app.screen = crate::app::Screen::DatabaseList;
                    }
                    Err(e) => {
                        app.error_message = Some(format!("错误: 无法获取数据库列表 - {}", e));
                    }
                }
            }
        }
        Err(e) => {
            // 连接失败，显示错误消息
            app.error_message = Some(format!("错误: 连接失败 - {}", e));
        }
    }
    false
}

