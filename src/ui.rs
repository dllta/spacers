use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Widget, Wrap},
};

use crate::{app::App, object::Parent, space::World};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("spacers")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        block.render(area, buf);

        render_viewport(self, area, buf);
    }
}

fn render_viewport(app: &App, area: Rect, buf: &mut Buffer) {
    let layout = Layout::vertical(vec![Constraint::Fill(1)])
        .margin(1)
        .split(area);

    let height = render_path(app, layout[0], buf);

    let layout = Layout::vertical(vec![Constraint::Length(height), Constraint::Fill(1)])
        .margin(1)
        .split(area);

    render_view(app, layout[1], buf);
}

fn generate_path(app: &App) -> Paragraph<'_> {
    let mut line = Line::from(Span::from("Space").style(if app.view_index == Some(0) {
        Style::new().bg(Color::Yellow).fg(Color::Blue).bold()
    } else if app.view.len() == 0 {
        Style::new().fg(Color::Green)
    } else {
        Style::new().fg(Color::Blue)
    }));
    app.view
        .iter()
        .enumerate()
        .for_each(|(idx, object_handle)| {
            line.push_span(Span::from(" > ").style(Style::new().fg(Color::DarkGray)));
            line.push_span(
                Span::from(format!(
                    "{}",
                    app.world.get_object(*object_handle).unwrap().name
                ))
                .style(if app.view_index == Some(idx + 1) {
                    Style::new().bg(Color::Yellow).fg(Color::Blue).bold()
                } else if app.view.len() == idx + 1 {
                    if app.view_index == None {
                        Style::new().fg(Color::Green).bold()
                    } else {
                        Style::new().fg(Color::Green)
                    }
                } else {
                    Style::new().fg(Color::Blue)
                }),
            );
        });
    if app.view_index != None {
        line.push_span(Span::from(" [Esc]").style(Style::new().fg(Color::Yellow)));
    } else if app.get_view() != app.world.get_handle() {
        line.push_span(Span::from(" [Esc]").style(Style::new().fg(Color::Gray)));
    } else {
        line.push_span(Span::from(" <-You").style(Style::new().fg(Color::Gray)));
    }
    Paragraph::new(line).wrap(Wrap { trim: true })
}

fn render_path(app: &App, area: Rect, buf: &mut Buffer) -> u16 {
    let para = generate_path(app);
    let line_count = para.line_count(area.width) as u16;

    let layout =
        Layout::vertical(vec![Constraint::Length(line_count), Constraint::Fill(1)]).split(area);

    para.render(layout[0], buf);

    line_count
}

fn render_view(app: &App, area: Rect, buf: &mut Buffer) {
    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .split(area);
    let view = match app.view_index {
        Some(_) => app.get_view_idx(),
        None => app.get_view(),
    };

    if let Some(view_handle) = view {
        let object = app.world.get_object(view_handle).unwrap();

        Line::from(format!(
            "{}: (mass:{}kg, children:{})",
            object.name,
            object.mass,
            object.children_count(),
        ))
        .render(layout[0], buf);

        Line::from(match object.parent {
            Parent::Position(pos) => format!("at [{},{}]", pos[0], pos[1]),
            Parent::Relation(parent_handle) => {
                let parent = app.world.get_object(parent_handle).unwrap();
                format!(
                    "orbiting {}: {:?}",
                    parent.name,
                    parent.get_child(view_handle).unwrap()
                )
            }
        })
        .render(layout[1], buf);
    } else {
        Line::from("Global info here!").render(area, buf);
    }
}
