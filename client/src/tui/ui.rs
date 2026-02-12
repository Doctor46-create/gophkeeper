use crate::tui::app::{AddStep, InputMode, LoginStep, Screen, TuiApp};
use ratatui::{
  Frame,
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Row, Table, Wrap},
};

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
  let popup_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Percentage((100 - percent_y) / 2),
      Constraint::Percentage(percent_y),
      Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

  Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage((100 - percent_x) / 2),
      Constraint::Percentage(percent_x),
      Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

pub fn draw(f: &mut Frame, app: &TuiApp) {
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(1)
    .constraints([
      Constraint::Length(3),
      Constraint::Min(1),
      Constraint::Length(2),
    ])
    .split(f.size());

  if app.screen != Screen::Menu {
    draw_header(f, chunks[0]);
  } else {
    let empty_block = Block::default();
    f.render_widget(empty_block, chunks[0]);
  }
  draw_body(f, app, chunks[1]);
  draw_notification(f, app);
}

fn draw_header(f: &mut Frame, area: Rect) {
  let header = Paragraph::new(Line::from(vec![
    Span::styled(
      "🔐 GopherKeeper",
      Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD),
    ),
    Span::raw(" | "),
    Span::styled("😎", Style::default().fg(Color::Yellow)),
    Span::raw("TUI"),
  ]))
  .alignment(Alignment::Center);

  f.render_widget(header, area);
}

fn draw_body(f: &mut Frame, app: &TuiApp, area: Rect) {
  match app.screen {
    Screen::Menu => draw_menu(f, app, area),
    Screen::Secrets => draw_secrets(f, app, area),
    Screen::AddSecret => draw_add_secret(f, app, area),
    Screen::Login | Screen::Register => draw_auth(f, app, area),
    Screen::MasterPassword => draw_master_password(f, app),
  }
}

