use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::{
    app::App,
    space::{ObjectHandle, Parent, Position, SystemHandle},
};

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

    let mut entries = vec![PathEntry::Object(handle)];

    let mut next_parent = &object.parent;
    loop {
        match next_parent {
            Parent::Root(pos) => {
                entries.push(PathEntry::Position(*pos));
                break;
            }
            Parent::System(system_handle, _orbit) => {
                next_parent = &app.world.get_system(*system_handle).unwrap().parent;
                entries.push(PathEntry::System(*system_handle));
            }
        }
    }
    entries.reverse();

    Path::from_entries(2, entries).render(app, layout[1], buf);
}

struct Path {
    index: usize,
    position: Option<Position>,
    systems: Vec<SystemHandle>,
    object: Option<ObjectHandle>,
}
pub enum PathEntry {
    Position(Position),
    System(SystemHandle),
    Object(ObjectHandle),
}
impl Path {
    fn from_entries(index: usize, mut entries: Vec<PathEntry>) -> Path {
        let position = match entries.get(0) {
            Some(entry) => match entry {
                PathEntry::Position(position) => {
                    let position = *position;
                    entries.remove(0);
                    Some(position)
                }
                _ => None,
            },
            None => None,
        };
        let object = match entries.get(entries.len() - 1) {
            Some(entry) => match entry {
                PathEntry::Object(object_handle) => {
                    let object_handle = *object_handle;
                    entries.remove(entries.len() - 1);
                    Some(object_handle)
                }
                _ => None,
            },
            None => None,
        };

        Path {
            index,
            position,
            systems: entries
                .iter()
                .filter_map(|entry| match entry {
                    PathEntry::System(system_handle) => Some(*system_handle),
                    _ => None,
                })
                .collect(),
            object,
        }
    }
    fn render(&self, app: &App, area: Rect, buffer: &mut Buffer) {
        let mut line = Line::from("");
        if let Some(position) = self.position {
            line.push_span(
                Span::from(format!("Pos({},{})", position[0], position[1])).style(Style::new().fg(
                    match self.index == 0 {
                        true => Color::Yellow,
                        false => Color::Gray,
                    },
                )),
            );
            line.push_span(Span::from(" > ").style(Style::new().fg(Color::DarkGray)));
        }
        for (idx, system_handle) in self.systems.iter().enumerate() {
            line.push_span(Span::from(format!("System({:?})", system_handle)).style(
                Style::new().fg(match self.index == idx + 1 {
                    true => Color::Yellow,
                    false => Color::Gray,
                }),
            ));
            if idx != self.systems.len() - 1 {
                line.push_span(Span::from(" > ").style(Style::new().fg(Color::DarkGray)));
            }
        }
        if let Some(object_handle) = self.object {
            line.push_span(Span::from(" > ").style(Style::new().fg(Color::DarkGray)));
            line.push_span(Span::from(format!("Object({:?})", object_handle)).style(
                Style::new().fg(match self.index == self.systems.len() + 1 {
                    true => Color::Yellow,
                    false => Color::Gray,
                }),
            ));
        }
        line.render(area, buffer);
    }

    fn get_index(&self, index: usize) -> Option<PathEntry> {
        if let Some(position) = self.position {
            if index == 0 {
                return Some(PathEntry::Position(position));
            }
        }
        if let Some(system_handle) = self.systems.get(index - 1) {
            return Some(PathEntry::System(*system_handle));
        }
        if let Some(object_handle) = self.object {
            if index - 1 == self.systems.len() {
                return Some(PathEntry::Object(object_handle));
            }
        }
        None
    }
}
