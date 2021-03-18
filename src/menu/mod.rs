use bevy::{ecs::component::Component, prelude::*};

use crate::AppState;
use crate::UiAssets;

pub mod main_menu;
pub mod settings;

/// Every logical action for which we can have a UI button
///
/// Use as marker components to identify the buttons.
pub mod button {
    pub struct EnterGame;
    pub struct ExitApp;
    pub struct OpenSettingsMenu;
    pub struct ExitSettingsMenu;
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // Main menu
        app.add_system_set(
            SystemSet::on_enter(AppState::MainMenu).with_system(main_menu::setup.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(
                    button_interact::<button::ExitApp>
                        .system()
                        .chain(main_menu::button_exit_app.system()),
                )
                .with_system(
                    button_interact::<button::EnterGame>
                        .system()
                        .chain(main_menu::button_enter_game.system()),
                )
                .with_system(
                    button_interact::<button::OpenSettingsMenu>
                        .system()
                        .chain(main_menu::button_open_settings_menu.system()),
                ),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu)
                .with_system(crate::despawn_all::<main_menu::StateCleanup>.system()),
        );

        // Settings menu
        app.add_system_set(
            SystemSet::on_enter(AppState::SettingsMenu).with_system(settings::setup.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::SettingsMenu).with_system(
                button_interact::<button::ExitSettingsMenu>
                    .system()
                    .chain(settings::button_exit_settings_menu.system()),
            ),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::SettingsMenu)
                .with_system(crate::despawn_all::<settings::StateCleanup>.system()),
        );
    }
}

pub fn button_interact<B: Component>(
    materials: Res<UiAssets>,
    mut query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>, With<B>),
    >,
) -> bool {
    let mut clicked = false;

    for (interaction, mut material) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *material = materials.button_active.clone();
                clicked = true;
            }
            Interaction::Hovered => *material = materials.button_hover.clone(),
            Interaction::None => *material = materials.button_normal.clone(),
        }
    }

    clicked
}
