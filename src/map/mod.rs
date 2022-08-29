use bevy::prelude::Plugin;
use bevy_procedural::{CellLocation, Direction, SPCell, SuperPositionGrid};
use bevy::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum MapTile {
    //Direction of pipe is first exit, second is two directions clockwise
    NPipe,
    EPipe,

    //Direction of the elbow is first exit, second is one direction clockwise
    NElbow,
    EElbow,
    SElbow,
    WElbow,

    //Direction is the exit of the dead end
    NEnd,
    EEnd,
    SEnd,
    WEnd,

    //Direction is the left most branch of the Tee, second and third exits are the two directions adjacent clockwise
    NTee,
    ETee,
    STee,
    WTee,
    Cross,
    Empty,
}

impl MapTile {
    pub fn all() -> Vec<MapTile> {
        vec![
            //Direction of pipe is first exit, second is two directions counter-clockwise
            Self::NPipe,
            Self::EPipe,
            //Direction of the elbow is first exit, second is one direction counter-clockwise
            Self::NElbow,
            Self::EElbow,
            Self::SElbow,
            Self::WElbow,
            //Direction is the exit of the dead end
            Self::NEnd,
            Self::EEnd,
            Self::SEnd,
            Self::WEnd,
            //Direction is the right most branch of the Tee, second and third exits are the two directions adjacent counter-clockwise
            Self::NTee,
            Self::ETee,
            Self::STee,
            Self::WTee,
            Self::Empty,
            Self::Cross,
        ]
    }

    pub fn no_end() -> Vec<MapTile> {
        vec![
            //Direction of pipe is first exit, second is two directions clockwise
            Self::NPipe,
            Self::EPipe,
            //Direction of the elbow is first exit, second is one direction clockwise
            Self::NElbow,
            Self::EElbow,
            Self::SElbow,
            Self::WElbow,
            //Direction is the left most branch of the Tee, second and third exits are the two directions adjacent clockwise
            Self::NTee,
            Self::ETee,
            Self::STee,
            Self::WTee,
            Self::Cross,
            Self::Empty,
        ]
    }
    pub fn conns_to(&self, dir: Direction) -> bool {
        MapTile::tile_connections(self).contains(&dir)
    }

    pub fn tile_connections(tile: &MapTile) -> &[Direction] {
        match *tile {
            MapTile::NPipe => &[Direction::North, Direction::South],
            MapTile::EPipe => &[Direction::West, Direction::East],
            MapTile::NElbow => &[Direction::North, Direction::West],
            MapTile::EElbow => &[Direction::East, Direction::North],
            MapTile::SElbow => &[Direction::South, Direction::East],
            MapTile::WElbow => &[Direction::West, Direction::South],
            MapTile::NEnd => &[Direction::North],
            MapTile::EEnd => &[Direction::East],
            MapTile::SEnd => &[Direction::South],
            MapTile::WEnd => &[Direction::West],
            MapTile::NTee => &[Direction::North, Direction::West, Direction::South],
            MapTile::ETee => &[Direction::East, Direction::North, Direction::West],
            MapTile::STee => &[Direction::South, Direction::East, Direction::North],
            MapTile::WTee => &[Direction::West, Direction::South, Direction::East],
            MapTile::Empty => &[],
            MapTile::Cross => &[
                Direction::West,
                Direction::North,
                Direction::East,
                Direction::South,
            ],
        }
    }

    pub fn can_connect_to(tile: &SPCell<MapTile>, dir: Direction) -> bool {
        for position in &tile.positions {
            if MapTile::conns_to(position, dir) {
                return true;
            }
        }

        false
    }
    pub fn remove_positions_conn_to(dir: Direction, positions: Vec<MapTile>) -> Vec<MapTile> {
        positions
            .into_iter()
            .filter(|position| !position.conns_to(dir))
            .collect()
    }