//fn draw_menu(f: &mut Frame, app: &TuiApp, area: Rect) {
//  let chunks = Layout::default()
//    .direction(Direction::Vertical)
//    .constraints([Constraint::Length(14), Constraint::Min(6)])
//    .split(area);
//
//  let banner_area = chunks[0];
//  let menu_area = chunks[1];
//
//  let banner_chunks = Layout::default()
//    .direction(Direction::Horizontal)
//    .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
//    .split(banner_area);
//
//  let gopher_area = banner_chunks[0];
//  let text_area = banner_chunks[1];
//
//  let gopher_art = r#"
//     .----.
//          _.'__    `.
//      .--(#)(##)---/#\
//    .' @          /###\
//    :         ,   #####
//     `-..__.-' _.-\###/
//        `;_:    `"'
//         .'"""""`.
//        /,       ,\
//       //         \\
//       `-._______.-'
//       ___`. | .'___
//      (______|______)"#;
//
//  let gopher = Paragraph::new(gopher_art)
//    .style(Style::default().fg(Color::Green))
//    .alignment(Alignment::Center)
//    .block(Block::default());
//
//  let text_content = vec![
//    Line::from(Span::styled(
//      "╔══════════════════════════════════╗",
//      Style::default().fg(Color::Cyan),
//    )),
//    Line::from(vec![
//      Span::styled("║", Style::default().fg(Color::Cyan)),
//      Span::raw("                                  "),
//      Span::styled("║", Style::default().fg(Color::Cyan)),
//    ]),
//    Line::from(vec![
//      Span::styled("║", Style::default().fg(Color::Cyan)),
//      Span::styled(
//        "      🔐 GOPHERKEEPER      ",
//        Style::default()
//          .fg(Color::Yellow)
//          .add_modifier(Modifier::BOLD),
//      ),
//      Span::styled("║", Style::default().fg(Color::Cyan)),
//    ]),
//    Line::from(vec![
//      Span::styled("║", Style::default().fg(Color::Cyan)),
//      Span::raw("                                  "),
//      Span::styled("║", Style::default().fg(Color::Cyan)),
//    ]),
//    Line::from(vec![
//      Span::styled("║", Style::default().fg(Color::Cyan)),
//      Span::styled(
//        "  Secure Password Manager  ",
//        Style::default().fg(Color::Green),
//      ),
//      Span::styled("║", Style::default().fg(Color::Cyan)),
//    ]),
//    Line::from(vec![
//      Span::styled("║", Style::default().fg(Color::Cyan)),
//      Span::raw("                                  "),
//      Span::styled("║", Style::default().fg(Color::Cyan)),
//    ]),
//    Line::from(vec![
//      Span::styled("║", Style::default().fg(Color::Cyan)),
//      Span::styled(
//        "        TUI Edition        ",
//        Style::default().fg(Color::Magenta),
//      ),
//      Span::styled("║", Style::default().fg(Color::Cyan)),
//    ]),
//    Line::from(vec![
//      Span::styled("║", Style::default().fg(Color::Cyan)),
//      Span::raw("                                  "),
//      Span::styled("║", Style::default().fg(Color::Cyan)),
//    ]),
//    Line::from(Span::styled(
//      "╚══════════════════════════════════╝",
//      Style::default().fg(Color::Cyan),
//    )),
//  ];
//
//  let text_widget = Paragraph::new(text_content)
//    .alignment(Alignment::Center)
//    .block(Block::default());
//
//  f.render_widget(gopher, gopher_area);
//  f.render_widget(text_widget, text_area);
//
//  draw_helper_widget(f, app, menu_area);
//}
//
//fn draw_helper_widget(f: &mut Frame, app: &TuiApp, area: Rect) {
//  let chunks = Layout::default()
//    .direction(Direction::Horizontal)
//    .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
//    .split(area);
//
//  let menu_list_area = chunks[0];
//  let helper_area = chunks[1];
//
//  let items = vec![
//    ListItem::new("🔄  Sync secrets").style(Style::default().fg(Color::Green)),
//    ListItem::new("➕  Add secret").style(Style::default().fg(Color::Blue)),
//    ListItem::new("📋  View secrets").style(Style::default().fg(Color::Yellow)),
//    ListItem::new("👤  Logout").style(Style::default().fg(Color::Magenta)),
//    ListItem::new("🚪  Quit").style(Style::default().fg(Color::Red)),
//  ];
//
//  let list = List::new(items)
//    .block(
//      Block::default()
//        .borders(Borders::ALL)
//        .title(" Main Menu ")
//        .border_type(BorderType::Rounded)
//        .title_alignment(Alignment::Center),
//    )
//    .highlight_style(
//      Style::default()
//        .fg(Color::Black)
//        .bg(Color::Cyan)
//        .add_modifier(Modifier::BOLD),
//    )
//    .highlight_symbol("» ");
//
//  f.render_widget(list, menu_list_area);
//
//  let helper_text = vec![
//    Line::from(vec![
//      Span::styled("ℹ️ ", Style::default().fg(Color::Yellow)),
//      Span::styled(" Quick Help", Style::default().add_modifier(Modifier::BOLD)),
//    ]),
//    Line::from(""),
//    Line::from(vec![
//      Span::styled("• ", Style::default().fg(Color::Green)),
//      Span::styled("s", Style::default().fg(Color::Cyan)),
//      Span::raw(" - Sync secrets"),
//    ]),
//    Line::from(vec![
//      Span::styled("• ", Style::default().fg(Color::Blue)),
//      Span::styled("a", Style::default().fg(Color::Cyan)),
//      Span::raw(" - Add secret"),
//    ]),
//    Line::from(vec![
//      Span::styled("• ", Style::default().fg(Color::Yellow)),
//      Span::styled("v", Style::default().fg(Color::Cyan)),
//      Span::raw(" - View secrets"),
//    ]),
//    Line::from(vec![
//      Span::styled("• ", Style::default().fg(Color::Magenta)),
//      Span::styled("l", Style::default().fg(Color::Cyan)),
//      Span::raw(" - Logout"),
//    ]),
//    Line::from(vec![
//      Span::styled("• ", Style::default().fg(Color::Red)),
//      Span::styled("Ctrl + c", Style::default().fg(Color::Cyan)),
//      Span::raw(" - Quit"),
//    ]),
//    Line::from(""),
//    Line::from(Span::styled(
//      "─────────",
//      Style::default().fg(Color::DarkGray),
//    )),
//    Line::from(""),
//    Line::from(vec![
//      Span::styled("📊 ", Style::default().fg(Color::Green)),
//      Span::styled("Stats:", Style::default().add_modifier(Modifier::BOLD)),
//    ]),
//    Line::from(vec![
//      Span::raw("Secrets: "),
//      Span::styled(
//        format!("{}", app.secrets.len()),
//        Style::default()
//          .fg(Color::Magenta)
//          .add_modifier(Modifier::BOLD),
//      ),
//    ]),
//    Line::from(vec![
//      Span::raw("User: "),
//      Span::styled(
//        app.api.get_current_user().unwrap_or("Not logged in"),
//        Style::default().fg(Color::Cyan),
//      ),
//    ]),
//  ];
//
//  let helper_widget = Paragraph::new(helper_text)
//    .block(
//      Block::default()
//        .borders(Borders::ALL)
//        .title(" Information ")
//        .border_type(BorderType::Rounded)
//        .padding(ratatui::widgets::Padding::new(1, 1, 1, 1)),
//    )
//    .wrap(Wrap { trim: true })
//    .style(Style::default().fg(Color::Gray));
//
//  f.render_widget(helper_widget, helper_area);
//}

