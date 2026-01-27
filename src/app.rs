use crate::{
    event::{AppEvent, Event, EventHandler},
    space::{Galaxy, ObjectHandle, Parent, ParentBuilder, Relation},
};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,

    /// game world
    pub world: Galaxy,
    /// player handle
    pub handle: Option<ObjectHandle>,
    /// currently inspected object and its parents
    pub view: Vec<ObjectHandle>,
    pub view_index: Option<usize>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            world: Galaxy::new(),
            handle: None,
            view: vec![],
            view_index: None,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let mut app = Self::default();

        let sun = app
            .world
            .spawn_object(300_000_000, ParentBuilder::Position([1.2, 0.7]));
        let _planet_1 = app.world.spawn_object(
            500_000,
            ParentBuilder::Relation(sun, Relation::Orbit(400_000)),
        );
        let planet_2 = app.world.spawn_object(
            700_000,
            ParentBuilder::Relation(sun, Relation::Orbit(1_200_000)),
        );
        let _moon = app.world.spawn_object(
            50_000,
            ParentBuilder::Relation(planet_2, Relation::Orbit(30_000)),
        );
        let ship = app.world.spawn_object(
            10_000,
            ParentBuilder::Relation(planet_2, Relation::Orbit(20_000)),
        );

        app.handle = Some(ship);
        app.view_goto(ship);

        app
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Key(key_event)
                    if key_event.kind == crossterm::event::KeyEventKind::Press =>
                {
                    self.handle_key_event(key_event)?
                }
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::Quit => self.quit(),
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }

            KeyCode::Esc => {
                self.view_index = None;
            }
            KeyCode::Left => match &mut self.view_index {
                Some(index) => {
                    if *index > 0 {
                        *index -= 1
                    } else {
                        *index = self.view.len()
                    }
                }
                None => {
                    if self.view.len() != 0 {
                        self.view_index = Some(self.view.len() - 1);
                    } else {
                        self.view_index = Some(0)
                    }
                }
            },
            KeyCode::Right => match &mut self.view_index {
                Some(index) => {
                    if *index < self.view.len() {
                        *index += 1
                    } else {
                        *index = 0
                    }
                }
                None => {
                    self.view_index = Some(0);
                }
            },
            // set view to selected
            KeyCode::Enter => {
                if let Some(index) = self.view_index {
                    if index == 0 {
                        self.view_reset();
                    } else if let Some(object_handle) = self.view.get(index - 1) {
                        self.view_goto(*object_handle);
                    }
                }
            }
            // reset view to handle
            KeyCode::Backspace => {
                if let Some(handle) = self.handle {
                    self.view_goto(handle);
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    fn view_goto(&mut self, object_handle: ObjectHandle) {
        let mut view = vec![object_handle];

        let mut current = &self.world.get_object(object_handle).unwrap().parent;
        while let Parent::Relation(parent_handle) = current {
            view.push(*parent_handle);
            current = &self.world.get_object(*parent_handle).unwrap().parent
        }
        view.reverse();
        self.view = view;
        self.view_index = None;
    }

    fn view_reset(&mut self) {
        self.view.clear();
        self.view_index = None;
    }
}
