use std::collections::{HashMap, HashSet};

use kittycad_execution_plan::{
    events::{Event, Severity},
    BinaryArithmetic, ExecutionState, Instruction,
};
use kittycad_execution_plan_traits::{Address, Primitive};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize as _},
    text::Text,
    widgets::{Block, Cell, Padding, Paragraph, Row, Table},
    Frame,
};

use crate::app::{Context, InstructionSelected, Pane, State};

pub fn ui(f: &mut Frame, ctx: &Context, state: &mut State) {
    // Create all widgets.
    let title =
        Paragraph::new(Text::styled("Execution Plan Replay", Style::default().fg(GREEN))).block(Block::bordered());

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

    let basic_block = |title: &'static str, selected: bool| {
        Block::bordered()
            .padding(Padding::vertical(1))
            .title(title)
            .style(if selected {
                Style::default().fg(GREEN)
            } else {
                Style::default()
            })
    };

    let instruction_block = basic_block("Instructions", state.active_pane == Pane::Instructions);
    let instruction_view = make_instruction_view(instruction_block, ctx, &instructions_with_errors);

    let event_block = basic_block("Events", false);
    let events = match state.active_instruction() {
        InstructionSelected::Instruction(i) => &ctx.history[i].events,
        _ => [].as_slice(),
    };
    let (event_view, addr_colors) = make_events_view(event_block, events);

    // Render the addressable memory view.
    let address_block = basic_block("Address Memory", state.active_pane == Pane::Addresses);
    let address_view = match state.active_instruction() {
        InstructionSelected::Instruction(active_instruction) => {
            let mem = &ctx.history[active_instruction].mem;
            make_address_view(address_block, mem, addr_colors, ctx.address_size())
        }
        _ => Table::new(Vec::<Row>::new(), Vec::<Constraint>::new()).block(address_block),
    };

    // Render the stack memory view.
    let stack_view_block = basic_block("Stack Memory", false);
    let stack_mem_view = match state.active_instruction() {
        InstructionSelected::Instruction(active_instruction) => {
            let mem = &ctx.history[active_instruction].mem;
            make_stack_view(stack_view_block, &mem.stack)
        }
        _ => Table::new(Vec::<Row>::new(), Vec::<Constraint>::new()).block(stack_view_block),
    };

    let footer = Paragraph::new(Text::styled(
        "Controls: Up/Down or Left/Right to scroll, Tab to change pane, Q/Esc to quit",
        Style::default().fg(GREEN),
    ))
    .block(Block::bordered());

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
            // Left half of body, for instructions.
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
            // Top left, for instructions.
            Constraint::Percentage(75),
            // Bottom left, for events.
            Constraint::Percentage(25),
        ])
        .split(body_chunks[0]);
    // Put widgets into various areas.
    f.render_stateful_widget(instruction_view, left_chunks[0], &mut state.instruction_pane.table);
    f.render_widget(event_view, left_chunks[1]);
    f.render_widget(title, chunks[0]);
    f.render_stateful_widget(address_view, right_chunks[0], &mut state.address_pane.table);
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

const GREEN: Color = Color::from_u32(0x4ec9b0);
const HIGHLIGHT_COLORS: [Color; 9] = [
    Color::from_u32(0x007acc),
    Color::from_u32(0xffd602),
    Color::from_u32(0xc586c0),
    GREEN,
    Color::from_u32(0x569CD6),
    Color::from_u32(0x646695),
    Color::from_u32(0x6A9955),
    Color::from_u32(0xD16969),
    Color::from_u32(0xDCDCAA),
];

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
            Constraint::Percentage(100),
        ],
    )
    .column_spacing(1)
    .header(Row::new(vec!["#", "Level", "Related Addr", "Msg"]).style(Style::new().bold()))
    .block(block);
    (tbl, addr_colors)
}

fn make_address_view<'a>(
    block: Block<'a>,
    mem: &kittycad_execution_plan::Memory,
    addr_colors: HashMap<Address, Color>,
    num_rows: usize,
) -> Table<'a> {
    // After a certain address, all following addresses will be empty.
    // Only show addresses before that point.
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
            .style(
                Style::default()
                    .bold()
                    .fg(addr_colors.get(&(Address::ZERO + addr)).copied().unwrap_or_default()),
            )
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
    .highlight_style(Style::new().reversed())
    .highlight_symbol(">>")
    .block(block)
}

fn make_instruction_view<'a>(block: Block<'a>, ctx: &Context, instrs_with_errors: &HashSet<usize>) -> Table<'a> {
    let mut rows = Vec::with_capacity(ctx.plan.len() + 1);
    // Start row
    rows.push(Row::new(vec![Cell::new("0"), Cell::new("Start")]).style(Style::default().fg(GREEN)));
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
    rows.extend((ctx.last_instruction..ctx.plan.len() - 1).map(|i| {
        let instruction = &ctx.plan[i + 1];
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
fn describe_instruction(instruction: &Instruction) -> (std::borrow::Cow<'static, str>, String) {
    match instruction {
        Instruction::ApiRequest(req) => (format!("API {}", req.endpoint).into(), format!("{:?}", req.arguments)),
        Instruction::SetPrimitive { address, value } => {
            ("SetPrimitive".into(), format!("Set addr {address} to {value:?}"))
        }
        Instruction::Copy { source, destination } => ("Copy".into(), format!("From {source} to {destination}")),
        Instruction::SetValue { address, value_parts } => (
            "SetValue".into(),
            format!("Write {value_parts:?} starting at address {address}"),
        ),
        Instruction::AddrOfMember { start, member } => (
            "AddrOfMember".into(),
            format!("Find member '{member:?}'\nof object at address {start:?}"),
        ),
        Instruction::SetList { start, elements } => (
            "SetList234".into(),
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
                "BinaryArithmetic".into(),
                format!("Set {destination:?}\nto {arith_description}"),
            )
        }
        Instruction::UnaryArithmetic {
            arithmetic,
            destination,
        } => (
            "UnaryArithmetic".into(),
            format!("Set {destination:?}\nto {arithmetic:?}"),
        ),
        Instruction::StackPush { data } => ("StackPush".into(), format!("{data:?}")),
        Instruction::StackPop { destination } => (
            "StackPop".into(),
            match destination {
                Some(dst) => format!("Into: {dst:?}"),
                None => "Discard".to_owned(),
            },
        ),
        Instruction::CopyLen {
            source_range,
            destination_range,
        } => (
            "Copy".into(),
            format!("copy from {source_range:?} to {destination_range:?}"),
        ),
    }
}
