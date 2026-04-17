use vibe_core::ipc::protocol::WorkerState;
use vibe_core::env::{detect_current_terminal, TerminalType};
use vibe_core::adapter::{TerminalAdapter, WezTermAdapter, TmuxAdapter};
use vibe_core::state::StateStore;
use futures::StreamExt;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Row, Table, TableState},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{self, Stdout};
use std::time::Duration;

pub async fn run_status_tui() -> anyhow::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal).await;

    // Restore terminal
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    
    res
}

struct App {
    states: Vec<WorkerState>,
    table_state: TableState,
    logs: String,
    selected_id: Option<String>,
    adapter: Box<dyn TerminalAdapter>,
}

impl App {
    fn new() -> anyhow::Result<Self> {
        let terminal_type = detect_current_terminal()
            .ok_or_else(|| anyhow::anyhow!("No supported terminal detected for TUI"))?;
        let adapter: Box<dyn TerminalAdapter> = match terminal_type {
            TerminalType::WezTerm => Box::new(WezTermAdapter),
            TerminalType::Tmux => Box::new(TmuxAdapter),
        };
        Ok(Self {
            states: Vec::new(),
            table_state: TableState::default(),
            logs: String::new(),
            selected_id: None,
            adapter,
        })
    }

    fn next(&mut self) {
        if self.states.is_empty() { return; }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.states.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.update_selected_id();
    }

    fn previous(&mut self) {
        if self.states.is_empty() { return; }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.states.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.update_selected_id();
    }

    fn update_selected_id(&mut self) {
        self.selected_id = self.table_state.selected().and_then(|i| self.states.get(i).map(|s| s.vibe_id.clone()));
    }

    fn focus_selected(&self) -> anyhow::Result<()> {
        if let Some(ref id) = self.selected_id {
            if let Some(state) = self.states.iter().find(|s| s.vibe_id == *id) {
                self.adapter.focus(&state.physical_id)?;
            }
        }
        Ok(())
    }

    fn kill_selected(&self) -> anyhow::Result<()> {
        if let Some(ref id) = self.selected_id {
            if let Some(state) = self.states.iter().find(|s| s.vibe_id == *id) {
                let _ = self.adapter.close(&state.vibe_id);
                let store = StateStore::new()?;
                store.remove_pane(&state.vibe_id)?;
            }
        }
        Ok(())
    }
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<()> {
    let mut app = App::new()?;
    let mut reader = event::EventStream::new();
    let store = StateStore::new()?;

    loop {
        // Poll state from local store
        app.states = store.list_active_panes()?;
        // Keep selection if possible
        if let Some(ref id) = app.selected_id {
            if let Some(i) = app.states.iter().position(|s| s.vibe_id == *id) {
                app.table_state.select(Some(i));
            } else {
                app.table_state.select(None);
                app.selected_id = None;
            }
        }
        if app.table_state.selected().is_none() && !app.states.is_empty() {
            app.table_state.select(Some(0));
            app.update_selected_id();
        }

        terminal.draw(|f| ui(f, &mut app))?;

        tokio::select! {
            maybe_event = reader.next() => {
                match maybe_event {
                    Some(Ok(Event::Key(key))) => {
                        if key.kind == event::KeyEventKind::Press {
                            match key.code {
                                KeyCode::Char('q') => return Ok(()),
                                KeyCode::Down | KeyCode::Char('j') => app.next(),
                                KeyCode::Up | KeyCode::Char('k') => app.previous(),
                                KeyCode::Char('f') => {
                                    app.focus_selected()?;
                                }
                                KeyCode::Char('x') | KeyCode::Char('K') => {
                                    app.kill_selected()?;
                                }
                                KeyCode::Enter => {
                                    app.focus_selected()?;
                                    return Ok(());
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(250)) => {
                // Background refresh for logs
                if let Some(ref id) = app.selected_id {
                    let logs_dir = vibe_core::env::resolve_logs_dir()?;
                    let log_path = logs_dir.join(format!("{}.log", id));
                    if log_path.exists() {
                        let content = tokio::fs::read_to_string(&log_path).await.unwrap_or_default();
                        let lines: Vec<&str> = content.lines().collect();
                        let last_lines = if lines.len() > 20 {
                            &lines[lines.len()-20..]
                        } else {
                            &lines[..]
                        };
                        app.logs = last_lines.join("\n");
                    } else {
                        app.logs = format!("No logs found at {:?}", log_path);
                    }
                }
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED).fg(Color::Yellow);
    let normal_style = Style::default().fg(Color::White);
    let header_cells = ["ID", "Role", "Status", "CWD", "Summary"]
        .iter()
        .map(|h| ratatui::widgets::Cell::from(*h).style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);

    let rows = app.states.iter().map(|item| {
        let status_text = item.status.clone();
        let status_style = Style::default();

        let cells = vec![
            ratatui::widgets::Cell::from(item.vibe_id.clone()),
            ratatui::widgets::Cell::from(item.role.clone().unwrap_or_default()),
            ratatui::widgets::Cell::from(status_text).style(status_style),
            ratatui::widgets::Cell::from(item.cwd.clone().unwrap_or_default()),
            ratatui::widgets::Cell::from(item.summary.clone()),
        ];
        Row::new(cells).height(1)
    });

    let t = Table::new(rows, [
        Constraint::Percentage(15),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(20),
        Constraint::Percentage(45),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(" Vibe Agents "))
    .row_highlight_style(selected_style)
    .highlight_symbol(">> ");

    f.render_stateful_widget(t, chunks[0], &mut app.table_state);

    let log_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Logs: {} ", app.selected_id.as_deref().unwrap_or("None")));
    let log_p = Paragraph::new(app.logs.as_str()).block(log_block);
    f.render_widget(log_p, chunks[1]);
}
