use crate::tui::app::{AddField, AddKind, InputMode, LoginStep, Screen, TuiApp};
use ratatui::{
  Frame,
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Row, Table, Wrap},
};

use crate::core::models::SecretPayload;

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

pub fn draw_menu(f: &mut Frame, app: &TuiApp, area: Rect) {
  let vertical = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(5), Constraint::Min(10)])
    .split(area);

  let header_area = vertical[0];
  let body_area = vertical[1];

  let header = Paragraph::new(vec![
    Line::from(Span::styled(
      "GOPHERKEEPER",
      Style::default()
        .fg(Color::LightYellow)
        .add_modifier(Modifier::BOLD),
    )),
    Line::from(Span::styled(
      "Secure Vault • TUI Edition",
      Style::default().fg(Color::Green),
    )),
  ])
  .alignment(Alignment::Center)
  .block(
    Block::default()
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .border_style(Style::default().fg(Color::DarkGray)),
  );

  f.render_widget(header, header_area);

  let horizontal = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
    .split(body_area);

  let menu_area = horizontal[0];
  let info_area = horizontal[1];

  draw_menu_list(f, app, menu_area);
  draw_info_panel(f, app, info_area);
}

fn draw_menu_list(f: &mut Frame, app: &TuiApp, area: Rect) {
  let items = [
    "Sync secrets",
    "Add secret",
    "View secrets",
    "Logout",
    "Quit",
  ];

  let rows: Vec<ListItem> = items
    .iter()
    .enumerate()
    .map(|(i, item)| {
      let style = if i == app.selected {
        Style::default()
          .fg(Color::Black)
          .bg(Color::LightYellow)
          .add_modifier(Modifier::BOLD)
      } else {
        Style::default().fg(Color::White)
      };

      ListItem::new(*item).style(style)
    })
    .collect();

  let list = List::new(rows).block(
    Block::default()
      .borders(Borders::ALL)
      .title(" Main Menu ")
      .title_alignment(Alignment::Center)
      .border_type(BorderType::Rounded)
      .border_style(Style::default().fg(Color::DarkGray)),
  );

  f.render_widget(list, area);
}

fn draw_info_panel(f: &mut Frame, app: &TuiApp, area: Rect) {
  let label = Style::default()
    .fg(Color::LightYellow)
    .add_modifier(Modifier::BOLD);

  let user = app.api.get_current_user().unwrap_or("Not logged in");

  let lines = vec![
    Line::from(vec![
      Span::styled("User: ", label),
      Span::styled(user, Style::default().fg(Color::Cyan)),
    ]),
    Line::from(vec![
      Span::styled("Secrets: ", label),
      Span::styled(
        format!("{}", app.secrets.len()),
        Style::default()
          .fg(Color::LightGreen)
          .add_modifier(Modifier::BOLD),
      ),
    ]),
    Line::from(""),
    Line::from(Span::styled(
      "Quick Keys",
      Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD),
    )),
    Line::from(""),
    Line::from(vec![Span::styled("s ", label), Span::raw("Sync")]),
    Line::from(vec![Span::styled("a ", label), Span::raw("Add")]),
    Line::from(vec![Span::styled("v ", label), Span::raw("View")]),
    Line::from(vec![Span::styled("l ", label), Span::raw("Logout")]),
    Line::from(vec![
      Span::styled(
        "Ctrl+c ",
        Style::default()
          .fg(Color::LightRed)
          .add_modifier(Modifier::BOLD),
      ),
      Span::raw("Quit"),
    ]),
  ];

  let panel = Paragraph::new(lines)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .title(" Information ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray)),
    )
    .wrap(Wrap { trim: true })
    .style(Style::default().fg(Color::White));

  f.render_widget(panel, area);
}

