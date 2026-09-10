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