fn draw_menu(f: &mut Frame, app: &TuiApp, area: Rect) {
  // Center the entire menu horizontally
  let centered_area = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage(20),
      Constraint::Percentage(60),
      Constraint::Percentage(20),
    ])
    .split(area);

  let content_area = centered_area[1];

  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(14), Constraint::Min(6)])
    .split(content_area);

  let banner_area = chunks[0];
  let menu_area = chunks[1];

  let banner_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
    .split(banner_area);

  let gopher_area = banner_chunks[0];
  let text_area = banner_chunks[1];

  let gopher_art = r#"
     .----.
          _.'__    `.
      .--(#)(##)---/#\
    .' @          /###\
    :         ,   #####
     `-..__.-' _.-\###/
        `;_:    `"'
         .'"""""`.
        /,       ,\
       //         \\
       `-._______.-'
       ___`. | .'___
      (______|______)"#;

  let gopher = Paragraph::new(gopher_art)
    .style(Style::default().fg(Color::Green))
    .alignment(Alignment::Center)
    .block(Block::default());

  let text_content = vec![
    Line::from(Span::styled(
      "╔══════════════════════════════════╗",
      Style::default().fg(Color::DarkGray),
    )),
    Line::from(vec![
      Span::styled("║", Style::default().fg(Color::DarkGray)),
      Span::raw("                                  "),
      Span::styled("║", Style::default().fg(Color::DarkGray)),
    ]),
    Line::from(vec![
      Span::styled("║", Style::default().fg(Color::DarkGray)),
      Span::styled(
        "      GOPHERKEEPER      ",
        Style::default()
          .fg(Color::LightYellow)
          .add_modifier(Modifier::BOLD),
      ),
      Span::styled("║", Style::default().fg(Color::DarkGray)),
    ]),
    Line::from(vec![
      Span::styled("║", Style::default().fg(Color::DarkGray)),
      Span::raw("                                  "),
      Span::styled("║", Style::default().fg(Color::DarkGray)),
    ]),
    Line::from(vec![
      Span::styled("║", Style::default().fg(Color::DarkGray)),
      Span::styled(
        "  Secure Password Manager  ",
        Style::default().fg(Color::Green),
      ),
      Span::styled("║", Style::default().fg(Color::DarkGray)),
    ]),
    Line::from(vec![
      Span::styled("║", Style::default().fg(Color::DarkGray)),
      Span::raw("                                  "),
      Span::styled("║", Style::default().fg(Color::DarkGray)),
    ]),
    Line::from(vec![
      Span::styled("║", Style::default().fg(Color::DarkGray)),
      Span::styled(
        "        TUI Edition        ",
        Style::default().fg(Color::LightBlue),
      ),
      Span::styled("║", Style::default().fg(Color::DarkGray)),
    ]),
    Line::from(vec![
      Span::styled("║", Style::default().fg(Color::DarkGray)),
      Span::raw("                                  "),
      Span::styled("║", Style::default().fg(Color::DarkGray)),
    ]),
    Line::from(Span::styled(
      "╚══════════════════════════════════╝",
      Style::default().fg(Color::DarkGray),
    )),
  ];

  let text_widget = Paragraph::new(text_content)
    .alignment(Alignment::Center)
    .block(Block::default());

  f.render_widget(gopher, gopher_area);
  f.render_widget(text_widget, text_area);

  draw_helper_widget(f, app, menu_area);
}

