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
struct Unit;

#[derive(Component)]
struct Radio(String);

#[derive(Component)]
struct Health {
    hp: f32,
    max_health: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_unit)).run();
}

fn spawn_unit(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handler = asset_server.load("infantry.jpeg");
      commands.spawn((
        Sprite::from_image(texture_handler),
        Transform::default().with_scale(Vec3::splat(0.02)),
        Unit,
        Side {
            side: "Blufor".to_string(),
        },
        Position { x: 5.0, y: 4.0 },
        Health { hp: 100.0, max_health: 100.0 },
        Radio("Alpha".to_string())
    ));
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d::default(), Position {x: 0.0, y: 0.0}));
}
