use crate::app::App;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;

pub fn draw(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(f.size());
    
    let main_area = chunks[0];
    
    let port_str = app.port.to_string();
    let password_display = "*".repeat(app.password.len());
    let fields = vec![
        ("主机", app.host.as_str(), 0),
        ("端口", port_str.as_str(), 1),
        ("用户名", app.username.as_str(), 2),
        ("密码", password_display.as_str(), 3),
    ];
    
    let mut paragraphs = Vec::new();
    for (i, (label, value, field_idx)) in fields.iter().enumerate() {
        let is_active = app.input_field == *field_idx;
        let style = if is_active {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        
        let paragraph = Paragraph::new(*value)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("{} {}", if is_active { ">>" } else { "  " }, label))
                    .style(style)
            )
            .style(style);
        
        paragraphs.push((paragraph, i));
    }
    
    let constraints: Vec<Constraint> = vec![
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3), // 连接按钮
        Constraint::Length(3), // 错误消息区域
        Constraint::Min(0),
    ];
    
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints(constraints)
        .split(main_area);
    
    // 保存chunks信息用于鼠标点击判断
    app.chunks_info = Some(
        chunks.iter()
            .take(5) // 只保存前5个chunks (4个输入框 + 1个连接按钮)
            .map(|chunk| (chunk.y, chunk.height))
            .collect()
    );
    
    for (i, (paragraph, _)) in paragraphs.iter().enumerate() {
        f.render_widget(paragraph.clone(), chunks[i]);
    }
    
    // 显示连接按钮
    let (connect_text, connect_style) = match app.input_field {
        4 => (
            ">> [连接]",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        ),
        _ => (
            "   [连接]",
            Style::default().fg(Color::White)
        ),
    };
    let connect_paragraph = Paragraph::new(connect_text)
        .block(Block::default().borders(Borders::ALL).title("操作"))
        .style(connect_style);
    f.render_widget(connect_paragraph, chunks[4]);
    
    // 显示错误消息或操作提示
    if let Some(ref error_msg) = app.error_message {
        let error_paragraph = Paragraph::new(error_msg.as_str())
            .block(Block::default().borders(Borders::ALL).title("错误"))
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
        f.render_widget(error_paragraph, chunks[5]);
    } else {
        // 显示操作提示
        let hint_text = "提示: Tab/↑↓切换字段 | Enter确认/连接 | Esc退出 | 鼠标点击切换";
        let hint_paragraph = Paragraph::new(hint_text)
            .block(Block::default().borders(Borders::ALL).title("操作提示"))
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(hint_paragraph, chunks[5]);
    }
    
    // 显示光标位置
    match app.input_field {
        0 | 2 | 3 => {
            let current_input = match app.input_field {
                0 => &app.host,
                2 => &app.username,
                3 => &app.password,
                _ => unreachable!(),
            };
            let cursor_x = current_input.len().min(app.input_cursor);
            f.set_cursor(
                chunks[app.input_field].x + cursor_x as u16 + 1,
                chunks[app.input_field].y + 1,
            );
        }
        _ => {}
    }
}