fn draw_helper_widget(f: &mut Frame, app: &TuiApp, area: Rect) {
  // Center the menu and helper horizontally within the menu area
  let centered_area = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage(10),
      Constraint::Percentage(80),
      Constraint::Percentage(10),
    ])
    .split(area);

  let content_area = centered_area[1];

  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
    .split(content_area);

  let menu_list_area = chunks[0];
  let helper_area = chunks[1];

  let items = vec![
    ListItem::new("Sync secrets").style(Style::default().fg(Color::White)),
    ListItem::new("Add secret").style(Style::default().fg(Color::White)),
    ListItem::new("View secrets").style(Style::default().fg(Color::White)),
    ListItem::new("Logout").style(Style::default().fg(Color::White)),
    ListItem::new("Quit").style(Style::default().fg(Color::White)),
  ];

  let list = List::new(items)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .title(" Main Menu ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::DarkGray)),
    )
    .highlight_style(
      Style::default()
        .fg(Color::Black)
        .bg(Color::LightYellow)
        .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol("> ");

  f.render_widget(list, menu_list_area);

  let helper_text = vec![
    Line::from(vec![Span::styled(
      "Quick Help",
      Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD),
    )]),
    Line::from(""),
    Line::from(vec![
      Span::styled(
        "s",
        Style::default()
          .fg(Color::LightYellow)
          .add_modifier(Modifier::BOLD),
      ),
      Span::raw(" - Sync secrets"),
    ]),
    Line::from(vec![
      Span::styled(
        "a",
        Style::default()
          .fg(Color::LightYellow)
          .add_modifier(Modifier::BOLD),
      ),
      Span::raw(" - Add secret"),
    ]),
    Line::from(vec![
      Span::styled(
        "v",
        Style::default()
          .fg(Color::LightYellow)
          .add_modifier(Modifier::BOLD),
      ),
      Span::raw(" - View secrets"),
    ]),
    Line::from(vec![
      Span::styled(
        "l",
        Style::default()
          .fg(Color::LightYellow)
          .add_modifier(Modifier::BOLD),
      ),
      Span::raw(" - Logout"),
    ]),
    Line::from(vec![
      Span::styled(
        "Ctrl+c",
        Style::default()
          .fg(Color::LightRed)
          .add_modifier(Modifier::BOLD),
      ),
      Span::raw(" - Quit"),
    ]),
    Line::from(""),
    Line::from(Span::styled(
      "──────────",
      Style::default().fg(Color::DarkGray),
    )),
    Line::from(""),
    Line::from(vec![Span::styled(
      "Stats",
      Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD),
    )]),
    Line::from(vec![
      Span::styled("• ", Style::default().fg(Color::DarkGray)),
      Span::raw("Secrets: "),
      Span::styled(
        format!("{}", app.secrets.len()),
        Style::default()
          .fg(Color::LightGreen)
          .add_modifier(Modifier::BOLD),
      ),
    ]),
    Line::from(vec![
      Span::styled("• ", Style::default().fg(Color::DarkGray)),
      Span::raw("User: "),
      Span::styled(
        app.api.get_current_user().unwrap_or("Not logged in"),
        Style::default().fg(Color::Cyan),
      ),
    ]),
  ];

  let helper_widget = Paragraph::new(helper_text)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .title(" Information ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::DarkGray)),
    )
    .wrap(Wrap { trim: true })
    .style(Style::default().fg(Color::White));

  f.render_widget(helper_widget, helper_area);
}

