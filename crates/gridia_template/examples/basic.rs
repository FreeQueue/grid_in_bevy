use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use gridia_template::prelude::*;

#[derive(Reflect, Deserialize, Debug)]
#[reflect(InfoAny, Deserialize)]
struct InfoA {
    value: String,
}

impl Info for InfoA {
    type Trait = TraitA;

    fn gen(&self) -> Self::Trait {
        TraitA {
            value: self.value.to_owned(),
        }
    }
}

#[derive(Component, Reflect, Serialize, Deserialize, Debug)]
#[reflect(Deserialize)]
struct TraitA {
    value: String,
}

impl Trait for TraitA {
    type Info = InfoA;
}

#[derive(Reflect, Deserialize, Debug)]
#[reflect(InfoAny, Deserialize)]
struct TestInfo {
    value: i32,
}

impl Info for TestInfo {
    type Trait = TestTrait;

    fn gen(&self) -> Self::Trait {
        TestTrait { value: self.value }
    }
}

#[derive(Component, Reflect, Serialize, Deserialize, Debug)]
#[reflect(Deserialize)]
struct TestTrait {
    value: i32,
}

impl Trait for TestTrait {
    type Info = TestInfo;
}

#[derive(Bundle, Reflect, Deserialize, Debug)]
#[reflect(Deserialize)]
struct TestBundle {
    test: TestTrait,
    a: TraitA,
}

#[derive(Component)]
struct Test<'a> {
    value: &'a TestInfo,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TemplatePlugin)
        .register_type::<TestInfo>()
        .register_type::<TestTrait>()
        .register_type::<InfoA>()
        .register_type::<TraitA>()
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

fn check(query: Query<(Entity, &TestTrait, &TraitA, &Infos), Changed<Infos>>) {
    for (entity, test_trait, trait_a, infos) in query.iter() {
        info!("entity[{entity:?}] {test_trait:?}  {trait_a:?}");
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
