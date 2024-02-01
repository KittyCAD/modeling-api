use kittycad_execution_plan::{ExecutionState, Instruction};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize as _},
    text::Text,
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};

use crate::app::{Context, State};

pub fn ui(f: &mut Frame, ctx: &Context, state: &mut State) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(3)])
        .split(f.size());
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);
    let mem_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(body_chunks[1]);

    let title = Paragraph::new(Text::styled("Execution Plan Replay", Style::default().fg(Color::Green)))
        .block(Block::default().borders(Borders::ALL).style(Style::default()));

    // TODO: replace this with a table, with columns for the instruction type,
    // operands, etc.
    let instruction_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title("Instructions");
    let instruction_view = make_instruction_view(instruction_block, ctx);

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

    let active_instruction = state.active_instruction();

    let main_mem_view = match active_instruction {
        Some(active_instruction) => {
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
    let stack_mem_view = match active_instruction {
        Some(active_instruction) => {
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

    f.render_stateful_widget(instruction_view, body_chunks[0], &mut state.instruction_table_state);
    f.render_widget(title, chunks[0]);
    if let Some(view) = main_mem_view {
        f.render_widget(view, mem_chunks[0]);
    }
    if let Some(view) = stack_mem_view {
        f.render_widget(view, mem_chunks[1]);
    }
}

fn make_instruction_view<'a>(block: Block<'a>, ctx: &Context) -> Table<'a> {
    let widths = [
        Constraint::Percentage(10),
        Constraint::Percentage(20),
        Constraint::Percentage(70),
    ];
    let mut rows = Vec::with_capacity(ctx.history.len() + 1);
    rows.push(Row::new(vec!["Start".to_owned()]));
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
            Row::new(vec![i.to_string(), instr_type.to_owned(), operands])
        },
    ));
    Table::new(rows, widths)
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