fn draw_secrets(f: &mut Frame, app: &TuiApp, area: Rect) {
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Min(3), Constraint::Length(3)])
    .split(area);

  let table_area = chunks[0];
  let help_area = chunks[1];
  let rows: Vec<Row> = app
    .secrets
    .iter()
    .enumerate()
    .map(|(i, s)| {
      let style = if i == app.selected {
        Style::default()
          .fg(Color::Black)
          .bg(Color::Cyan)
          .add_modifier(Modifier::BOLD)
      } else {
        Style::default()
      };

      Row::new(vec![s.id.clone(), s.secret_type.clone()]).style(style)
    })
    .collect();

  let table = Table::new(
    rows,
    [
      Constraint::Length(4),
      Constraint::Percentage(50),
      Constraint::Percentage(50),
    ],
  )
  .block(Block::default().borders(Borders::ALL).title("Secrets"));

  f.render_widget(table, table_area);

  let help_text = if !app.secrets.is_empty() {
    Line::from(vec![
      Span::styled("↑/↓", Style::default().add_modifier(Modifier::BOLD)),
      Span::raw(" Navigate • "),
      Span::styled("c", Style::default().add_modifier(Modifier::BOLD)),
      Span::raw(" Copy • "),
      Span::styled("d", Style::default().add_modifier(Modifier::BOLD)),
      Span::raw(" Delete • "),
      Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD)),
      Span::raw(" Back • "),
      Span::styled("l", Style::default().add_modifier(Modifier::BOLD)),
      Span::raw(" Logout"),
    ])
  } else {
    Line::from("No secrets found. Press 'ESC' to go back.")
  };

  let help_widget = Paragraph::new(help_text)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded),
    )
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::Gray));

  f.render_widget(help_widget, help_area);
}

fn draw_add_secret(f: &mut Frame, app: &TuiApp, area: Rect) {
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Min(5), Constraint::Length(4)])
    .split(area);

  let input_area = chunks[0];
  let help_area = chunks[1];

  let input_text = match app.add_step {
    AddStep::Type => format!("Type: {}", app.add_type),
    AddStep::Data => format!("Data: {}", app.add_data),
  };

  let display_text = if app.input_mode == InputMode::Editing {
    match app.add_step {
      AddStep::Type => format!("Type: {}{}", app.add_type, "█"),
      AddStep::Data => format!("Data: {}{}", app.add_data, "█"),
    }
  } else {
    input_text.clone()
  };

  let title = match app.add_step {
    AddStep::Type => "Add Secret - Enter Type",
    AddStep::Data => "Add Secret - Enter Data",
  };

  let input_widget = Paragraph::new(display_text)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_type(BorderType::Rounded),
    )
    .style(match app.input_mode {
      InputMode::Editing => Style::default().fg(Color::Yellow),
      InputMode::Normal => Style::default(),
    })
    .wrap(Wrap { trim: true })
    .alignment(Alignment::Left);

  f.render_widget(input_widget, input_area);

  let help_text = if app.input_mode == InputMode::Editing {
    Line::from(vec![
      Span::styled("TAB", Style::default().add_modifier(Modifier::BOLD)),
      Span::raw(" Switch field • "),
      Span::styled("ENTER", Style::default().add_modifier(Modifier::BOLD)),
      Span::raw(" Submit • "),
      Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD)),
      Span::raw(" Cancel"),
    ])
  } else {
    Line::from(vec![
      Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
      Span::raw(" Edit • "),
      Span::styled("TAB", Style::default().add_modifier(Modifier::BOLD)),
      Span::raw(" Switch field • "),
      Span::styled("ENTER", Style::default().add_modifier(Modifier::BOLD)),
      Span::raw(" Submit • "),
      Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD)),
      Span::raw(" Back to menu"),
    ])
  };

  let field_indicator = match app.add_step {
    AddStep::Type => "(currently editing: Type field)",
    AddStep::Data => "(currently editing: Data field)",
  };

  let help_widget = Paragraph::new(vec![
    Line::from(help_text),
    Line::from(""),
    Line::from(Span::styled(
      field_indicator,
      Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::ITALIC),
    )),
  ])
  .block(
    Block::default()
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded),
  )
  .alignment(Alignment::Center)
  .style(Style::default().fg(Color::Gray));

  f.render_widget(help_widget, help_area);
}

