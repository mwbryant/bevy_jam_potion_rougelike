use rand::Rng;

use crate::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_map)
            .add_event::<ExitEvent>()
            .add_system(exit_collision)
            .add_system(fadeout)
            .add_system_set(SystemSet::on_update(GameState::Main).with_system(load_next_room))
            .add_system_set(SystemSet::on_enter(GameState::Main).with_system(spawn_start_room));
    }
}

#[derive(Clone)]
pub struct ExitEvent(ExitDirection);

#[derive(Component)]
pub struct ScreenFade {
    pub event: ExitEvent,
    pub timer: Timer,
    pub sent: bool,
    pub alpha: f32,
}

fn fadeout(
    mut commands: Commands,
    mut fade_query: Query<(Entity, &mut ScreenFade, &mut Sprite)>,
    mut events: EventWriter<ExitEvent>,
    time: Res<Time>,
) {
    for (entity, mut fade, mut sprite) in fade_query.iter_mut() {
        fade.timer.tick(time.delta());
        if fade.timer.percent() < 0.5 {
            fade.alpha = fade.timer.percent() * 2.0;
        } else {
            fade.alpha = fade.timer.percent_left() * 2.0;
        }
        sprite.color.set_a(fade.alpha);

        if fade.timer.percent() > 0.5 && !fade.sent {
            fade.sent = true;
            events.send(fade.event.clone());
        }

        if fade.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Component)]
pub struct RoomMember;

pub struct MapDesc {
    x: usize,
    y: usize,
    map: Vec<Vec<MapTile>>,
}

fn load_next_room(
    mut commands: Commands,
    to_despawn: Query<Entity, With<RoomMember>>,
    mut event: EventReader<ExitEvent>,
    game_assets: Res<GameAssets>,
    bg_assets: Res<BackgroundAssets>,
    images: Res<Assets<Image>>,
    mut player: Query<&mut Transform, With<Player>>,
    mut map: ResMut<MapDesc>,
) {
    for event in event.iter() {
        for ent in &to_despawn {
            commands.entity(ent).despawn_recursive();
        }
        match event.0 {
            ExitDirection::North => map.y += 1,
            ExitDirection::South => map.y -= 1,
            ExitDirection::East => map.x += 1,
            ExitDirection::West => map.x -= 1,
        }
        let mut player = player.single_mut();
        let bounds = 28.0 * 0.8 * 64.0;
        match event.0 {
            ExitDirection::North => player.translation.y = -bounds,
            ExitDirection::South => player.translation.y = bounds,
            ExitDirection::East => player.translation.x = -bounds,
            ExitDirection::West => player.translation.x = bounds,
        }
        let bounds = bounds * 0.7;

        let mut frog_pos = Vec::default();
        //Spawn some frogs
        for i in 0..rand::thread_rng().gen_range(3..7) {
            frog_pos.push(Vec3::new(
                rand::thread_rng().gen_range(-bounds..bounds),
                rand::thread_rng().gen_range(-bounds..bounds),
                0.0,
            ));
        }
        let mut bat_pos = Vec::default();
        //Spawn some bats
        for i in 0..rand::thread_rng().gen_range(3..6) {
            bat_pos.push(Vec3::new(
                rand::thread_rng().gen_range(-bounds..bounds),
                rand::thread_rng().gen_range(-bounds..bounds),
                0.0,
            ));
        }

        spawn_room(
            &mut commands,
            &bg_assets,
            &images,
            &map,
            &mut frog_pos,
            &mut bat_pos,
        );

        for pos in frog_pos {
            spawn_frog(&mut commands, &game_assets, pos);
        }
        for pos in bat_pos {
            spawn_bat(&mut commands, &game_assets, pos);
        }
    }
}

