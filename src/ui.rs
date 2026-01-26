use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
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
        crate::space::Parent::Root(pos) => format!("not in orbit, at {:?}", pos),
        crate::space::Parent::System(_system_handle, orbit) => {
            format!("orbiting at altitude {:?}", orbit.altitude)
        }
    })
    .centered()
    .render(layout[0], buf);

    let mut path = vec!["object".to_owned()];

    let mut next_parent = &object.parent;
    loop {
        match next_parent {
            Parent::Root(pos) => {
                path.push(format!("{:?}", pos));
                break;
            }
            Parent::System(system_handle, _orbit) => {
                next_parent = &app.world.get_system(*system_handle).unwrap().parent;
                path.push("system".to_owned());
            }
        }
    }
    path.reverse();

    Paragraph::new(path.join(" < "))
        .centered()
        .render(layout[1], buf);
}