pub fn draw_secrets(f: &mut Frame, app: &TuiApp, area: Rect) {
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Percentage(55),
      Constraint::Percentage(35),
      Constraint::Length(3),
    ])
    .split(area);

  let header_style = Style::default()
    .fg(Color::LightYellow)
    .add_modifier(Modifier::BOLD);

  let header = Row::new(vec!["Title", "Kind"])
    .style(header_style)
    .bottom_margin(1);

  let rows: Vec<Row> = app
    .secrets
    .iter()
    .enumerate()
    .map(|(i, s)| {
      let kind = match &s.payload {
        SecretPayload::Password { .. } => "Password",
        SecretPayload::Note { .. } => "Note",
        SecretPayload::Card { .. } => "Card",
      };

      let title = match &s.payload {
        SecretPayload::Password { title, .. } => title,
        SecretPayload::Note { title, .. } => title,
        SecretPayload::Card { title, .. } => title,
      };

      let style = if i == app.selected {
        Style::default()
          .fg(Color::Black)
          .bg(Color::LightYellow)
          .add_modifier(Modifier::BOLD)
      } else {
        Style::default().fg(Color::White)
      };

      Row::new(vec![title.clone(), kind.to_string()]).style(style)
    })
    .collect();

  let table = Table::new(
    rows,
    [Constraint::Percentage(70), Constraint::Percentage(30)],
  )
  .header(header)
  .block(
    Block::default()
      .borders(Borders::ALL)
      .title(" Secrets ")
      .title_alignment(Alignment::Center)
      .border_type(BorderType::Rounded)
      .border_style(Style::default().fg(Color::DarkGray)),
  );

  f.render_widget(table, chunks[0]);

  if let Some(secret) = app.secrets.get(app.selected) {
    let mut lines: Vec<Line> = Vec::new();

    let label_style = Style::default()
      .fg(Color::LightYellow)
      .add_modifier(Modifier::BOLD);

    let fields = app.current_secret_fields();

    for (i, (label, value)) in fields.iter().enumerate() {
      let style = if i == app.detail_selected {
        Style::default()
          .fg(Color::Black)
          .bg(Color::LightYellow)
          .add_modifier(Modifier::BOLD)
      } else {
        Style::default().fg(Color::White)
      };

      lines.push(Line::from(vec![
        Span::styled(format!("{}: ", label), Style::default().fg(Color::DarkGray)),
        Span::styled(value.clone(), style),
      ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
      Span::styled("Created: ", label_style),
      Span::raw(secret.created_at.as_deref().unwrap_or("-")),
    ]));
    lines.push(Line::from(vec![
      Span::styled("Updated: ", label_style),
      Span::raw(secret.updated_at.as_deref().unwrap_or("-")),
    ]));

    let detail = Paragraph::new(lines)
      .block(
        Block::default()
          .borders(Borders::ALL)
          .title(" Secret Info ")
          .title_alignment(Alignment::Center)
          .border_type(BorderType::Rounded)
          .border_style(Style::default().fg(Color::DarkGray)),
      )
      .wrap(Wrap { trim: true });

    f.render_widget(detail, chunks[1]);
  }

let help = Line::from(vec![
    Span::styled(
        "↑/↓ ",
        Style::default()
            .fg(Color::LightYellow)
            .add_modifier(Modifier::BOLD),
    ),
    Span::raw("Navigate Secrets • "),
    Span::styled(
        "←/→ ",
        Style::default()
            .fg(Color::LightYellow)
            .add_modifier(Modifier::BOLD),
    ),
    Span::raw("Navigate Fields • "),
    Span::styled(
        "c ",
        Style::default()
            .fg(Color::LightYellow)
            .add_modifier(Modifier::BOLD),
    ),
    Span::raw("Copy • "),
    Span::styled(
        "d ",
        Style::default()
            .fg(Color::LightYellow)
            .add_modifier(Modifier::BOLD),
    ),
    Span::raw("Delete • "),
    Span::styled(
        "ESC ",
        Style::default()
            .fg(Color::LightYellow)
            .add_modifier(Modifier::BOLD),
    ),
    Span::raw("Back • "),
    Span::styled(
        "l ",
        Style::default()
            .fg(Color::LightYellow)
            .add_modifier(Modifier::BOLD),
    ),
    Span::raw("Logout"),
]);


  let help_widget = Paragraph::new(help)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray)),
    )
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::White));

  f.render_widget(help_widget, chunks[2]);
}

