use crate::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoardMouseState>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                mouse_over_tiles.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
}

#[derive(Resource, Default)]
pub struct BoardMouseState {
    pub hovered_tile: Option<(usize, usize)>,
}

fn setup(
    mut commands: Commands,
    asset: Res<TextureAssets>,
    mut cameras: Query<(&Camera, &mut Transform)>,
) {
    let (_, mut camera_transform) = cameras.single_mut();
    camera_transform.translation = Vec3::new(TILE_SIZE * 4., TILE_SIZE * 4., 10.);
    let tiles = vec![
        0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0,
        1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0,
        1, 0, 1, 0,
    ];
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
            Tile { x, y },
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
            Tile { x, y: 8 },
        ));
    }
}

fn mouse_over_tiles(
    mut board_mouse_state: ResMut<BoardMouseState>,
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(&GlobalTransform, &Tile, &mut Sprite)>,
) {
    board_mouse_state.hovered_tile = None;
    if let (Some(position), Some((camera, camera_transform))) =
        (q_windows.single().cursor_position(), camera.iter().next())
    {
        let position = Vec2::new(position.x as f32, position.y as f32);
        let position = camera
            .viewport_to_world(camera_transform, position)
            .unwrap()
            .origin
            .truncate();
        for (g_transform, tile, mut sprite) in query.iter_mut() {
            let tile_position = g_transform.translation().truncate();
            let x = tile_position.x - HALF_TILE_SIZE;
            let y = tile_position.y - HALF_TILE_SIZE;
            if position.x > x
                && position.x < x + TILE_SIZE
                && position.y > y
                && position.y < y + TILE_SIZE
            {
                sprite.color = Color::rgb(0.5, 0.5, 0.5);
                board_mouse_state.hovered_tile = Some((tile.x, tile.y));
            } else {
                sprite.color = Color::WHITE;
            }
        }
    }
}
