use bevy::prelude::Entity;
use std::mem::size_of;

fn main() {
    print!("{}", size_of::<Option<Entity>>());
}
