use bevy::{
    app::{App, Plugin},
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    text::TextFont,
    utils::default,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    text_color: bevy::color::palettes::css::WHITE.into(),
                    enabled: true,
                    ..default()
                },
            },
        ));
    }
}
