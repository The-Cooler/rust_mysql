use crate::app::App;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Frame;

pub fn draw(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(f.size());
    
    let items: Vec<ListItem> = app.databases
        .iter()
        .map(|db| ListItem::new(db.clone()))
        .collect();
    
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("数据库列表 (Enter: 进入, Esc: 返回)")
        )
        .highlight_symbol(">>")
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
        .style(Style::default().fg(Color::White));
    
    app.database_state.select(Some(app.database_index));
    f.render_stateful_widget(list, chunks[0], &mut app.database_state);
}

