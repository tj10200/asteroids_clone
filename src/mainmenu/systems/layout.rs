use crate::game::player::PLAYER_SHIP;
use crate::game::sprite_loader::mapper::XMLSpriteSheetLoader;
use crate::main;
use crate::mainmenu::components::*;
use crate::mainmenu::styles::*;
use bevy::prelude::*;

pub const TITLE_SPRITE_IMAGE: &str = "sprites/title.png";

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sprite_loader: Res<XMLSpriteSheetLoader>,
) {
    let main_menu_entity = build_main_menu(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &sprite_loader,
    );
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    sprite_loader: &Res<XMLSpriteSheetLoader>,
) -> Entity {
    let main_menu_entity = commands
        .spawn(NodeBundle {
            style: main_menu_syle(),
            ..default()
        })
        .insert(MainMenu {})
        .with_children(|parent| {
            let texture_handle = asset_server.load(TITLE_SPRITE_IMAGE);
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(256.0, 155.0), 1, 1, None, None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            // Title
            parent
                .spawn(NodeBundle {
                    style: title_style(),
                    ..default()
                })
                .with_children(|parent| {
                    // Left-side image
                    parent.spawn(AtlasImageBundle {
                        style: Style {
                            width: Val::Px(150.),
                            height: Val::Px(100.),
                            ..default()
                        },
                        texture_atlas: texture_atlas_handle.clone(),
                        texture_atlas_image: UiTextureAtlasImage::default(),
                        ..default()
                    });
                    // Game Title
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Asteroids Clone",
                                get_title_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });

                    // Right-side image
                    parent.spawn(AtlasImageBundle {
                        style: Style {
                            width: Val::Px(150.),
                            height: Val::Px(100.),
                            ..default()
                        },
                        texture_atlas: texture_atlas_handle,
                        texture_atlas_image: UiTextureAtlasImage {
                            index: 0,
                            flip_x: true,
                            flip_y: true,
                        },
                        ..default()
                    });
                });
            // ==== Play Button ====
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style(),
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // ==== Quit Button ====
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style(),
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    main_menu_entity
}

fn atlas_image_bundle(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    sprite_loader: &Res<XMLSpriteSheetLoader>,
    sprite_name: &str,
    frame_cols: usize,
    frame_rows: usize,
    start_frame: usize,
    flip_x: bool,
    flip_y: bool,
) -> AtlasImageBundle {
    let texture_handle = asset_server.load(&sprite_loader.file);
    let sprite = sprite_loader.get_sprite(sprite_name).unwrap();
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(sprite.width, sprite.height),
        frame_cols,
        frame_rows,
        None,
        Some(Vec2::new(sprite.x, sprite.y)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    AtlasImageBundle {
        style: image_style(sprite.width, sprite.height),
        texture_atlas: texture_atlas_handle,
        texture_atlas_image: UiTextureAtlasImage::default(),
        ..default()
    }
}
