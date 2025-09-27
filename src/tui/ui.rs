use crate::tui::app::{App, AppState};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let size = f.size();

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(size);

    // Draw header
    draw_header(f, chunks[0], app);

    // Draw main content
    draw_main_content(f, chunks[1], app);

    // Draw footer
    draw_footer(f, chunks[2], app);

    // Draw popups if needed
    if app.show_help {
        draw_help_popup(f, app);
    }

    match app.state {
        AppState::AddVariable => draw_add_variable_popup(f, app),
        AppState::EditVariable => draw_edit_variable_popup(f, app),
        AppState::ConfirmDelete => draw_confirm_delete_popup(f, app),
        _ => {}
    }
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    // Create a colorful header with gradient-like effect
    let header_spans = vec![
        Span::styled("🔧 ", Style::default().fg(Color::Yellow)),
        Span::styled("env", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        Span::styled("Match", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(" - Current Environment: ", Style::default().fg(Color::White)),
        Span::styled(&app.current_environment, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
    ];
    
    let header = Paragraph::new(Line::from(header_spans))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    " Environment Manager ",
                    Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD)
                ))
                .border_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(header, area);
}

fn draw_main_content(f: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    // Draw environments list
    draw_environments_list(f, chunks[0], app);

    // Draw variables list
    draw_variables_list(f, chunks[1], app);
}

