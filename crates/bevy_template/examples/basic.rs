use bevy::prelude::*;
use bevy_template::prelude::*;
use serde::Deserialize;

#[derive(Reflect, Deserialize, Debug)]
#[reflect(InfoAny, Deserialize)]
struct TestInfo {
    value: i32,
}

impl Info for TestInfo {
    type Component = TestTrait;

    fn gen(&self) -> Self::Component {
        TestTrait { value: self.value }
    }
}

#[derive(Component, Reflect, Deserialize, Debug)]
#[reflect(Deserialize)]
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
        .add_systems(Startup, spawn)
        .add_systems(Update, (check, listener))
        .run();
}

#[allow(unused)]
fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_empty()
        .insert(asset_server.load::<Template>("test.temp.ron"));
}

fn spawn(mut commands: Commands) {
    let e = commands.spawn_template("test.temp.ron");
    info!("{:?}", e.id());
}

fn check(
    query: Query<(Entity, &TestTrait, &Handle<Template>), Added<TestTrait>>,
    templates: Res<Assets<Template>>,
) {
    for (entity, test_trait, handle) in query.iter() {
        info!("entity[{entity:?}] {test_trait:?}");
        let infos = &templates.get(handle).unwrap().infos;
        info!("{:?}", infos);
        info!("{:?}", infos.get::<TestInfo>());
    }
}

fn listener(mut events: EventReader<AssetEvent<Template>>, templates: Res<Assets<Template>>) {
    for event in events.read() {
        match event {
            AssetEvent::Added { id } => {
                info!("Added: {:?} ", id);
            }
            AssetEvent::Modified { id } => {
                info!("Modified: {:?}", id);
            }
            AssetEvent::Removed { id } => {
                info!("Removed: {:?}", id);
            }
            AssetEvent::Unused { id } => {
                info!("Unused: {:?}", id);
            }
            AssetEvent::LoadedWithDependencies { id } => {
                info!("LoadedWithDependencies: {:?} ", id);
                let template = templates.get(*id).unwrap();
                info!("Template: {:?}", template)
            }
        }
    }
}
