use bevy::asset::{AssetIndex, StrongHandle, UntypedAssetId, UntypedHandle};
use bevy::prelude::{AssetId, Handle, TextureAtlasLayout};
use bevy::utils::Uuid;
use std::any::TypeId;
use std::mem::size_of;
use std::sync::Arc;

fn main() {
    // App::new()
    //     .add_plugins(DefaultPlugins.set(WindowPlugin {
    //         primary_window: Some(Window {
    //             present_mode: PresentMode::AutoNoVsync,
    //             resolution: WindowResolution::new(1920., 1080.),
    //             title: "SimpleWorld".into(),
    //             ..default()
    //         }),
    //         ..default()
    //     }))
    //     .add_plugins(editor::EditorPlugin)
    //     .init_resource::<ChunkSize>()
    //     .add_systems(Startup, test)
    //     .run()

    println!("{}", size_of::<AssetId<TextureAtlasLayout>>());
    println!("{}", size_of::<UntypedAssetId>());
    println!("{}", size_of::<Uuid>());
    println!("{}", size_of::<AssetIndex>());
    println!("{}", size_of::<TypeId>());
    println!("{}", size_of::<Handle<TextureAtlasLayout>>());
    println!("{}", size_of::<UntypedHandle>());
    println!("{}", size_of::<StrongHandle>());
    println!("{}", size_of::<Arc<i32>>());
}
