use crate::app::App;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Table, Row, Cell};
use tui::Frame;

pub fn draw(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app: &mut App) {
    // 左右分栏：左侧表列表，右侧数据
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.size());
    
    // 左侧：表列表
    let table_items: Vec<ListItem> = app.tables
        .iter()
        .map(|table| {
            let name = if Some(table.clone()) == app.current_table {
                format!(">> {}", table)
            } else {
                format!("   {}", table)
            };
            ListItem::new(name)
        })
        .collect();
    
    let table_list = List::new(table_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("表列表")
        )
        .style(Style::default().fg(Color::White));
    
    f.render_widget(table_list, chunks[0]);
    
    // 右侧：数据表格
    let right_chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(chunks[1]);
    
    // 显示表名
    let default_table = String::from("未知");
    let table_name = app.current_table.as_ref().unwrap_or(&default_table);
    let title = Paragraph::new(format!("表: {} (Esc: 返回)", table_name))
        .block(Block::default().borders(Borders::ALL).title("数据"))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(title, right_chunks[0]);
    
    // 显示数据
    if app.table_data.is_empty() {
        let empty_text = Paragraph::new("暂无数据")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(empty_text, right_chunks[1]);
    } else {
        // 创建表格
        let mut rows = Vec::new();
        
        // 表头
        let header_cells: Vec<Cell> = app.columns
            .iter()
            .map(|col| Cell::from(col.clone()).style(Style::default().fg(Color::Yellow).add_modifier(tui::style::Modifier::BOLD)))
            .collect();
        rows.push(Row::new(header_cells));
        
        // 数据行 - 确保每行数据在一行显示
        for (idx, row_data) in app.table_data.iter().enumerate() {
            let is_selected = idx == app.data_index;
            let style = if is_selected {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else {
                Style::default().fg(Color::White)
            };
            
            let cells: Vec<Cell> = row_data
                .iter()
                .map(|cell| {
                    // 截断过长的内容，确保一行显示
                    let display = if cell.len() > 20 {
                        format!("{}...", &cell[..20])
                    } else {
                        cell.clone()
                    };
                    Cell::from(display).style(style)
                })
                .collect();
            rows.push(Row::new(cells));
        }
        
        let col_count = app.columns.len().max(1);
        let col_width = 100 / col_count as u16;
        let widths: Vec<Constraint> = (0..col_count)
            .map(|_| Constraint::Percentage(col_width))
            .collect();
        
        let table = Table::new(rows)
            .block(Block::default().borders(Borders::ALL))
            .widths(&widths)
            .column_spacing(1);
        
        f.render_widget(table, right_chunks[1]);
        
        // 高亮当前选中的行
        app.data_state.select(Some(app.data_index));
    }
}