fn draw_auth(f: &mut Frame, app: &TuiApp, area: Rect) {
  let title = match app.screen {
    Screen::Login => "Login",
    Screen::Register => "Register",
    _ => unreachable!(),
  };

  let centered_area = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage(30),
      Constraint::Percentage(40),
      Constraint::Percentage(30),
    ])
    .split(area);

  let content_area = centered_area[1];

  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Min(5), Constraint::Length(3)])
    .split(content_area);

  let active = Style::default()
    .fg(Color::LightYellow)
    .add_modifier(Modifier::BOLD);

  let username_display = if app.login_step == LoginStep::Username {
    format!("{}█", app.username)
  } else {
    app.username.clone()
  };

  let username_box = if app.username.is_empty() && app.login_step != LoginStep::Username {
    Paragraph::new(" username ")
      .style(Style::default().fg(Color::DarkGray))
      .block(
        Block::default()
          .borders(Borders::ALL)
          .border_style(Style::default().fg(Color::DarkGray))
          .title(" Username ")
          .title_alignment(Alignment::Center),
      )
  } else {
    Paragraph::new(username_display)
      .style(if app.login_step == LoginStep::Username {
        Style::default().fg(Color::LightYellow)
      } else {
        Style::default().fg(Color::White)
      })
      .block(
        Block::default()
          .borders(Borders::ALL)
          .border_style(if app.login_step == LoginStep::Username {
            Style::default().fg(Color::LightYellow)
          } else {
            Style::default().fg(Color::DarkGray)
          })
          .title(" Username ")
          .title_alignment(Alignment::Center),
      )
  };

  let password_mask = "*".repeat(app.password.len());
  let password_display = if app.login_step == LoginStep::Password {
    format!("{}█", password_mask)
  } else {
    password_mask
  };

  let password_box = if app.password.is_empty() && app.login_step != LoginStep::Password {
    Paragraph::new(" password ")
      .style(Style::default().fg(Color::DarkGray))
      .block(
        Block::default()
          .borders(Borders::ALL)
          .border_style(Style::default().fg(Color::DarkGray))
          .title(" Password ")
          .title_alignment(Alignment::Center),
      )
  } else {
    Paragraph::new(password_display)
      .style(if app.login_step == LoginStep::Password {
        Style::default().fg(Color::LightYellow)
      } else {
        Style::default().fg(Color::White)
      })
      .block(
        Block::default()
          .borders(Borders::ALL)
          .border_style(if app.login_step == LoginStep::Password {
            Style::default().fg(Color::LightYellow)
          } else {
            Style::default().fg(Color::DarkGray)
          })
          .title(" Password ")
          .title_alignment(Alignment::Center),
      )
  };

  let mut input_boxes = vec![username_box, password_box];

  if app.screen == Screen::Register {
    let confirm_mask = "*".repeat(app.confirm_password.len());
    let confirm_display = if app.login_step == LoginStep::ConfirmPassword {
      format!("{}█", confirm_mask)
    } else {
      confirm_mask
    };

    let confirm_box =
      if app.confirm_password.is_empty() && app.login_step != LoginStep::ConfirmPassword {
        Paragraph::new(" confirm password ")
          .style(Style::default().fg(Color::DarkGray))
          .block(
            Block::default()
              .borders(Borders::ALL)
              .border_style(Style::default().fg(Color::DarkGray))
              .title(" Confirm ")
              .title_alignment(Alignment::Center),
          )
      } else {
        Paragraph::new(confirm_display)
          .style(if app.login_step == LoginStep::ConfirmPassword {
            Style::default().fg(Color::LightYellow)
          } else {
            Style::default().fg(Color::White)
          })
          .block(
            Block::default()
              .borders(Borders::ALL)
              .border_style(if app.login_step == LoginStep::ConfirmPassword {
                Style::default().fg(Color::LightYellow)
              } else {
                Style::default().fg(Color::DarkGray)
              })
              .title(" Confirm ")
              .title_alignment(Alignment::Center),
          )
      };

    input_boxes.push(confirm_box);
  }

  let box_height = 3;
  let spacing = 1;
  let total_height =
    (box_height * input_boxes.len() as u16) + (spacing * (input_boxes.len() as u16 - 1));

  let box_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(
      std::iter::once(Constraint::Length((chunks[0].height - total_height) / 2))
        .chain(
          input_boxes
            .iter()
            .flat_map(|_| [Constraint::Length(box_height), Constraint::Length(spacing)]),
        )
        .take(input_boxes.len() * 2 + 1)
        .collect::<Vec<_>>(),
    )
    .split(chunks[0]);

  for (i, box_widget) in input_boxes.into_iter().enumerate() {
    f.render_widget(box_widget, box_layout[i * 2 + 1]);
  }

  let help_text = Line::from(vec![
    Span::styled(
      "Ctrl+r",
      Style::default()
        .fg(Color::LightYellow)
        .add_modifier(Modifier::BOLD),
    ),
    Span::raw(" register • "),
    Span::styled(
      "Ctrl+l",
      Style::default()
        .fg(Color::LightYellow)
        .add_modifier(Modifier::BOLD),
    ),
    Span::raw(" login • "),
    Span::styled(
      "Tab",
      Style::default()
        .fg(Color::LightYellow)
        .add_modifier(Modifier::BOLD),
    ),
    Span::raw(" next • "),
    Span::styled(
      "Enter",
      Style::default()
        .fg(Color::LightGreen)
        .add_modifier(Modifier::BOLD),
    ),
    Span::raw(" submit • "),
    Span::styled(
      "Ctrl+c",
      Style::default()
        .fg(Color::LightRed)
        .add_modifier(Modifier::BOLD),
    ),
    Span::raw(" quit"),
  ]);

  let help_widget = Paragraph::new(help_text)
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::White));

  f.render_widget(help_widget, chunks[1]);
}

