use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Brendi".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Matt".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Grant".to_string()));
}

struct GreetTimer(Timer);
// Res = resource.
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

fn hello_world() {
    println!("hello world!");
}

// creating a Plugin for better Systems management.
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    // use from_seconds with true for repeat to repeat the timer.
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
