use std::{io::stderr, ops::ControlFlow, time::Duration};

use kittycad_execution_plan::Instruction;
use ratatui::{backend::CrosstermBackend, widgets::TableState, Terminal};

const REFRESH_RATE: Duration = Duration::from_millis(250);

/// Probably immutable, given by the parent.
pub struct Context {
    pub history: Vec<kittycad_execution_plan::ExecutionState>,
    pub result: Result<(), kittycad_execution_plan::ExecutionError>,
    pub plan: Vec<Instruction>,
}

/// Probably mutable
pub struct State {
    pub instruction_table_state: TableState,
    pub num_rows: usize,
}

impl State {
    pub fn active_instruction(&self) -> Option<usize> {
        match self.instruction_table_state.selected() {
            Some(i) if i > 0 => Some(i - 1),
            _ => None,
        }
    }
}

pub fn run(ctx: Context) -> anyhow::Result<()> {
    // Boilerplate
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(stderr(), crossterm::terminal::EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;

    // App-specific
    let mut instruction_table_state = TableState::default();
    instruction_table_state.select(Some(0));
    let mut state = State {
        instruction_table_state,
        num_rows: ctx.history.len() + 1,
    };

    loop {
        if let ControlFlow::Break(_) = main_loop(&mut terminal, &mut state, &ctx)? {
            break;
        }
    }

    // Boilerplate
    crossterm::execute!(stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn main_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stderr>>,
    state: &mut State,
    ctx: &Context,
) -> anyhow::Result<ControlFlow<(), ()>> {
    // Render the UI
    terminal.draw(|f| crate::ui::ui(f, ctx, state))?;

    // Check user input and maybe update state.
    if crossterm::event::poll(REFRESH_RATE)? {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.kind == crossterm::event::KeyEventKind::Press {
                match KeyPress::try_from(key.code) {
                    Ok(KeyPress::Backwards) => match state.instruction_table_state.selected_mut() {
                        Some(x) if *x > 0 => *x -= 1,
                        _ => {}
                    },
                    Ok(KeyPress::Forwards) => match state.instruction_table_state.selected_mut() {
                        Some(x) if *x < state.num_rows - 1 => *x += 1,
                        _ => {}
                    },
                    Ok(KeyPress::Quit) => return Ok(ControlFlow::Break(())),
                    Err(()) => {}
                }
            }
        }
    }
    Ok(ControlFlow::Continue(()))
}

enum KeyPress {
    Backwards,
    Forwards,
    Quit,
}

impl TryFrom<crossterm::event::KeyCode> for KeyPress {
    type Error = ();

    fn try_from(value: crossterm::event::KeyCode) -> Result<Self, Self::Error> {
        use crossterm::event::KeyCode;
        use crossterm::event::KeyCode::Char;
        let key = match value {
            Char('a' | 'h' | 'w' | 'k') | KeyCode::Up | KeyCode::Left => Self::Backwards,
            Char('d' | 'l' | 's' | 'j') | KeyCode::Down | KeyCode::Right => Self::Forwards,
            Char('q') | KeyCode::Esc => Self::Quit,
            _ => return Err(()),
        };
        Ok(key)
    }
}