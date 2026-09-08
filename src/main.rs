use bevy::prelude::*;

#[derive(Component)]
struct Side {
    side: String,
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Health {
    hp: f32,
}

#[derive(Component)]
struct Unit;

fn main() {
    App::new().add_systems(Startup, spawn_units).run();
}

fn spawn_units(mut commands: Commands) {
    commands.spawn((
        Unit,
        Side {
            side: "Blufor".to_string(),
        },
        Position { x: 5.0, y: 4.0 },
        Health { hp: 100.0 },
    ));
}
