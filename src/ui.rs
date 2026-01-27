use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::{app::App, space::Parent};

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

    let Some(handle) = app.handle else {
        Paragraph::new("missing handle!")
            .centered()
            .render(layout[0], buf);
        return;
    };

    let Some(object) = app.world.get_object(handle) else {
        Paragraph::new("object does not exist!")
            .centered()
            .render(layout[0], buf);
        return;
    };

    Paragraph::new(match &object.parent {
        crate::space::Parent::Position(pos) => format!("position {:?}", pos),
        crate::space::Parent::Relation(parent_handle) => {
            format!("relation {:?}", parent_handle)
        }
    })
    .centered()
    .render(layout[0], buf);

    // render path
    let mut entries = vec![];

    let mut next_parent = &object.parent;
    let pos = loop {
        match next_parent {
            Parent::Position(pos) => {
                break pos;
            }
            Parent::Relation(parent_handle) => {
                entries.push(parent_handle.clone());
                next_parent = &app.world.get_object(*parent_handle).unwrap().parent;
            }
        }
    };
    entries.reverse();
    let mut line = Line::from(
        Span::from(format!("[{},{}]", pos[0], pos[1])).style(Style::new().fg(Color::Blue)),
    );
    for (_idx, parent) in entries.iter().enumerate() {
        line.push_span(Span::from(" > ").style(Style::new().fg(Color::DarkGray)));
        line.push_span(Span::from(format!("{:?}", parent)).style(Style::new().fg(Color::Blue)));
    }

    line.render(layout[1], buf);
}
