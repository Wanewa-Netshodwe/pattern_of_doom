use std::{
    hash::{DefaultHasher, Hash, Hasher},
    path::Path,
    sync::{
        atomic::{AtomicBool, AtomicI32},
        Arc,
    },
    vec,
};

use crate::models::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect, Spacing},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn ui(app: &App, frame: &mut Frame, disk_space_avail: u64) {
    
    
    //root Layout
    let root_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)])
        .split(frame.area());

    // main section
    let main_section = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(root_layout[0]);

    // right side
    let main_section_right_side = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
        .split(main_section[0]);
    // info areas
    let info_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
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

    let account_profile: Block<'_> = Block::default()
        .title("Account Infomation")
        .borders(Borders::ALL)
        .style(Style::default().fg(ratatui::style::Color::White));

    let game_menu = Block::default()
        .title("Game_menu")
        .borders(Borders::ALL)
        .style(Style::default().fg(ratatui::style::Color::White));
    // message list

    let messages: Vec<ListItem> = app.display_messages
        .iter()
        .skip(app.scroll_offset)
        .take(15)
        .flat_map(|msg| {
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
            return vec![
                ListItem::new(spans),
                ListItem::new(Line::from(""))
            ];
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
        
    
    let content = vec![
        Line::from(Span::styled(
            "Gen-1(Easy Pattern)-350MB",
            Style::default().fg(Color::White),
        ))
        .alignment(Alignment::Right),
        Line::from(Span::styled(
            "Gen-2(Medium Pattern)-750MB",
            Style::default().fg(Color::White),
        ))
        .alignment(Alignment::Right),
        Line::from(Span::styled(
            "Gen-3(Hard Pattern)-1GB",
            Style::default().fg(Color::White),
        ))
        .alignment(Alignment::Right),
        Line::from(Span::styled(
            " Gen-4(Impossible Pattern)-2GB",
            Style::default().fg(Color::White),
        ))
        .alignment(Alignment::Right),
        Line::from(Span::styled(
            format!("Rank :{}", app.user_account.rank),
            Style::default().fg(Color::White),
        ))
        .alignment(Alignment::Left),
        Line::from(Span::styled(
            format!(
                "Solved Patterns :{}",
                app.user_account.patterns_solved.len()
            ),
            Style::default().fg(Color::White),
        ))
        .alignment(Alignment::Left),
        Line::from(Span::styled(
            format!(
                "Current Patterns :{:?}",
                app.user_account.incomplete_pattern.pattern
            ),
            Style::default().fg(Color::White),
        ))
        .alignment(Alignment::Left),
        Line::from(Span::styled(
            format!(
                "Patterns Diff :{:?}",
                app.user_account.incomplete_pattern.level
            ),
            Style::default().fg(Color::White),
        ))
        .alignment(Alignment::Left),
        Line::from(Span::styled("\n", Style::default())).alignment(Alignment::Left),
        if disk_space_avail > 1_111_741_824 {
            Line::from(Span::styled(
                format!("Disk Space Available:{} GB", { disk_space_avail }),
                Style::default().fg(Color::White),
            ))
            .alignment(Alignment::Left)
        } else {
            Line::from(Span::styled(
                format!("Disk Space Available:{} MB", { disk_space_avail }),
                Style::default().fg(Color::White),
            ))
            .alignment(Alignment::Left)
        },
    ];
    let info = Paragraph::new(content).block(game_menu);

    frame.render_widget(message, root_layout[1]);
    frame.render_widget(messages_list, main_section[1]);
    frame.render_widget(account_profile, info_area[1]);
    frame.render_widget(info, info_area[0]);
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

// fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
//     // Cut the given rectangle into three vertical pieces
//     let popup_layout = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([
//             Constraint::Percentage((100 - percent_y) / 2),
//             Constraint::Percentage(percent_y),
//             Constraint::Percentage((100 - percent_y) / 2),
//         ])
//         .split(r);

//     // Then cut the middle vertical piece into three width-wise pieces
//     Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([
//             Constraint::Percentage((100 - percent_x) / 2),
//             Constraint::Percentage(percent_x),
//             Constraint::Percentage((100 - percent_x) / 2),
//         ])
//         .split(popup_layout[1])[1] // Return the middle chunk
// }
// fn render_pop_up(frame: &mut Frame, app: &MutexGuard<'_, App>) {
//     if app.user.toggle || !app.user.confirm.reciever_confirm {
//         if app.user.username == app.user.confirm.sender_username || app.user.username == app.user.confirm.reciever_username && app.user.confirm.sender_confirm == true {
//  // Create a dark overlay for the entire screen
//  let overlay = Block::default().style(
//     Style::default()
//         .bg(Color::Rgb(0, 0, 0))
//         .fg(Color::Rgb(0, 0, 0)),
// );
// frame.render_widget(overlay, frame.size());

// // Create popup area
// let area = centered_rect(60, 25, frame.area());
// let popup_block = Block::default()
//     .title("Message Invitation")
//     .borders(Borders::ALL)
//     .border_style(Style::default().fg(Color::White))
//     .style(Style::default().bg(Color::DarkGray));

// frame.render_widget(popup_block, area);

// let popup_chunks = Layout::default()
//     .direction(Direction::Vertical)
//     .margin(2)
//     .constraints([
//         Constraint::Length(3), // Message area
//         Constraint::Length(3), // Button area
//     ])
//     .split(area);

// // Message text
// let message = if app.user.confirm.sender_username == app.user.username {
//     if app.user.confirm.reciever_confirm {
//         format!(
//             "You're about to start a chat with {}",
//             app.user.confirm.reciever_username
//         )
//     }else{
       
//         format!(
//             "Invitation Rejected",
//         )
//     }
   
// } else {
//         format!(
//             "You received a chat invitation from {}",
//             app.user.confirm.sender_username
//         ) 
// };

// let message_widget = Paragraph::new(message)
//     .alignment(ratatui::layout::Alignment::Center)
//     .style(Style::default().fg(Color::White));
// frame.render_widget(message_widget, popup_chunks[0]);

// // Button layout
// let button_area = Layout::default()
//     .direction(Direction::Horizontal)
//     .spacing(Spacing::Space(5))
//     .constraints([
//         Constraint::Percentage(30), // Left margin
//         Constraint::Length(10),     // OK button
//         Constraint::Min(5),         // Space between
//         Constraint::Length(10),     // Cancel button
//         Constraint::Percentage(30), // Right margin
//     ])
//     .split(popup_chunks[1]);

// // OK button
// let ok_block = Block::default().borders(Borders::BOTTOM);

// let ok_text = Paragraph::new("OK")
// .alignment(ratatui::layout::Alignment::Center)
// .style(Style::default().fg(Color::Cyan))
// .bold()
// .block(ok_block);

// // Cancel button
// let cancel_block = Block::default().borders(Borders::BOTTOM);

// let cancel_text = Paragraph::new("Cancel")
//     .alignment(ratatui::layout::Alignment::Center)
//     .style(Style::default().fg(Color::Cyan))
//     .bold()
//     .block(cancel_block);

// // Render buttons
// if app.user.confirm.reciever_confirm{
//     frame.render_widget(ok_text, button_area[1]);
// }
// frame.render_widget(cancel_text, button_area[3]);

// // Navigation hint
// let hint = Paragraph::new("[Left] ← Navigate → [Right], [Enter] to confirm")
//     .alignment(ratatui::layout::Alignment::Center)
//     .style(Style::default().fg(Color::Gray));
// frame.render_widget(
//     hint,
//     Rect::new(area.x, area.y + area.height - 2, area.width, 1),
// );
//         }
       
//     }
// }

