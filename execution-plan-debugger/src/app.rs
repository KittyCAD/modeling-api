use std::{io::stderr, ops::ControlFlow, time::Duration};

use kittycad_execution_plan::Instruction;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::counter::Counter;

const REFRESH_RATE: Duration = Duration::from_millis(250);

/// Probably immutable, given by the parent.
pub struct Context {
    pub history: Vec<kittycad_execution_plan::ExecutionState>,
    pub result: Result<(), kittycad_execution_plan::ExecutionError>,
    pub plan: Vec<Instruction>,
}

/// Probably mutable
pub struct State {
    pub counter: Counter,
}

pub fn run(ctx: Context) -> anyhow::Result<()> {
    // Boilerplate
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(stderr(), crossterm::terminal::EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;

    // App-specific
    let mut state = State {
        counter: Counter::new(ctx.history.len()),
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
                    Ok(KeyPress::Left) => state.counter.dec(),
                    Ok(KeyPress::Right) => state.counter.inc(),
                    Ok(KeyPress::Quit) => return Ok(ControlFlow::Break(())),
                    Err(()) => {}
                }
            }
        }
    }
    Ok(ControlFlow::Continue(()))
}

enum KeyPress {
    Left,
    Right,
    Quit,
}

impl TryFrom<crossterm::event::KeyCode> for KeyPress {
    type Error = ();

    fn try_from(value: crossterm::event::KeyCode) -> Result<Self, Self::Error> {
        use crossterm::event::KeyCode;
        use crossterm::event::KeyCode::Char;
        let key = match value {
            Char('a' | 'h' | 'w' | 'k') | KeyCode::Up | KeyCode::Left => Self::Left,
            Char('d' | 'l' | 's' | 'j') | KeyCode::Down | KeyCode::Right => Self::Right,
            Char('q') | KeyCode::Esc => Self::Quit,
            _ => return Err(()),
        };
        Ok(key)
    }
}