pub fn draw_add_secret(f: &mut Frame, app: &TuiApp, area: Rect) {
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Min(8), Constraint::Length(4)])
    .split(area);

  let highlight = |active: bool| {
    if active {
      Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD)
    } else {
      Style::default()
    }
  };

  let make_field = |label: &str, val: &str, field: AddField| {
    let display = if val.is_empty() && app.add_field != field {
      format!(" {} ", label)
    } else if app.add_field == field {
      format!("{}█", val)
    } else {
      val.to_string()
    };

    let block = Block::default()
      .borders(Borders::ALL)
      .title(label.to_string())
      .title_alignment(ratatui::layout::Alignment::Center)
      .border_style(if app.add_field == field {
        Style::default()
          .fg(Color::LightYellow)
          .add_modifier(Modifier::BOLD)
      } else {
        Style::default().fg(Color::DarkGray)
      })
      .border_type(BorderType::Rounded);

    (
      field,
      Paragraph::new(display)
        .style(highlight(app.add_field == field))
        .block(block),
    )
  };

  let mut field_widgets = Vec::new();

  field_widgets.push(make_field(
    "Kind: Password/Note/Card",
    match app.add_kind {
      AddKind::Password => "Password",
      AddKind::Note => "Note",
      AddKind::Card => "Card",
    },
    AddField::Kind,
  ));

  match app.add_kind {
    AddKind::Password => {
      field_widgets.push(make_field("Title", &app.title, AddField::Title));
      field_widgets.push(make_field("Login", &app.field1, AddField::Field1));
      field_widgets.push(make_field("Password", &app.field2, AddField::Field2));
      field_widgets.push(make_field("URL", &app.field3, AddField::Field3));
    }
    AddKind::Note => {
      field_widgets.push(make_field("Title", &app.title, AddField::Title));
      field_widgets.push(make_field("Content", &app.field1, AddField::Field1));
    }
    AddKind::Card => {
      field_widgets.push(make_field("Title", &app.title, AddField::Title));
      field_widgets.push(make_field("Holder", &app.field1, AddField::Field1));
      field_widgets.push(make_field("Number", &app.field2, AddField::Field2));
      field_widgets.push(make_field("Expiry", &app.field3, AddField::Field3));
      field_widgets.push(make_field("CVV", &app.field4, AddField::Field4));
    }
  }

  let field_count = field_widgets.len() as u16;
  let spacing = 1;
  let block_height = 3;
  let total_height = (block_height + spacing) * field_count - spacing;

  let field_chunks: Vec<Rect> = Layout::default()
    .direction(Direction::Vertical)
    .constraints(
      std::iter::once(Constraint::Length((chunks[0].height - total_height) / 2))
        .chain(
          std::iter::repeat_with(|| Constraint::Length(block_height)).take(field_count as usize),
        )
        .collect::<Vec<_>>(),
    )
    .split(chunks[0])
    .iter()
    .skip(1)
    .copied()
    .collect();

  for ((_, widget), rect) in field_widgets.into_iter().zip(field_chunks) {
    f.render_widget(widget, rect);
  }

  let help = Paragraph::new(Line::from(vec![
    Span::styled("TAB ", Style::default().add_modifier(Modifier::BOLD)),
    Span::raw("Next field • "),
    Span::styled("←/→ ", Style::default().add_modifier(Modifier::BOLD)),
    Span::raw("Change Kind • "),
    Span::styled("ENTER ", Style::default().add_modifier(Modifier::BOLD)),
    Span::raw("Save • "),
    Span::styled("ESC ", Style::default().add_modifier(Modifier::BOLD)),
    Span::raw("Cancel"),
  ]))
  .block(
    Block::default()
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded),
  )
  .alignment(ratatui::layout::Alignment::Center)
  .style(Style::default().fg(Color::Gray));

  f.render_widget(help, chunks[1]);
}

fn draw_auth(f: &mut Frame, app: &TuiApp, area: Rect) {
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

pub fn draw_notification(f: &mut Frame, app: &TuiApp) {
  let Some((msg, _)) = &app.notification else {
    return;
  };

  let size = f.size();

  let width = msg.len() as u16 + 6;
  let height = 3;

  let area = Rect {
    x: size.width.saturating_sub(width + 2),
    y: 1,
    width,
    height,
  };

  let block = Block::default()
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)
    .border_style(
      Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD),
    )
    .style(Style::default().bg(Color::Black));

  let inner = block.inner(area);

  let text = Paragraph::new(msg.clone())
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true })
    .style(
      Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD),
    );

  f.render_widget(block, area);
  f.render_widget(text, inner);
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
