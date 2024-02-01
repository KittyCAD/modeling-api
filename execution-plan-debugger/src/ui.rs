use kittycad_execution_plan::ExecutionState;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{self, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{Context, State};

pub fn ui(f: &mut Frame, ctx: &Context, state: &State) {
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

    let title_block = Block::default().borders(Borders::ALL).style(Style::default());
    let title =
        Paragraph::new(Text::styled("Execution Plan Replay", Style::default().fg(Color::Green))).block(title_block);

    // TODO: replace this with a table, with columns for the instruction type,
    // operands, etc.
    let instruction_block = Block::default().borders(Borders::ALL).style(Style::default());
    let instruction_view = make_instruction_view(instruction_block, ctx, state);

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
    let main_mem_view = if let Some(active_instruction) = state.counter.curr {
        let mem = &ctx.history[active_instruction].mem;
        Some(Paragraph::new(Text::styled(mem.debug_table(max_mem), Style::default())))
    } else {
        None
    };

    // Render the stack view.
    let stack_mem_view = if let Some(active_instruction) = state.counter.curr {
        let mem = &ctx.history[active_instruction].mem;
        if mem.stack.is_empty() {
            None
        } else {
            Some(Paragraph::new(Text::styled(mem.debug_table_stack(), Style::default())))
        }
    } else {
        None
    };

    f.render_widget(instruction_view, body_chunks[0]);
    f.render_widget(title, chunks[0]);
    if let Some(view) = main_mem_view {
        f.render_widget(view, mem_chunks[0]);
    }
    if let Some(view) = stack_mem_view {
        f.render_widget(view, mem_chunks[1]);
    }
}

fn make_instruction_view<'a>(instruction_block: Block<'a>, ctx: &Context, state: &State) -> List<'a> {
    const ARROW: &str = "*";
    const NO_ARROW: &str = " ";
    const HIGHLIGHT: Color = Color::LightBlue;
    let mut instruction_entries = Vec::with_capacity(ctx.history.len() + 1);
    let (arrow, color) = if state.counter.is_start() {
        (ARROW, HIGHLIGHT)
    } else {
        (NO_ARROW, Color::default())
    };
    instruction_entries.push(ListItem::new(text::Line::from(Span::styled(
        format!("{arrow}Start"),
        Style::default().fg(color),
    ))));
    instruction_entries.extend(ctx.history.iter().enumerate().map(
        |(
            i,
            ExecutionState {
                mem: _,
                active_instruction,
            },
        )| {
            let instruction = &ctx.plan[*active_instruction];
            let (arrow, color) = if state.counter == Some(i) {
                (ARROW, HIGHLIGHT)
            } else {
                (NO_ARROW, Color::default())
            };

            ListItem::new(text::Line::from(Span::styled(
                format!("{arrow}{instruction:?}"),
                Style::default().fg(color),
            )))
        },
    ));
    List::new(instruction_entries).block(instruction_block)
}
