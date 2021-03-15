# Heron Manual

## Basic example

Here is a basic example of using Heron in 2d. It draws a green box
that falls down due to gravity:

```rust
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
```

You can add it into your own Rust project's `main.rs`. Also add this
to your `Cargo.toml`:

```toml
[dependencies]
bevy = { version = "0.4"} 
heron = { version = "0.2.0", default-features = false, features = ["2d"] }
```

If you then run `cargo run` you should see the green box fall.

### Anatomy of the example

We create a normal `Bevy` app. To enable Heron we must add the `PhysicsPlugin`.
Optionally you can add a `Gravity` resource, as we do here.

We also have a startup system `spawn` where we spawn our box. 

It starts by setting up the camera bundle to 2d so that we can see something.

We then spawn a `SpriteBundle` in the normal Bevy style; here we generate a very
basic green `box` sprite and place it on `x` and `y` coordinates. Note that we
use `Vec3` even though we are in 2d space - to do so, we simply set the `z`
coordinate to zero.

To make it work with the physics engine must add a Heron `Body` component. In
this case we add a cuboid (rectangular in 2d) collision shape.

And that's all there is to it! Heron, using the Rapier physics engine, will
make your sprite behave according to physics!

# A run-through of features

* You can add BodyType Static, Kinematic and Dynamic

* PhysicsMaterial

* In order to rotate a transform, use `rotate` with a Quad.

* Rotation in 2d is around the z axis.

* Modifying Velocity and Acceleration



