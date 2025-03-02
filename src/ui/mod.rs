use std::hash::{DefaultHasher, Hash, Hasher};

use crate::models::App;
use ratatui::{
    layout::{Constraint, Direction, Layout}, style::{Color, Style}, text::{Line, Span}, widgets::{Block, Borders, List, ListItem, Paragraph}, Frame
};
pub fn ui(app: &App, frame: &mut Frame) {
    //root Layout
    let root_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)])
        .split(frame.area());

    // main section
    let main_section = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(root_layout[0]);

    // right side
    let main_section_right_side = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
        .split(main_section[0]);
    // info areas
    let info_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(main_section_right_side[1]);

    //create blocks
    let online_users = Block::default()
        .title("Online users")
        .borders(Borders::ALL)
        .style(Style::default().fg(ratatui::style::Color::White));

    let chat = Block::default()
        .title("Global Chat")
        .borders(Borders::ALL)
        .style(Style::default().fg(ratatui::style::Color::White));

    let message = Paragraph::new(app.user_input.as_str()).block(
        Block::default()
            .title("Input")
            .borders(Borders::ALL)
            .style(Style::default().fg(ratatui::style::Color::White)),
    );

    let account_profile = Block::default()
        .title("Account Profile")
        .borders(Borders::ALL)
        .style(Style::default().fg(ratatui::style::Color::White));

    let game_menu = Block::default()
        .title("Game_menu")
        .borders(Borders::ALL)
        .style(Style::default().fg(ratatui::style::Color::White));
    // message list

    let messages: Vec<ListItem> = app
    .chat.chats
    .iter()
    .skip(app.scroll_offset)
    .take(10)
    .map(|msg| {
        let spans = Line::from(vec![
            Span::raw("["),
            Span::styled(
                &msg.sender,
                Style::default().fg(get_random_color_for_username(&msg.sender)),
            ),
            Span::raw("#"),
            Span::raw(&msg.time),
            Span::raw("]"),
            Span::raw(&msg.content),
        ]);
        return ListItem::new(spans);
    })
    .collect();
let messages_list = List::new(messages)
        .block(
            Block::default()
                .title("Global Chat")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        )
        .highlight_style(Style::default().fg(Color::Yellow));


    frame.render_widget(message, root_layout[1]);
    frame.render_widget(messages_list, main_section[1]);
    frame.render_widget(account_profile, info_area[1]);
    frame.render_widget(game_menu, info_area[0]);
    frame.render_widget(online_users, main_section_right_side[0]);
}
fn get_random_color_for_username(username: &str) -> Color {
    let mut hasher = DefaultHasher::new();
    username.hash(&mut hasher);
    let hash = hasher.finish();
    // List of candidate colors
    let colors = [
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::LightRed,
        Color::LightGreen,
        Color::LightYellow,
        Color::LightBlue,
        Color::LightMagenta,
        Color::LightCyan,
    ];
    colors[(hash as usize) % colors.len()]
}