fn draw_notification(f: &mut Frame, app: &TuiApp) {
  let Some((msg, _)) = &app.notification else {
    return;
  };

  let (border, bg) = if msg.starts_with("SUCCESS") {
    (Color::Green, Color::DarkGray)
  } else if msg.starts_with("ERROR") {
    (Color::Red, Color::Black)
  } else {
    (Color::Yellow, Color::Black)
  };

  let area = centered_rect(60, 20, f.size());

  let block = Block::default()
    .borders(Borders::ALL)
    .border_type(BorderType::Thick)
    .border_style(Style::default().fg(border))
    .style(Style::default().bg(bg))
    .title("Notification");

  let text = Paragraph::new(msg.clone())
    .alignment(Alignment::Center)
    .style(
      Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD),
    )
    .wrap(Wrap { trim: true });

  f.render_widget(block, area);
  f.render_widget(text, area);
}

pub fn draw_master_password(f: &mut Frame, app: &TuiApp) {
  let size = f.size();

  let compact_area = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(10), Constraint::Min(0)])
    .split(size);

  let centered_area = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage(30),
      Constraint::Percentage(40),
      Constraint::Percentage(30),
    ])
    .split(compact_area[0]);

  let area = centered_area[1];

  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(5), Constraint::Length(4)])
    .split(area);

  let info_text = vec![
    Line::from(vec![Span::styled(
      "✓ You are already logged in.",
      Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD),
    )]),
    Line::from(""),
    Line::from(vec![
      Span::styled("Enter ", Style::default().fg(Color::White)),
      Span::styled(
        "MASTER PASSWORD",
        Style::default()
          .fg(Color::Yellow)
          .add_modifier(Modifier::BOLD),
      ),
      Span::styled(" to decrypt secrets.", Style::default().fg(Color::White)),
    ]),
  ];

  let info = Paragraph::new(info_text)
    .wrap(Wrap { trim: true })
    .block(
      Block::default()
        .borders(Borders::ALL)
        .title(" Session Active ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Green)),
    )
    .alignment(Alignment::Center);

  f.render_widget(info, chunks[0]);

  let password_section = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(1), Constraint::Length(3)])
    .split(chunks[1]);

  let mode_text = match app.input_mode {
    InputMode::Normal => Span::styled(
      "LOCKED",
      Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    ),
    InputMode::Editing => Span::styled(
      "EDITING",
      Style::default()
        .fg(Color::LightGreen)
        .add_modifier(Modifier::BOLD),
    ),
  };

  let helper = if app.input_mode == InputMode::Normal {
    Line::from(vec![
      Span::styled("Mode: ", Style::default().fg(Color::White)),
      mode_text,
      Span::styled("  •  ", Style::default().fg(Color::White)),
      Span::styled(
        "E",
        Style::default()
          .fg(Color::LightYellow)
          .add_modifier(Modifier::BOLD),
      ),
      Span::styled(": edit  ", Style::default().fg(Color::White)),
      Span::styled(
        "Ctrl+c",
        Style::default()
          .fg(Color::LightRed)
          .add_modifier(Modifier::BOLD),
      ),
      Span::styled(": quit", Style::default().fg(Color::White)),
    ])
  } else {
    Line::from(vec![
      Span::styled("Mode: ", Style::default().fg(Color::White)),
      mode_text,
      Span::styled("  •  ", Style::default().fg(Color::White)),
      Span::styled(
        "Enter",
        Style::default()
          .fg(Color::LightGreen)
          .add_modifier(Modifier::BOLD),
      ),
      Span::styled(": confirm  ", Style::default().fg(Color::White)),
      Span::styled(
        "Esc",
        Style::default()
          .fg(Color::LightYellow)
          .add_modifier(Modifier::BOLD),
      ),
      Span::styled(": cancel", Style::default().fg(Color::White)),
    ])
  };

  let mode_indicator = Paragraph::new(helper).block(Block::default());

  f.render_widget(mode_indicator, password_section[0]);

  let masked = "*".repeat(app.password.len());

  let display_password = if app.password.is_empty() && app.input_mode == InputMode::Normal {
    String::from(" your password ")
  } else if app.password.is_empty() && app.input_mode == InputMode::Editing {
    String::from(" ")
  } else {
    masked
  };

  let input = if app.password.is_empty() && app.input_mode == InputMode::Normal {
    Paragraph::new(display_password)
      .style(Style::default().fg(Color::DarkGray))
      .block(
        Block::default()
          .borders(Borders::ALL)
          .border_style(Style::default().fg(Color::DarkGray)),
      )
  } else {
    Paragraph::new(display_password)
      .style(Style::default().fg(Color::Yellow))
      .block(
        Block::default()
          .borders(Borders::ALL)
          .border_style(Style::default().fg(Color::Yellow)),
      )
  };

  f.render_widget(input, password_section[1]);
  if app.input_mode == InputMode::Editing {
    let cursor_x = password_section[1].x + 1 + app.password.len() as u16;
    let cursor_y = password_section[1].y + 1;
    f.set_cursor(cursor_x, cursor_y);
  }
}
