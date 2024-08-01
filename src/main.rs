use std::io;

use ratatui::backend::CrosstermBackend;
use ratatui::style::Color;
use ratatui::Terminal;
use specs::{Builder, WorldExt};

use terminal_transport_game::app::{App, AppResult};
use terminal_transport_game::components::{
    Direction, Point, TrafficLight, TrainColors, TrainHead, TrainParameters, TrainParts,
};
use terminal_transport_game::event::{Event, EventHandler};
use terminal_transport_game::handler::handle_key_events;
use terminal_transport_game::map::Map;
use terminal_transport_game::tui::Tui;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();
    app.ecs.insert(Map::simple_ring(20, 10));

    app.ecs.register::<Point>();
    app.ecs.register::<TrainParts>();
    app.ecs.register::<TrainHead>();
    app.ecs.register::<TrainColors>();
    app.ecs.register::<TrainParameters>();
    app.ecs.register::<Direction>();
    app.ecs.register::<TrafficLight>();

    app.ecs
        .create_entity()
        .with(TrainHead {
            position: Point::new(0, 1),
        })
        .with(TrainParts {
            parts: vec![Point::new(0, 1), Point::new(0, 2), Point::new(0, 3)],
        })
        .with(TrainColors {
            main_color: Color::Blue,
            head_color: Color::LightBlue,
        })
        .with(TrainParameters {
            mass: 5.0,
            velocity: 0.0,
            acceleration: 0.0,
            force: 3.0,
            movement_direction: Direction::Up,
        })
        .build();

    app.ecs
        .create_entity()
        .with(TrafficLight { is_green: true })
        .with(Point { x: 5, y: 0 })
        .with(Direction::Right)
        .build();

    app.ecs
        .create_entity()
        .with(TrafficLight { is_green: true })
        .with(Point { x: 19, y: 5 })
        .with(Direction::Down)
        .build();

    app.ecs
        .create_entity()
        .with(TrafficLight { is_green: true })
        .with(Point { x: 3, y: 9 })
        .with(Direction::Left)
        .build();

    app.ecs
        .create_entity()
        .with(TrafficLight { is_green: true })
        .with(Point { x: 0, y: 6 })
        .with(Direction::Up)
        .build();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