fn exit_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    fade: Query<&ScreenFade>,
    exits: Query<&ExitDirection>,
    player: Query<(), With<Player>>,
) {
    if fade.iter().count() != 0 {
        return;
    }
    for event in collision_events.iter() {
        if let CollisionEvent::Started(d1, d2) = event {
            if exits.contains(d1.rigid_body_entity()) {
                //ugh
                if let Ok(dir) = exits.get(d1.rigid_body_entity()) {
                    if let Ok(()) = player.get(d2.rigid_body_entity()) {
                        commands
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::rgba(0.2, 0.2, 0.2, 0.0),
                                    ..default()
                                },
                                transform: Transform {
                                    translation: Vec3::new(0.0, 0.0, 999.0),
                                    scale: Vec3::splat(10000.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(ScreenFade {
                                alpha: 0.0,
                                sent: false,
                                event: (ExitEvent(*dir)),
                                timer: Timer::from_seconds(0.7, false),
                            })
                            .insert(Name::new("Fadeout"));
                        return;
                    }
                }
            }
            if exits.contains(d2.rigid_body_entity()) {
                if let Ok(dir) = exits.get(d2.rigid_body_entity()) {
                    if let Ok(()) = player.get(d1.rigid_body_entity()) {
                        commands
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::rgba(0.2, 0.2, 0.2, 0.0),
                                    ..default()
                                },
                                transform: Transform {
                                    translation: Vec3::new(0.0, 0.0, 999.0),
                                    scale: Vec3::splat(10000.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(ScreenFade {
                                alpha: 0.0,
                                sent: false,
                                event: (ExitEvent(*dir)),
                                timer: Timer::from_seconds(0.7, false),
                            })
                            .insert(Name::new("Fadeout"));
                        return;
                    }
                }
            }
        }
    }
}

pub fn create_map(mut commands: Commands) {
    //TODO solve bugs with the map generator
    //Workaround for now is to just hard code a single good map
    /*     let mut map = generate_map(5, 5);

    while matches!(map, Err(..)) {
        map = generate_map(5, 5);
    }

    let map = map.unwrap(); */
    let map = vec![
        vec![
            MapTile::EElbow,
            MapTile::EPipe,
            MapTile::WTee,
            MapTile::NElbow,
            MapTile::NEnd,
        ],
        vec![
            MapTile::SElbow,
            MapTile::ETee,
            MapTile::Cross,
            MapTile::Cross,
            MapTile::WElbow,
        ],
        vec![
            MapTile::EEnd,
            MapTile::NTee,
            MapTile::NPipe,
            MapTile::SElbow,
            MapTile::NElbow,
        ],
        vec![
            MapTile::EElbow,
            MapTile::Cross,
            MapTile::WTee,
            MapTile::ETee,
            MapTile::NTee,
        ],
        vec![
            MapTile::SElbow,
            MapTile::WElbow,
            MapTile::Empty,
            MapTile::SEnd,
            MapTile::SEnd,
        ],
    ];
    commands.insert_resource(MapDesc { x: 0, y: 0, map });
}
fn spawn_start_room(
    mut commands: Commands,
    assets: Res<BackgroundAssets>,
    images: Res<Assets<Image>>,
    game_assets: Res<GameAssets>,
    map: Res<MapDesc>,
) {
    spawn_room(
        &mut commands,
        &assets,
        &images,
        &map,
        &mut Vec::default(),
        &mut Vec::default(),
    );
    let pos = Vec3::new(200., 200.0, 0.0);
    spawn_frog(&mut commands, &game_assets, pos);
}

fn spawn_room(
    commands: &mut Commands,
    assets: &Res<BackgroundAssets>,
    images: &Res<Assets<Image>>,
    map: &MapDesc,
    frogs_to_check: &mut Vec<Vec3>,
    bats_to_check: &mut Vec<Vec3>,
) {
    let room = map.map[map.y][map.x];
    println!("Loading {:?}", room);
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

    let mut ids = Vec::default();
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
                .insert(RoomMember)
                .id();
            ids.push(id);
            if index == 1 {
                let x = x as f32 * tile_size * pixel_size + offset.x;
                let y = y as f32 * -tile_size * pixel_size + offset.y;
                frogs_to_check.retain(|pos| {
                    !((pos.x - x).abs() < tile_size * pixel_size * 1.5
                        && (pos.y - y).abs() < tile_size * pixel_size * 1.5)
                });
                bats_to_check.retain(|pos| {
                    !((pos.x - x).abs() < tile_size * pixel_size * 1.5
                        && (pos.y - y).abs() < tile_size * pixel_size * 1.5)
                });
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
    commands
        .spawn_bundle(SpatialBundle::default())
        .insert(RoomMember)
        .push_children(&ids);
}

fn color_to_tile_index(r: u8, g: u8, b: u8) -> usize {
    match (r, g, b) {
        (11, 61, 38) | (16, 121, 15) | (76, 90, 84) => 0,
        _ => 1, //_ => unreachable!("Unknown Color {:?}", (r, g, b)),
    }
}
