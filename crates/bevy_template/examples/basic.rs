use bevy::prelude::*;

use bevy_template::prelude::*;

#[derive(Reflect)]
#[reflect(InfoAny)]
struct TestInfo {
    value: i32,
}

impl Info for TestInfo {
    type Component = TestTrait;

    fn gen(&self) -> Self::Component {
        TestTrait { value: self.value }
    }
}

#[derive(Component, Reflect)]
struct TestTrait {
    value: i32,
}

impl InfoComponent for TestTrait {
    type Info = TestInfo;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TemplatePlugin)
        .register_type::<TestInfo>()
        .register_type::<TestTrait>()
        .add_systems(Startup, load)
        .add_systems(Update, (check, listener))
        .run();
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_empty()
        .insert(asset_server.load::<Template>("test.temp.ron"));
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_empty()
        .insert_template(asset_server.load("test.temp.ron"));
}

fn check(query: Query<&TestTrait>) {
    for test_trait in query.iter() {
        println!("value: {}", test_trait.value);
    }
}

fn listener(mut events: EventReader<AssetEvent<Template>>) {
    for event in events.read() {
        match event {
            AssetEvent::Added { id } => {
                println!("Added: {:?}", id);
            }
            AssetEvent::Modified { id } => {
                println!("Modified: {:?}", id);
            }
            AssetEvent::Removed { id } => {
                println!("Removed: {:?}", id);
            }

            AssetEvent::Unused { id } => {
                println!("Unused: {:?}", id);
            }
            AssetEvent::LoadedWithDependencies { id } => {
                println!("LoadedWithDependencies: {:?} ", id);
            }
        }
    }
}
