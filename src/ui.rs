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
    let layout = Layout::vertical(vec![Constraint::Length(1), Constraint::Length(1)])
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
    let mut line = Line::from(Span::from(format!("Root")).style(Style::new().fg(
        if app.view_index == 0 {
            Color::Yellow
        } else {
            Color::Blue
        },
    )));
    app.view
        .iter()
        .enumerate()
        .for_each(|(idx, object_handle)| {
            line.push_span(Span::from(" > ").style(Style::new().fg(Color::DarkGray)));
            line.push_span(
                Span::from(format!("{:?}", object_handle)).style(Style::new().fg(
                    if app.view_index == idx + 1 {
                        Color::Yellow
                    } else if app.view.len() == idx + 1 {
                        Color::Green
                    } else {
                        Color::Blue
                    },
                )),
            );
        });
    line.render(path_layout[0], buf);

    if app.handle != app.view.get(app.view.len().saturating_sub(1)).cloned() {
        Line::from("[Back]")
            .red()
            .on_black()
            .right_aligned()
            .render(path_layout[1], buf);
    }
}
