#[cfg(test)]
mod tests {
    use bevy_procedural::*;
    use potion_roguelike::map::*;

    fn init_grid(height: usize, width: usize) -> SuperPositionGrid<MapTile> {
        SuperPositionGrid::new(height, width, &MapTile::all())
    }

    #[test]
    fn test_edge_boundaries(){
        let mut grid = init_grid(5, 5);
        let table = no_end_table_value();

        grid.full_collapse(collapse_map_geometry, (&table.0, &table.1));

        let result= grid.peek_render();
        result.iter().for_each(|row| {
            row.iter().for_each(|cell|{
                let positions = cell.positions.clone();
                print!("{positions:?}");
            });
            
            println!();
        })
    }

    #[test]
    fn test_map_generator() {
        let mut grid = init_grid(20, 29);
        let table = no_end_table_value();

        grid.full_collapse(collapse_map_geometry, (&table.0, &table.1));

        let result= grid.peek_render();
        result.iter().for_each(|row| {
            row.iter().for_each(|cell|{
                print!("{cell:?}");
            });
            
            println!();
        })
    }
}
