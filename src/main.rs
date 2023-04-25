use bevy::math::*;
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_prototype_lyon::prelude::*;
use shrinkwraprs::Shrinkwrap;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .register_type::<Anchor>()
        .register_type::<Acceleration>()
        .register_type::<Spring>()
        .register_type::<Velocity>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin::default())
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_system(velocity_system)
        .add_system(spring_system)
        .add_system(acceleration_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle {
                center: vec2(0.0, 0.0),
                radius: 10.0,
            }),
            ..default()
        },
        Fill::color(Color::RED),
        Stroke::new(Color::BLACK, 2.0),
        Anchor::default(),
        Spring(0.01),
        Velocity(vec2(10.0, 10.0)),
        Acceleration::default(),
    ));
}

fn spring_system(mut query: Query<(&mut Acceleration, &Spring, &Transform, &Anchor)>) {
    for (mut acceleration, &spring, transform, &anchor) in query.iter_mut() {
        let displacement = transform.translation.xy() - *anchor;
        *acceleration = Acceleration(-*spring * displacement);
    }
}

fn acceleration_system(mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.x += acceleration.x;
        velocity.y += acceleration.y;
    }
}

fn velocity_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

#[derive(Component, Clone, Copy, Default, Shrinkwrap, Reflect)]
struct Anchor(Vec2);

#[derive(Component, Clone, Copy, Default, Shrinkwrap, Reflect)]
struct Spring(f32);

#[derive(Component, Clone, Copy, Default, Shrinkwrap, Reflect)]
#[shrinkwrap(mutable)]
struct Velocity(Vec2);

#[derive(Component, Clone, Copy, Default, Shrinkwrap, Reflect)]
#[shrinkwrap(mutable)]
struct Acceleration(Vec2);
