use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy_mod_sysfail::macros::*;
use std::io::Cursor;
use winit::window::Icon;

/// Overrides the default Bevy plugins and configures things like the screen settings.
pub struct BevyConfigPlugin;

impl Plugin for BevyConfigPlugin {
    fn build(&self, app: &mut App) {
        let default_plugins = DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (800., 600.).into(),
                title: "Foxtrot".to_string(),
                canvas: Some("#bevy".to_owned()),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        });
        #[cfg(feature = "native-dev")]
        let default_plugins = default_plugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        });
        app.insert_resource(Msaa::Sample4)
            .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
            .add_plugins(default_plugins)
            .add_system(set_window_icon.on_startup());
    }
}

// Sets the icon on Windows and X11
#[sysfail(log(level = "error"))]
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_windows: Query<Entity, With<PrimaryWindow>>,
) -> Result<()> {
    let primary_entity = primary_windows.single();
    let primary = windows
        .get_window(primary_entity)
        .context("Failed to get primary window")?;
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height)?;
        primary.set_window_icon(Some(icon));
    };
    Ok(())
}
