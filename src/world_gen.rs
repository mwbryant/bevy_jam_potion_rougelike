use crate::prelude::*;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_map)
            .add_system_set(SystemSet::on_enter(GameState::Main).with_system(spawn_room));
    }
}

pub struct MapDesc {
    x: usize,
    y: usize,
    map: Vec<Vec<MapTile>>,
}

pub fn create_map(mut commands: Commands) {
    let mut map = generate_map(5, 5);

    while matches!(map, Err(..)) {
        map = generate_map(5, 5);
    }

    let map = map.unwrap();
    commands.insert_resource(MapDesc { x: 0, y: 0, map });
}
fn spawn_room(
    mut commands: Commands,
    assets: Res<BackgroundAssets>,
    images: Res<Assets<Image>>,
    map: Res<MapDesc>,
) {
    let room = map.map[map.x][map.y];
    let image = match room {
        MapTile::NPipe => images.get(&assets.npipe.clone()).unwrap(),
        MapTile::EPipe => images.get(&assets.epipe.clone()).unwrap(),
        MapTile::NElbow => images.get(&assets.nelbow.clone()).unwrap(),
        MapTile::EElbow => images.get(&assets.eelbow.clone()).unwrap(),
        MapTile::SElbow => images.get(&assets.selbow.clone()).unwrap(),
        MapTile::WElbow => images.get(&assets.weblow.clone()).unwrap(),
        MapTile::NEnd => images.get(&assets.nend.clone()).unwrap(),
        MapTile::EEnd => images.get(&assets.eend.clone()).unwrap(),
        MapTile::SEnd => images.get(&assets.send.clone()).unwrap(),
        MapTile::WEnd => images.get(&assets.wend.clone()).unwrap(),
        MapTile::NTee => images.get(&assets.ntee.clone()).unwrap(),
        MapTile::ETee => images.get(&assets.etee.clone()).unwrap(),
        MapTile::STee => images.get(&assets.stee.clone()).unwrap(),
        MapTile::WTee => images.get(&assets.wtee.clone()).unwrap(),
        MapTile::Cross => images.get(&assets.cross.clone()).unwrap(),
        MapTile::Empty => images.get(&assets.empty.clone()).unwrap(),
    };
    assert!(image.texture_descriptor.format == TextureFormat::Rgba8UnormSrgb);

    let width = image.size().x as usize;
    let height = image.size().y as usize;

    let pixel_size = 0.8;
    let tile_size = 64.0;
    let offset = Vec3::new(
        -32.0 * tile_size * pixel_size,
        32.0 * tile_size * pixel_size,
        0.0,
    );

    for y in 0..height {
        for x in 0..width {
            let index = 4 * (x + y * width);
            let r = image.data[index];
            let g = image.data[index + 1];
            let b = image.data[index + 2];
            let index = color_to_tile_index(r, g, b);
            let id = commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite { index, ..default() },
                    texture_atlas: assets.tileset.clone(),
                    transform: Transform::from_translation(
                        Vec3::new(
                            x as f32 * tile_size * pixel_size,
                            y as f32 * -tile_size * pixel_size,
                            0.0,
                        ) + offset,
                    )
                    .with_scale(Vec3::splat(pixel_size)),
                    ..default()
                })
                .id();
            if index == 1 {
                commands
                    .entity(id)
                    .insert(CollisionShape::Cuboid {
                        half_extends: Vec2::splat(tile_size * pixel_size / 2.0).extend(1.0),
                        border_radius: None,
                    })
                    .insert(
                        CollisionLayers::all_masks::<PhysicLayer>().with_group(PhysicLayer::World),
                    )
                    .insert(RotationConstraints::lock())
                    .insert(RigidBody::Static);
            }
        }
    }
}

fn color_to_tile_index(r: u8, g: u8, b: u8) -> usize {
    match (r, g, b) {
        (0, 154, 83) => 1,
        _ => 0, //_ => unreachable!("Unknown Color {:?}", (r, g, b)),
    }
}