fn draw_environments_list(f: &mut Frame, area: Rect, app: &mut App) {
    let items: Vec<ListItem> = app
        .environments
        .iter()
        .enumerate()
        .map(|(i, env)| {
            let style = if i == app.selected_env_index && app.state == AppState::EnvironmentList {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else if env == &app.current_environment {
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::LightBlue)
            };

            let (prefix, prefix_style) = if env == &app.current_environment {
                ("▶ ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            } else {
                ("  ", Style::default())
            };

            ListItem::new(Line::from(vec![
                Span::styled(prefix, prefix_style),
                Span::styled(env, style),
            ]))
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.selected_env_index));

    let title = if app.state == AppState::EnvironmentList {
        Span::styled(
            " 📁 Environments (Active) ",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        )
    } else {
        Span::styled(
            " 📁 Environments ",
            Style::default().fg(Color::LightCyan)
        )
    };

    let border_style = if app.state == AppState::EnvironmentList {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("❯ ");

    f.render_stateful_widget(list, area, &mut state);
}

fn draw_variables_list(f: &mut Frame, area: Rect, app: &mut App) {
    let items: Vec<ListItem> = app
        .variables
        .iter()
        .enumerate()
        .map(|(i, var)| {
            // Color the key and value differently
            let key_style = if i == app.selected_var_index && app.state == AppState::VariableList {
                Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD)
            };
            
            let equals_style = if i == app.selected_var_index && app.state == AppState::VariableList {
                Style::default().fg(Color::Black).bg(Color::Yellow)
            } else {
                Style::default().fg(Color::White)
            };
            
            let value_style = if i == app.selected_var_index && app.state == AppState::VariableList {
                Style::default().fg(Color::Black).bg(Color::Yellow)
            } else {
                Style::default().fg(Color::LightGreen)
            };

            ListItem::new(Line::from(vec![
                Span::styled(&var.key, key_style),
                Span::styled("=", equals_style),
                Span::styled(&var.value, value_style),
            ]))
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.selected_var_index));

    let title = if app.state == AppState::VariableList {
        Span::styled(
            " 🔧 Variables (Active) ",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        )
    } else {
        Span::styled(
            " 🔧 Variables ",
            Style::default().fg(Color::LightCyan)
        )
    };

    let border_style = if app.state == AppState::VariableList {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("❯ ");

    f.render_stateful_widget(list, area, &mut state);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let mut lines = vec![];

    // Status messages with enhanced colors
    if !app.status_message.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("✅ ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(&app.status_message, Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)),
        ]));
    }

    if !app.error_message.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("❌ ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled(&app.error_message, Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)),
        ]));
    }

    // Enhanced help text with colors
    let help_spans = match app.state {
        AppState::EnvironmentList => vec![
            Span::styled("Tab", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": Variables | "),
            Span::styled("↑↓", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": Navigate | "),
            Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(": Switch | "),
            Span::styled("q", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(": Quit | "),
            Span::styled("h", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            Span::raw(": Help"),
        ],
        AppState::VariableList => vec![
            Span::styled("Tab", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": Environments | "),
            Span::styled("↑↓", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": Navigate | "),
            Span::styled("a", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(": Add | "),
            Span::styled("e", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            Span::raw(": Edit | "),
            Span::styled("d", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(": Delete | "),
            Span::styled("F5", Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD)),
            Span::raw(": Refresh | "),
            Span::styled("q", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(": Quit | "),
            Span::styled("h", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            Span::raw(": Help"),
        ],
        AppState::AddVariable => vec![
            Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(": Confirm (Key first, then Value) | "),
            Span::styled("Esc", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(": Cancel"),
        ],
        AppState::EditVariable => vec![
            Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(": Save | "),
            Span::styled("Esc", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(": Cancel"),
        ],
        AppState::ConfirmDelete => vec![
            Span::styled("y", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(": Yes | "),
            Span::styled("n", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(": No | "),
            Span::styled("Esc", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(": Cancel"),
        ],
    };
    lines.push(Line::from(help_spans));

    let footer = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    " Status & Help ",
                    Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD)
                ))
                .border_style(Style::default().fg(Color::DarkGray))
        )
        .wrap(Wrap { trim: true });

    f.render_widget(footer, area);
}

fn draw_add_variable_popup(f: &mut Frame, app: &App) {
    let size = f.size();
    let popup_area = centered_rect(60, 20, size);

    f.render_widget(Clear, popup_area);

    let title = if app.input_key.is_empty() {
        Span::styled(
            " 🔧 Add Variable - Enter Key ",
            Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)
        )
    } else {
        Span::styled(
            " 🔧 Add Variable - Enter Value ",
            Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD)
        )
    };

    let content_lines = if app.input_key.is_empty() {
        vec![
            Line::from(vec![
                Span::styled("Key: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(&app.input_buffer, Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
                Span::styled("█", Style::default().fg(Color::White)), // Cursor
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Press Enter to continue to value input",
                Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC)
            )),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::styled("Key: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(&app.input_key, Style::default().fg(Color::LightGreen)),
            ]),
            Line::from(vec![
                Span::styled("Value: ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
                Span::styled(&app.input_buffer, Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
                Span::styled("█", Style::default().fg(Color::White)), // Cursor
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Press Enter to save variable",
                Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC)
            )),
        ]
    };

    let popup = Paragraph::new(content_lines)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(popup, popup_area);
}

fn draw_edit_variable_popup(f: &mut Frame, app: &App) {
    let size = f.size();
    let popup_area = centered_rect(60, 20, size);

    f.render_widget(Clear, popup_area);

    let content_lines = vec![
        Line::from(vec![
            Span::styled("Key: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(&app.input_key, Style::default().fg(Color::LightGreen)),
        ]),
        Line::from(vec![
            Span::styled("Value: ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            Span::styled(&app.input_buffer, Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
            Span::styled("█", Style::default().fg(Color::White)), // Cursor
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter to save changes",
            Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC)
        )),
    ];

    let popup = Paragraph::new(content_lines)
        .block(
            Block::default()
                .title(Span::styled(
                    " ✏️ Edit Variable ",
                    Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD)
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(popup, popup_area);
}

fn draw_confirm_delete_popup(f: &mut Frame, app: &App) {
    let size = f.size();
    let popup_area = centered_rect(50, 15, size);

    f.render_widget(Clear, popup_area);

    let var_name = app
        .variables
        .get(app.selected_var_index)
        .map(|v| v.key.as_str())
        .unwrap_or("unknown");

    let content_lines = vec![
        Line::from(vec![
            Span::styled("Delete variable ", Style::default().fg(Color::White)),
            Span::styled("'", Style::default().fg(Color::Gray)),
            Span::styled(var_name, Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)),
            Span::styled("'", Style::default().fg(Color::Gray)),
            Span::styled("?", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("y", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(": Yes", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("n", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled(": No", Style::default().fg(Color::White)),
        ]),
    ];

    let popup = Paragraph::new(content_lines)
        .block(
            Block::default()
                .title(Span::styled(
                    " ⚠️ Confirm Delete ",
                    Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(popup, popup_area);
}

fn draw_help_popup(f: &mut Frame, _app: &App) {
    let size = f.size();
    let popup_area = centered_rect(80, 70, size);

    f.render_widget(Clear, popup_area);

    let help_lines = vec![
        Line::from(vec![
            Span::styled("🔧 ", Style::default().fg(Color::Yellow)),
            Span::styled("env", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            Span::styled("Match", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(" - Environment Variable Manager", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(Span::styled("NAVIGATION:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED))),
        Line::from(vec![
            Span::styled("  Tab", Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD)),
            Span::raw("                 Switch between Environments and Variables panels"),
        ]),
        Line::from(vec![
            Span::styled("  ↑/↓ or k/j", Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD)),
            Span::raw("         Navigate up/down in lists"),
        ]),
        Line::from(vec![
            Span::styled("  Enter", Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)),
            Span::raw("              Switch to selected environment"),
        ]),
        Line::from(vec![
            Span::styled("  q", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)),
            Span::raw("                  Quit application"),
        ]),
        Line::from(""),
        Line::from(Span::styled("VARIABLE MANAGEMENT:", Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD | Modifier::UNDERLINED))),
        Line::from(vec![
            Span::styled("  a", Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)),
            Span::raw("                  Add new variable"),
        ]),
        Line::from(vec![
            Span::styled("  e", Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD)),
            Span::raw("                  Edit selected variable"),
        ]),
        Line::from(vec![
            Span::styled("  d", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)),
            Span::raw(" or "),
            Span::styled("Delete", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)),
            Span::raw("        Delete selected variable"),
        ]),
        Line::from(vec![
            Span::styled("  F5", Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD)),
            Span::raw("                 Refresh variable list"),
        ]),
        Line::from(""),
        Line::from(Span::styled("POPUP CONTROLS:", Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD | Modifier::UNDERLINED))),
        Line::from(vec![
            Span::styled("  Enter", Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)),
            Span::raw("              Confirm action"),
        ]),
        Line::from(vec![
            Span::styled("  Esc", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)),
            Span::raw("                Cancel/Close"),
        ]),
        Line::from(""),
        Line::from(Span::styled("FEATURES:", Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED))),
        Line::from(vec![
            Span::styled("  • ", Style::default().fg(Color::Green)),
            Span::raw("Manage multiple environments"),
        ]),
        Line::from(vec![
            Span::styled("  • ", Style::default().fg(Color::Green)),
            Span::raw("Add, edit, and delete environment variables"),
        ]),
        Line::from(vec![
            Span::styled("  • ", Style::default().fg(Color::Green)),
            Span::raw("Switch between environments instantly"),
        ]),
        Line::from(vec![
            Span::styled("  • ", Style::default().fg(Color::Green)),
            Span::raw("Visual feedback for current environment"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Press "),
            Span::styled("'h'", Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD)),
            Span::raw(" or "),
            Span::styled("F1", Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD)),
            Span::raw(" to toggle this help, "),
            Span::styled("'Esc'", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)),
            Span::raw(" to close."),
        ]),
    ];

    let help_paragraph = Paragraph::new(help_lines)
        .block(
            Block::default()
                .title(Span::styled(
                    " 📖 Help & Controls ",
                    Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD)
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(help_paragraph, popup_area);
}

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
