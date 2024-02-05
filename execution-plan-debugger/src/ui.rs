use std::collections::{HashMap, HashSet};

use kittycad_execution_plan::{
    events::{Event, Severity},
    BinaryArithmetic, ExecutionState, Instruction,
};
use kittycad_execution_plan_traits::{Address, Primitive, ReadMemory};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize as _},
    text::Text,
    widgets::{Block, Borders, Cell, Padding, Paragraph, Row, Table},
    Frame,
};

use crate::app::{Context, HistorySelected, State};

pub fn ui(f: &mut Frame, ctx: &Context, state: &mut State) {
    // Create all widgets.
    let title = Paragraph::new(Text::styled("Execution Plan Replay", Style::default().fg(Color::Green)))
        .block(Block::default().borders(Borders::ALL));

    let instructions_with_errors: HashSet<_> = ctx
        .history
        .iter()
        .enumerate()
        .filter_map(|(i, st)| {
            if st.events.iter().any(|evt| evt.severity == Severity::Error) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    let basic_block = |title: &'static str| {
        Block::default()
            .borders(Borders::ALL)
            .padding(Padding::vertical(1))
            .title(title)
    };

    let history_block = basic_block("History");
    let history_view = make_history_view(history_block, ctx, &instructions_with_errors);

    let event_block = basic_block("Events");
    let events = match state.active_instruction() {
        HistorySelected::Instruction(i) => &ctx.history[i].events,
        _ => [].as_slice(),
    };
    let (event_view, addr_colors) = make_events_view(event_block, events);

    // Render the main memory view.
    let main_mem_block = basic_block("Address Memory");
    let main_mem_view = match state.active_instruction() {
        HistorySelected::Instruction(active_instruction) => {
            let mem = &ctx.history[active_instruction].mem;
            make_memory_view(main_mem_block, mem, addr_colors)
        }
        _ => Table::new(Vec::<Row>::new(), Vec::<Constraint>::new()).block(main_mem_block),
    };

    // Render the stack view.
    let stack_view_block = basic_block("Stack Memory");
    let stack_mem_view = match state.active_instruction() {
        HistorySelected::Instruction(active_instruction) => {
            let mem = &ctx.history[active_instruction].mem;
            make_stack_view(stack_view_block, &mem.stack)
        }
        _ => Table::new(Vec::<Row>::new(), Vec::<Constraint>::new()).block(stack_view_block),
    };

    let footer = Paragraph::new(Text::styled(
        "Use up/down or left/right to scroll through the execution of your program",
        Style::default().fg(Color::Green),
    ))
    .block(Block::default().borders(Borders::ALL));

    // Create areas for the widgets above to go into.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            // Header
            Constraint::Length(3),
            // Body
            Constraint::Min(1),
            // Footer
            Constraint::Length(3),
        ])
        .split(f.size());
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            // Left half of body, for history/instructions.
            Constraint::Percentage(50),
            // Right half of body, for memory.
            Constraint::Percentage(50),
        ])
        .split(chunks[1]);
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            // Top left, for main memory
            Constraint::Percentage(75),
            // Bottom left, for stack memory
            Constraint::Percentage(25),
        ])
        .split(body_chunks[1]);
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            // Top left, for history
            Constraint::Percentage(75),
            // Bottom left, for events
            Constraint::Percentage(25),
        ])
        .split(body_chunks[0]);
    // Put widgets into various areas.
    f.render_stateful_widget(history_view, left_chunks[0], &mut state.instruction_table_state);
    f.render_widget(event_view, left_chunks[1]);
    f.render_widget(title, chunks[0]);
    f.render_widget(main_mem_view, right_chunks[0]);
    f.render_widget(stack_mem_view, right_chunks[1]);
    f.render_widget(footer, chunks[2]);
}

fn make_stack_view<'a>(block: Block<'a>, stack: &kittycad_execution_plan::Stack<Vec<Primitive>>) -> Table<'a> {
    let rows = stack
        .iter()
        .enumerate()
        .map(|(depth, val)| Row::new(vec![depth.to_string(), format!("{val:?}")]));

    Table::new(
        rows,
        [
            // Depth
            Constraint::Length(5),
            // Value
            Constraint::Max(50),
        ],
    )
    .column_spacing(1)
    .header(Row::new(vec!["Depth", "Value"]).style(Style::new().bold()))
    .block(block)
}

const HIGHLIGHT_COLORS: [Color; 5] = [Color::Green, Color::Cyan, Color::Magenta, Color::Yellow, Color::Blue];

fn make_events_view<'a>(block: Block<'a>, events: &[Event]) -> (Table<'a>, HashMap<Address, Color>) {
    let mut addr_colors = HashMap::new();
    let rows = events.iter().cloned().enumerate().map(|(i, event)| {
        let text_color = match event.severity {
            Severity::Error => Color::Red,
            Severity::Info => Color::default(),
            Severity::Debug => Color::DarkGray,
        };
        let highlight_color = match event.related_address {
            Some(addr) => {
                let color_num = addr_colors.len();
                addr_colors.insert(addr, HIGHLIGHT_COLORS[color_num]);
                HIGHLIGHT_COLORS[color_num]
            }
            None => Color::default(),
        };
        Row::new(vec![
            // Event number
            Cell::from(i.to_string()),
            // Severity
            // Cell::from(event.severity.to_string()),
            Cell::new(Text::styled(
                event.severity.to_string(),
                Style::default().fg(text_color),
            )),
            // Related address
            Cell::new(Text::styled(
                if let Some(addr) = event.related_address {
                    addr.to_string()
                } else {
                    "-".to_owned()
                },
                Style::default().fg(highlight_color),
            )),
            // Text
            Cell::new(Text::styled(event.text.to_string(), Style::default().fg(text_color))),
        ])
    });

    let tbl = Table::new(
        rows,
        [
            // Event number
            Constraint::Length(3),
            // Event severity
            Constraint::Length(6),
            // Address
            Constraint::Length(12),
            // Message
            Constraint::Max(50),
        ],
    )
    .column_spacing(1)
    .header(Row::new(vec!["#", "Level", "Related Addr", "Msg"]).style(Style::new().bold()))
    .block(block);
    (tbl, addr_colors)
}

