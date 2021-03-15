use bevy::prelude::*;
use heron::*;

#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default()) // Add the plugin
        .add_resource(Gravity::from(Vec3::new(0.0, -300.0, 0.0))) // Optionally define gravity
        .add_startup_system(spawn.system())
        .run();
}

fn spawn(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    let size = Vec2::new(30.0, 30.0);
    commands
        // Spawn any bundle of your choice. Only make sure there is a `GlobalTransform`
        .spawn(SpriteBundle {
            sprite: Sprite::new(size),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            ..Default::default()
        })
        // Make it a physics body, by attaching a collision shape
        .with(Body::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
        });
}
