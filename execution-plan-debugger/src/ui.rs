use kittycad_execution_plan::{Event, ExecutionState, Instruction};
use kittycad_execution_plan_traits::Primitive;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize as _},
    text::Text,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::app::{Context, HistorySelected, State};

pub fn ui(f: &mut Frame, ctx: &Context, state: &mut State) {
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

    let title = Paragraph::new(Text::styled("Execution Plan Replay", Style::default().fg(Color::Green)))
        .block(Block::default().borders(Borders::ALL).style(Style::default()));

    let history_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title("History");
    let history_view = make_history_view(history_block, ctx);

    let event_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title("Events");
    let events = match state.active_instruction() {
        HistorySelected::Instruction(i) => &ctx.history[i].events,
        _ => [].as_slice(),
    };
    let event_view = make_events_view(event_block, &events);

    // Render the main memory view.
    let num_memory_rows = ctx
        .history
        .iter()
        .filter_map(|exec_st| {
            exec_st
                .mem
                .addresses
                .iter()
                .enumerate()
                .find_map(|(i, mem)| if mem.is_none() { Some(i) } else { None })
        })
        .max()
        .unwrap();

    let main_mem_view = match state.active_instruction() {
        HistorySelected::Instruction(active_instruction) => {
            let mem = &ctx.history[active_instruction].mem;
            let block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .title("Address Memory");
            Some(make_memory_view(block, mem, num_memory_rows))
        }
        _ => None,
    };

    // Render the stack view.
    let stack_mem_view = match state.active_instruction() {
        HistorySelected::Instruction(active_instruction) => {
            let mem = &ctx.history[active_instruction].mem;
            let block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .title("Stack Memory");
            Some(make_stack_view(block, &mem.stack))
        }
        _ => None,
    };

    let footer = Paragraph::new(Text::styled(
        "Use up/down or left/right to scroll through the execution of your program",
        Style::default().fg(Color::Green),
    ))
    .block(Block::default().borders(Borders::ALL).style(Style::default()));

    f.render_stateful_widget(history_view, left_chunks[0], &mut state.instruction_table_state);
    f.render_widget(event_view, left_chunks[1]);
    f.render_widget(title, chunks[0]);
    if let Some(view) = main_mem_view {
        f.render_widget(view, right_chunks[0]);
    }
    if let Some(view) = stack_mem_view {
        f.render_widget(view, right_chunks[1]);
    }
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

fn make_events_view<'a>(block: Block<'a>, events: &[Event]) -> Table<'a> {
    let rows = events
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, event)| Row::new(vec![i.to_string(), event.text]));

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
fn make_memory_view<'a>(block: Block<'a>, mem: &kittycad_execution_plan::Memory, num_rows: usize) -> Table<'a> {
    let rows = mem
        .addresses
        .iter()
        .cloned()
        .enumerate()
        .filter_map(|(addr, val)| val.map(|val| (addr, val)))
        .take(num_rows)
        .map(|(addr, val)| Row::new(vec![addr.to_string(), format!("{val:?}")]));

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

fn make_history_view<'a>(block: Block<'a>, ctx: &Context) -> Table<'a> {
    let mut rows = Vec::with_capacity(ctx.history.len() + 2);
    rows.push(Row::new(vec![
        Cell::new("0"),
        Cell::new(Text::styled("Start", Style::default().fg(Color::Green))),
    ]));
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

            let (instr_type, operands) = match instruction {
                Instruction::ApiRequest(_) => ("API request", "".to_owned()),
                Instruction::SetPrimitive { address, value } => {
                    ("SetPrimitive", format!("Set addr {address} to {value:?}"))
                }
                Instruction::SetValue { address, value_parts } => (
                    "SetValue",
                    format!("Write {value_parts:?} starting at address {address}"),
                ),
                Instruction::GetElement { start, index } => (
                    "GetElement",
                    format!("Find element #{index:?}\nof array at address {start}"),
                ),
                Instruction::GetProperty { start, property } => (
                    "GetProperty",
                    format!("Find property '{property:?}'\nof object at address {start}"),
                ),
                Instruction::SetList { start, elements } => (
                    "SetList",
                    format!("Create list at {start:?}\nwith elements {elements:?}"),
                ),
                Instruction::BinaryArithmetic {
                    arithmetic,
                    destination,
                } => ("BinaryArithmetic", format!("Set {destination:?}\nto {arithmetic:?}")),
                Instruction::UnaryArithmetic {
                    arithmetic,
                    destination,
                } => ("UnaryArithmetic", format!("Set {destination:?}\nto {arithmetic:?}")),
                Instruction::StackPush { data } => ("StackPush", format!("{data:?}")),
                Instruction::StackPop { destination } => ("StackPop", format!("{destination:?}")),
            };
            let height = operands.chars().filter(|ch| ch == &'\n').count() + 1;
            Row::new(vec![(i + 1).to_string(), instr_type.to_owned(), operands])
                .height(height.try_into().expect("height of cell must fit into u16"))
        },
    ));
    rows.push(Row::new(vec![
        Cell::new((ctx.history.len() + 1).to_string()),
        match &ctx.result {
            Ok(_) => Cell::new(Text::styled("Finished", Style::default().fg(Color::Green))),
            Err(_e) => Cell::new(Text::styled("Error", Style::default().fg(Color::Red))),
        },
        match &ctx.result {
            Ok(_) => Cell::new(Text::styled("", Style::default().fg(Color::Green))),
            Err(e) => Cell::new(Text::styled(e.to_string(), Style::default().fg(Color::Red))),
        },
    ]));
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