    pub fn remove_empty(positions: Vec<MapTile>) -> Vec<MapTile> {
        positions
            .into_iter()
            .filter(|position| *position != MapTile::Empty)
            .collect()
    }
    pub fn filter_for_neighbor(
        neighbor_dir: Direction,
        positions: Vec<MapTile>,
        neighbor: &SPCell<MapTile>,
    ) -> Vec<MapTile> {
        //Get the available connections
        let mut filtered_positions = positions.clone();

        match neighbor_dir {
            Direction::North => {
                if !MapTile::can_connect_to(neighbor, Direction::South) {
                    filtered_positions = MapTile::remove_positions_conn_to(neighbor_dir, positions);
                } else {
                }
            }
            Direction::South => {
                if !MapTile::can_connect_to(neighbor, Direction::North) {
                    filtered_positions = MapTile::remove_positions_conn_to(neighbor_dir, positions);
                } else {
                }
            }
            Direction::East => {
                if !MapTile::can_connect_to(neighbor, Direction::West) {
                    filtered_positions = MapTile::remove_positions_conn_to(neighbor_dir, positions);
                } else {
                }
            }
            Direction::West => {
                if !MapTile::can_connect_to(neighbor, Direction::East) {
                    filtered_positions = MapTile::remove_positions_conn_to(neighbor_dir, positions);
                } else {
                }
            }
        }

        filtered_positions
    }
}

pub fn collapse_map_geometry(
    grid: &SuperPositionGrid<MapTile>,
    cell: &SPCell<MapTile>,
) -> Vec<MapTile> {
    let mut new_positions = cell.positions.clone();

    let north = grid.neighbor(cell, Direction::North.unit());
    let south = grid.neighbor(cell, Direction::South.unit());
    let east = grid.neighbor(cell, Direction::East.unit());
    let west = grid.neighbor(cell, Direction::West.unit());

    new_positions = match south {
        //Remove all positions that require a connection to the north if we're on the edge
        None => MapTile::remove_positions_conn_to(Direction::South, new_positions),
        Some(neighbor) => MapTile::filter_for_neighbor(Direction::South, new_positions, neighbor),
    };

    new_positions = match north {
        None => new_positions
            .into_iter()
            .filter(|position| !position.conns_to(Direction::North))
            .collect(),

        Some(neighbor) => MapTile::filter_for_neighbor(Direction::North, new_positions, neighbor),
    };

    new_positions = match west {
        None => new_positions
            .into_iter()
            .filter(|position| !position.conns_to(Direction::West))
            .collect(),

        Some(neighbor) => MapTile::filter_for_neighbor(Direction::West, new_positions, neighbor),
    };

    new_positions = match east {
        None => new_positions
            .into_iter()
            .filter(|position| !position.conns_to(Direction::East))
            .collect(),

        Some(neighbor) => MapTile::filter_for_neighbor(Direction::East, new_positions, neighbor),
    };
    new_positions
}

pub fn no_end_table_value() -> (Vec<MapTile>, Vec<f32>) {
    (
        MapTile::no_end(),
        vec![0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
    )
}

pub fn all_table_values() -> (Vec<MapTile>, Vec<f32>) {
    (
        MapTile::all(),
        vec![
            0.5, 0.5, 0.5, 0.5, 0.5, 0.8, 0.8, 0.8, 0.8, 0.8, 0.8, 0.8, 0.8, 0.5, 0.01, 0.5,
        ],
    )
}

fn init_grid(height: usize, width: usize) -> SuperPositionGrid<MapTile> {
    SuperPositionGrid::new(height, width, &MapTile::all())
}

pub fn generate_map(
    width: usize,
    height: usize,
) -> Result<Vec<Vec<MapTile>>, bevy_procedural::PossibilityError> {
    let mut grid = init_grid(height, width);
    let table = no_end_table_value();

    grid.override_positions(CellLocation { x: 3, y: 3 }, vec![MapTile::Cross]);

    grid.full_collapse(collapse_map_geometry, (&table.0, &table.1));

    grid.render()
}

pub struct MapPlugin;

impl Plugin for MapPlugin{
    fn build(&self, app: &mut App) {
        todo!()
    }
}