fn make_memory_view<'a>(
    block: Block<'a>,
    mem: &kittycad_execution_plan::Memory,
    // num_rows: usize,
    addr_colors: HashMap<Address, Color>,
) -> Table<'a> {
    // After a certain address, all following addresses will be empty.
    // Only show addresses before that point.
    let num_rows = (0..(mem.addresses.len()))
        .rev()
        .find(|addr| mem.get(&(Address::ZERO + *addr)).is_some())
        .map(|x| x + 1)
        .unwrap_or(mem.addresses.len());
    let rows = mem
        .addresses
        .iter()
        .cloned()
        .enumerate()
        .take(num_rows)
        .map(|(addr, val)| {
            Row::new(vec![
                addr.to_string(),
                if let Some(val) = val {
                    format!("{val:?}")
                } else {
                    ".".to_owned()
                },
            ])
            .style(Style::default().fg(addr_colors.get(&(Address::ZERO + addr)).copied().unwrap_or_default()))
        });

    Table::new(
        rows,
        [
            // Address
            Constraint::Length(4),
            // Value
            Constraint::Max(50),
        ],
    )
    .column_spacing(1)
    .header(Row::new(vec!["Address", "Value"]).style(Style::new().bold()))
    .block(block)
}

fn make_history_view<'a>(block: Block<'a>, ctx: &Context, instrs_with_errors: &HashSet<usize>) -> Table<'a> {
    let mut rows = Vec::with_capacity(ctx.plan.len() + 1);
    // Start row
    rows.push(Row::new(vec![Cell::new("0"), Cell::new("Start")]).style(Style::default().fg(Color::Green)));
    // One row per executed instruction
    rows.extend(ctx.history.iter().enumerate().map(
        |(
            i,
            ExecutionState {
                mem: _,
                active_instruction,
                events: _,
            },
        )| {
            let instruction = &ctx.plan[*active_instruction];

            let (instr_type, operands) = describe_instruction(instruction);
            let height = operands.chars().filter(|ch| ch == &'\n').count() + 1;
            let style = Style::default().fg(if instrs_with_errors.contains(&i) {
                Color::Red
            } else {
                Color::default()
            });
            Row::new(vec![
                Cell::new((i + 1).to_string()),
                Cell::new(instr_type),
                Cell::new(operands),
            ])
            .style(style)
            .height(height.try_into().expect("height of cell must fit into u16"))
        },
    ));
    // One row per remaining (unexecuted) instructions.
    let n = ctx.history.len();
    rows.extend((ctx.last_instruction..ctx.plan.len()).map(|i| {
        let instruction = &ctx.plan[i];
        let (instr_type, operands) = describe_instruction(instruction);
        let height = operands.chars().filter(|ch| ch == &'\n').count() + 1;
        let style = Style::default().fg(Color::DarkGray);
        Row::new(vec![
            Cell::new(((i - ctx.last_instruction) + 1 + n).to_string()),
            Cell::new(instr_type),
            Cell::new(operands),
        ])
        .style(style)
        .height(height.try_into().expect("height of cell must fit into u16"))
    }));

    // Combine all rows into the table.
    Table::new(
        rows,
        [
            // Instruction number
            Constraint::Percentage(10),
            // Instruction type
            Constraint::Percentage(20),
            // Instruction operands
            Constraint::Percentage(70),
        ],
    )
    .column_spacing(1)
    .header(
        Row::new(vec!["Time", "Type", "Operands"]).style(Style::new().bold()), // .bottom_margin(1),
    )
    // Styles the selected row
    .highlight_style(Style::new().reversed())
    .highlight_symbol(">>")
    .block(block)
}

/// Display the instruction type and the operands, in a human-readable, friendly way.
fn describe_instruction(instruction: &Instruction) -> (&'static str, String) {
    match instruction {
        Instruction::ApiRequest(_) => ("API request", "".to_owned()),
        Instruction::SetPrimitive { address, value } => ("SetPrimitive", format!("Set addr {address} to {value:?}")),
        Instruction::SetValue { address, value_parts } => (
            "SetValue",
            format!("Write {value_parts:?} starting at address {address}"),
        ),
        Instruction::AddrOfMember { start, member } => (
            "GetProperty",
            format!("Find member '{member:?}'\nof object at address {start}"),
        ),
        Instruction::SetList { start, elements } => (
            "SetList",
            format!("Create list at {start:?}\nwith elements {elements:?}"),
        ),
        Instruction::BinaryArithmetic {
            arithmetic,
            destination,
        } => {
            let BinaryArithmetic {
                operation,
                operand0,
                operand1,
            } = arithmetic;
            let arith_description = format!("{operand0:?} {operation} {operand1:?}");
            (
                "BinaryArithmetic",
                format!("Set {destination:?}\nto {arith_description}"),
            )
        }
        Instruction::UnaryArithmetic {
            arithmetic,
            destination,
        } => ("UnaryArithmetic", format!("Set {destination:?}\nto {arithmetic:?}")),
        Instruction::StackPush { data } => ("StackPush", format!("{data:?}")),
        Instruction::StackPop { destination } => (
            "StackPop",
            match destination {
                Some(dst) => format!("Into: {dst:?}"),
                None => "Discard".to_owned(),
            },
        ),
    }
}
