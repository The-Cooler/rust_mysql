mod app;
mod handlers;
mod ui;
mod database;
mod config;

use std::io;
use std::time::Duration;

use crate::app::{App, Screen};
use crate::handlers::{login, database_list, table_list, table_data};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    // 初始化终端环境
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 初始化App状态
    let mut app = App::new();

    // 主循环
    loop {
        // 渲染界面
        terminal.draw(|f| {
            match app.screen {
                Screen::Login => ui::login::draw(f, &mut app),
                Screen::DatabaseList => ui::database_list::draw(f, &mut app),
                Screen::TableList => ui::table_list::draw(f, &mut app),
                Screen::TableData => ui::table_data::draw(f, &mut app),
            }
        })?;

        // 事件处理
        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => {
                    let exit = match app.screen {
                        Screen::Login => login::handle_event(&mut app, key),
                        Screen::DatabaseList => database_list::handle_event(&mut app, key),
                        Screen::TableList => table_list::handle_event(&mut app, key),
                        Screen::TableData => table_data::handle_event(&mut app, key),
                    };
                    if exit {
                        break;
                    }
                }
                Event::Mouse(mouse) => {
                    match app.screen {
                        Screen::Login => login::handle_mouse(&mut app, mouse),
                        Screen::DatabaseList => database_list::handle_mouse(&mut app, mouse),
                        Screen::TableList => table_list::handle_mouse(&mut app, mouse),
                        Screen::TableData => table_data::handle_mouse(&mut app, mouse),
                    }
                }
                _ => {}
            }
        }
    }

    // 恢复终端状态
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}

