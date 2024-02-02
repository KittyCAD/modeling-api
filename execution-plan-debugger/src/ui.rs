use kittycad_execution_plan::{ExecutionState, Instruction};
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
    let mem_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            // Top left, for main memory
            Constraint::Percentage(50),
            // Bottom left, for stack memory
            Constraint::Percentage(50),
        ])
        .split(body_chunks[1]);

    let title = Paragraph::new(Text::styled("Execution Plan Replay", Style::default().fg(Color::Green)))
        .block(Block::default().borders(Borders::ALL).style(Style::default()));

    let history_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title("History");
    let history_view = make_history_view(history_block, ctx);

    // Render the main memory view.
    let max_mem = ctx
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
        .max();

    let main_mem_view = match state.active_instruction() {
        HistorySelected::Instruction(active_instruction) => {
            let mem = &ctx.history[active_instruction].mem;
            let block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .title("Address Memory");
            Some(Paragraph::new(Text::styled(mem.debug_table(max_mem), Style::default())).block(block))
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
            if !mem.stack.is_empty() {
                Some(Paragraph::new(Text::styled(mem.debug_table_stack(), Style::default())).block(block))
            } else {
                None
            }
        }
        _ => None,
    };

    f.render_stateful_widget(history_view, body_chunks[0], &mut state.instruction_table_state);
    f.render_widget(title, chunks[0]);
    if let Some(view) = main_mem_view {
        f.render_widget(view, mem_chunks[0]);
    }
    if let Some(view) = stack_mem_view {
        f.render_widget(view, mem_chunks[1]);
    }
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
            },
        )| {
            let instruction = &ctx.plan[*active_instruction];

            let (instr_type, operands) = match instruction {
                Instruction::ApiRequest(_) => ("API request", "".to_owned()),
                Instruction::SetPrimitive { address, value } => ("SetPrimitive", format!("{address} to {value:?}")),
                Instruction::SetValue { address, value_parts } => ("SetValue", format!("{address} to {value_parts:?}")),
                Instruction::GetElement { start, index } => ("GetElement", format!("Addr {start} elem #{index:?}")),
                Instruction::GetProperty { start, property } => ("GetProperty", format!("Addr {start}[#{property:?}]")),
                Instruction::SetList { start, elements } => ("SetList", format!("{start:?}: {elements:?}")),
                Instruction::BinaryArithmetic {
                    arithmetic,
                    destination,
                } => ("BinaryArithmetic", format!("{arithmetic:?} to {destination:?}")),
                Instruction::UnaryArithmetic {
                    arithmetic,
                    destination,
                } => ("UnaryArithmetic", format!("{arithmetic:?} to {destination:?}")),
                Instruction::StackPush { data } => ("StackPush", format!("{data:?}")),
                Instruction::StackPop { destination } => ("StackPop", format!("{destination:?}")),
            };
            Row::new(vec![(i + 1).to_string(), instr_type.to_owned(), operands])
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
        Row::new(vec!["#", "Type", "Operands"])
            .style(Style::new().bold())
            .bottom_margin(1),
    )
    // Styles the selected row
    .highlight_style(Style::new().reversed())
    .highlight_symbol(">>")
    .block(block)
}
