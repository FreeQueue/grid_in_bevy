use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_editor_pls::controls::EditorControls;
use bevy_editor_pls::default_windows::cameras::camera_3d_free::FlycamControls;
use bevy_editor_pls::egui;

use bevy_egui::EguiContexts;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_editor_pls::EditorPlugin::default());

        app.add_plugins((FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin));

        // Setup Controls
        app.insert_resource(res_editor_controls());
        app.add_systems(Startup, setup_editor_camera_controls);

        // Setup Egui Style
        // app.add_systems(Startup, setup_egui_style);
    }
}

fn res_editor_controls() -> EditorControls {
    use bevy_editor_pls::controls::*;
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(Action::PlayPauseEditor);

    editor_controls.insert(
        Action::PlayPauseEditor,
        Binding {
            input: UserInput::Single(Button::Keyboard(KeyCode::Escape)),
            conditions: vec![BindingCondition::ListeningForText(false)],
        },
    );

    editor_controls
}

fn setup_editor_camera_controls(mut query: Query<&mut FlycamControls>) {
    let mut controls = query.single_mut();
    controls.key_up = KeyCode::KeyE;
    controls.key_down = KeyCode::KeyQ;
}

fn ui_example_system(mut ctx: EguiContexts) {
    egui::Window::new("Hello").show(ctx.ctx_mut(), |ui| {
        ui.label("world");

        if ui.button("text").clicked() {}
    });
}
