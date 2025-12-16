use crate::app::App;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Frame;

pub fn draw(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app: &mut App) {
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.size());
    
    // 显示当前数据库名称
    let default_db = String::from("未知");
    let db_name = app.current_database.as_ref().unwrap_or(&default_db);
    let title = Paragraph::new(format!("当前数据库: {}", db_name))
        .block(Block::default().borders(Borders::ALL).title("信息"))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(title, chunks[0]);
    
    // 显示表列表
    let items: Vec<ListItem> = app.tables
        .iter()
        .map(|table| ListItem::new(table.clone()))
        .collect();
    
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("表列表 (Enter: 查看数据, Esc: 返回)")
        )
        .highlight_symbol(">>")
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
        .style(Style::default().fg(Color::White));
    
    app.table_state.select(Some(app.table_index));
    f.render_stateful_widget(list, chunks[1], &mut app.table_state);
}

