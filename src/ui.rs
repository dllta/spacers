use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Widget},
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
    let layout = Layout::vertical(vec![
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .margin(1)
    .split(area);

    //let Some(handle) = app.handle else {
    //    Paragraph::new("missing handle!")
    //        .centered()
    //        .render(layout[0], buf);
    //    return;
    //};

    //let Some(object) = app.world.get_object(handle) else {
    //    Paragraph::new("object does not exist!")
    //        .centered()
    //        .render(layout[0], buf);
    //    return;
    //};

    // render path
    let path_layout =
        Layout::horizontal([Constraint::Fill(1), Constraint::Length(6)]).split(layout[0]);
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
    line.render(path_layout[0], buf);

    if app.handle != app.get_view() {
        Line::from("[Back]")
            .red()
            .on_black()
            .right_aligned()
            .render(path_layout[1], buf);
    }

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
        .render(layout[1], buf);
    } else {
        Line::from("Global info here!").render(layout[1], buf);
    }
}
