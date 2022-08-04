use bevy::prelude::*;

fn hello_world() {
    println!("hello nerds!")
}

fn main() {
    App::new().add_system(hello_world).run();
}
