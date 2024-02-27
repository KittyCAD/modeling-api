use std::{io::stderr, ops::ControlFlow, time::Duration};

use kittycad_execution_plan::Instruction;
use ratatui::{backend::CrosstermBackend, widgets::TableState, Terminal};

const REFRESH_RATE: Duration = Duration::from_millis(250);

/// Probably immutable, given by the parent.
pub struct Context {
    pub history: Vec<kittycad_execution_plan::ExecutionState>,
    pub last_instruction: usize,
    pub plan: Vec<Instruction>,
}
impl Context {
    /// How many addresses should be shown?
    /// This is the maximum across the entire history.
    pub fn address_size(&self) -> usize {
        self.history
            .iter()
            .filter_map(|hist| hist.mem.last_nonempty_address())
            .max()
            .unwrap_or_default()
    }
}

/// Probably mutable
pub struct State {
    pub instruction_pane: TablePaneState,
    pub address_pane: TablePaneState,
    pub active_pane: Pane,
}

pub struct TablePaneState {
    pub table: TableState,
    num_rows: usize,
}

impl TablePaneState {
    pub fn start(&mut self) {
        self.table.select(Some(0));
    }
    pub fn end(&mut self) {
        self.table.select(Some(self.num_rows - 1))
    }
    pub fn back(&mut self) {
        match self.table.selected_mut() {
            Some(x) if *x > 0 => *x -= 1,
            _ => {}
        }
    }
    fn forwards(&mut self) {
        if let Some(x) = self.table.selected_mut() {
            if *x < self.num_rows - 1 {
                *x += 1
            }
        }
    }
}

#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub enum Pane {
    #[default]
    Instructions,
    Addresses,
    // If you add a new enum variant, make sure to update the `fn next` below.
}

impl Pane {
    pub fn next(self) -> Self {
        match self {
            Pane::Instructions => Self::Addresses,
            Pane::Addresses => Self::Instructions,
        }
    }
}

pub enum InstructionSelected {
    Start,
    Instruction(usize),
}

impl State {
    pub fn active_instruction(&self) -> InstructionSelected {
        match self.instruction_pane.table.selected().unwrap() {
            0 => InstructionSelected::Start,
            other => InstructionSelected::Instruction(other - 1),
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
    let mut address_table_state = TableState::default();
    address_table_state.select(Some(0));
    let mut state = State {
        instruction_pane: TablePaneState {
            table: instruction_table_state,
            num_rows: ctx.history.len() + 1,
        },
        address_pane: TablePaneState {
            table: address_table_state,
            num_rows: ctx.address_size(),
        },
        // 1 extra row for start (before any instructions),
        // and 1 extra row for the finish result (err/ok).
        active_pane: Pane::default(),
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
                    Ok(KeyPress::PaneNext) => state.active_pane = state.active_pane.next(),
                    Ok(KeyPress::Backwards) => match state.active_pane {
                        Pane::Instructions => state.instruction_pane.back(),
                        Pane::Addresses => state.address_pane.back(),
                    },
                    Ok(KeyPress::Start) => match state.active_pane {
                        Pane::Instructions => state.instruction_pane.start(),
                        Pane::Addresses => state.address_pane.start(),
                    },
                    Ok(KeyPress::End) => match state.active_pane {
                        Pane::Instructions => state.instruction_pane.end(),
                        Pane::Addresses => state.address_pane.end(),
                    },
                    Ok(KeyPress::Forwards) => match state.active_pane {
                        Pane::Instructions => state.instruction_pane.forwards(),
                        Pane::Addresses => state.address_pane.forwards(),
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
    Start,
    End,
    Quit,
    PaneNext,
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
            Char('G') | KeyCode::End => Self::End,
            KeyCode::Tab => Self::PaneNext,
            KeyCode::Home => Self::Start,
            _ => return Err(()),
        };
        Ok(key)
    }
}
