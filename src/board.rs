use bevy::render::camera::Viewport;

use crate::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoardMouseState>()
            .add_systems(OnExit(GameState::Menu), setup)
            .add_systems(Update, mouse_over_tiles)
            .add_systems(
                Update,
                move_board_to_left.run_if(in_state(GameState::Summoning)),
            )
            .add_systems(
                Update,
                (
                    move_board_to_center.run_if(in_state(GameState::Victory)),
                    move_board_to_center.run_if(in_state(GameState::Defeat)),
                    move_board_to_center.run_if(in_state(GameState::Battling)),
                ),
            )
            .add_systems(
                OnEnter(GameState::Summoning),
                enable_picking.run_if(in_state(GameState::Summoning)),
            )
            .add_systems(
                OnExit(GameState::Summoning),
                disable_picking.run_if(in_state(GameState::Summoning)),
            );
    }
}

#[derive(Component)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub can_place: bool,
    pub sprite: usize,
}

#[derive(Component)]
pub struct BorderTile {
    pub x: usize,
    pub sprite: usize,
}

#[derive(Resource, Default)]
pub struct BoardMouseState {
    pub hovered_tile: Option<(usize, usize)>,
    pub pickable_tile: Option<(usize, usize)>,
    pub can_place: bool,
}

fn disable_picking(mut board_mouse_state: ResMut<BoardMouseState>) {
    board_mouse_state.can_place = false;
}

fn enable_picking(mut board_mouse_state: ResMut<BoardMouseState>) {
    board_mouse_state.can_place = true;
}

fn setup(
    mut commands: Commands,
    asset: Res<TextureAssets>,
    mut cameras: Query<(&Camera, &mut Transform)>,
) {
    let (_, mut camera_transform) = cameras.single_mut();
    camera_transform.translation = Vec3::new(TILE_SIZE * 4., TILE_SIZE * 4., 10.);
    let tiles = vec![
        0, 1, 0, 1, 0, 1, 0, 1, // 0
        1, 0, 1, 0, 1, 0, 1, 0, // 1
        0, 1, 0, 1, 0, 1, 0, 1, // 2
        17, 16, 17, 16, 17, 16, 17, 16, // 3
        16, 17, 16, 17, 16, 17, 16, 17, // 4
        17, 16, 17, 16, 17, 16, 17, 16, // 5
        16, 17, 16, 17, 16, 17, 16, 17, // 6
        17, 16, 17, 16, 17, 16, 17, 16, // 7
    ];
    commands.spawn(SpriteBundle {
        texture: asset.background.clone(),
        transform: Transform::from_scale(Vec3::new(2., 2., 1.))
            .with_translation(Vec3::new(0., 0., -1.)),
        ..Default::default()
    });
    for (i, tile) in tiles.iter().enumerate() {
        let x = i % 8;
        let y = i / 8;
        commands.spawn((
            SpriteSheetBundle {
                texture: asset.board.clone(),
                atlas: TextureAtlas {
                    index: *tile,
                    layout: asset.board_layout.clone(),
                },
                transform: Transform::from_translation(
                    tile_position_to_translation(x as i32, y as i32).extend(0.),
                ),
                ..Default::default()
            },
            Tile {
                x,
                y,
                can_place: y <= 2,
                sprite: *tile,
            },
        ));
    }
    let tile_borders = vec![8, 9, 8, 9, 8, 9, 8, 9];
    for (x, tile) in tile_borders.iter().enumerate() {
        commands.spawn((
            SpriteSheetBundle {
                texture: asset.board.clone(),
                atlas: TextureAtlas {
                    index: *tile,
                    layout: asset.board_layout.clone(),
                },
                transform: Transform::from_translation(
                    tile_position_to_translation(x as i32, -1).extend(0.),
                ),
                ..Default::default()
            },
            BorderTile { x, sprite: *tile },
        ));
    }
}

const SUMMONING_CAMERA_POSITION: (f32, f32) = (TILE_SIZE * 4. + WINDOW_SIZE.0 / 4., TILE_SIZE * 3.);

fn move_board_to_left(mut camera: Query<&mut Transform, With<Camera>>) {
    for mut transform in camera.iter_mut() {
        if transform.translation.x < SUMMONING_CAMERA_POSITION.0 {
            transform.translation.x += 1.;
        }
        transform.translation.y = SUMMONING_CAMERA_POSITION.1;
    }
}

const BATTLING_CAMERA_POSITION: (f32, f32) = (TILE_SIZE * 4., TILE_SIZE * 3.);

fn move_board_to_center(mut camera: Query<&mut Transform, With<Camera>>) {
    for mut transform in camera.iter_mut() {
        if transform.translation.x > BATTLING_CAMERA_POSITION.0 {
            transform.translation.x -= 1.;
        }
        transform.translation.y = BATTLING_CAMERA_POSITION.1;
    }
}

fn mouse_over_tiles(
    mut board_mouse_state: ResMut<BoardMouseState>,
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(
        &GlobalTransform,
        Option<&Tile>,
        Option<&BorderTile>,
        &mut Sprite,
    )>,
) {
    board_mouse_state.hovered_tile = None;
    board_mouse_state.pickable_tile = None;
    if board_mouse_state.can_place == false {
        return;
    }
    if let (Some(position), Some((camera, camera_transform))) =
        (q_windows.single().cursor_position(), camera.iter().next())
    {
        let position = Vec2::new(position.x as f32, position.y as f32);
        let ray = camera.viewport_to_world(camera_transform, position);
        if ray.is_none() {
            return;
        }
        let position = ray.unwrap().origin.truncate();
        for (g_transform, m_tile, m_border, mut sprite) in query.iter_mut() {
            if m_tile.is_none() && m_border.is_none() {
                continue;
            }
            if let Some(tile) = m_tile {
                let tile_position = g_transform.translation().truncate();
                let x = tile_position.x - HALF_TILE_SIZE;
                let y = tile_position.y - HALF_TILE_SIZE;
                if position.x > x
                    && position.x < x + TILE_SIZE
                    && position.y > y
                    && position.y < y + TILE_SIZE
                {
                    board_mouse_state.hovered_tile = Some((tile.x, tile.y));
                    if tile.can_place {
                        sprite.color = Color::rgb(0.5, 0.5, 0.5);
                        board_mouse_state.pickable_tile = Some((tile.x, tile.y));
                    } else {
                        sprite.color = Color::rgb(0.95, 0.95, 0.95);
                    }
                } else {
                    sprite.color = Color::WHITE;
                }
            } else if let Some(_border) = m_border {
                let tile_position = g_transform.translation().truncate();
                let x = tile_position.x - HALF_TILE_SIZE;
                let y = tile_position.y - HALF_TILE_SIZE + TILE_SIZE;
                if position.x > x
                    && position.x < x + TILE_SIZE
                    && position.y > y
                    && position.y < y + TILE_SIZE
                {
                    sprite.color = Color::rgb(0.5, 0.5, 0.5);
                } else {
                    sprite.color = Color::WHITE;
                }
            }
        }
    }
}
