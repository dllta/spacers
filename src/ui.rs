use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Widget, Wrap},
};

use crate::app::App;

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

    let layout = Layout::vertical(vec![Constraint::Length(height + 1), Constraint::Fill(1)])
        .margin(1)
        .split(area);

    render_view(app, layout[1], buf);
}

fn generate_path(app: &App) -> Paragraph<'_> {
    let mut line = Line::from(
        Span::from(format!("Root")).style(if app.view_index == Some(0) {
            Style::new().fg(Color::Yellow).bold()
        } else if app.view.len() == 0 {
            Style::new().fg(Color::Green)
        } else {
            Style::new().fg(Color::Blue)
        }),
    );
    app.view
        .iter()
        .enumerate()
        .for_each(|(idx, object_handle)| {
            line.push_span(Span::from(" > ").style(Style::new().fg(Color::DarkGray)));
            line.push_span(Span::from(format!("{:?}", object_handle)).style(
                if app.view_index == Some(idx + 1) {
                    Style::new().fg(Color::Yellow).bold()
                } else if app.view.len() == idx + 1 {
                    if app.view_index == None {
                        Style::new().fg(Color::Green).bold()
                    } else {
                        Style::new().fg(Color::Green)
                    }
                } else {
                    Style::new().fg(Color::Blue)
                },
            ));
        });
    Paragraph::new(line).wrap(Wrap { trim: true })
}

fn render_path(app: &App, mut area: Rect, buf: &mut Buffer) -> u16 {
    if app.handle != app.get_view() {
        let layout = Layout::horizontal([Constraint::Fill(1), Constraint::Length(6)]).split(area);
        area = layout[0];

        Line::from("[Back]")
            .red()
            .on_black()
            .right_aligned()
            .render(layout[1], buf);
    }

    let para = generate_path(app);
    let line_count = para.line_count(area.width) as u16;

    let layout =
        Layout::vertical(vec![Constraint::Length(line_count), Constraint::Fill(1)]).split(area);

    para.render(layout[0], buf);

    line_count
}

fn render_view(app: &App, area: Rect, buf: &mut Buffer) {
    let view = match app.view_index {
        Some(_) => app.get_view_idx(),
        None => app.get_view(),
    };

    if let Some(view_handle) = view {
        let object = app.world.get_object(view_handle).unwrap();
        Line::from(format!(
            "Object (mass:{}, children:{})",
            object.mass,
            object.children_count(),
        ))
        .render(area, buf);
    } else {
        Line::from("Global info here!").render(area, buf);
    }
}
