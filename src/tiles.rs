use crate::bombs::*;

pub fn tile_builder(input: u8) -> Vec<Tile> {
  let bombs: Vec<Tile> = bomb_assignment(input);
  let mut tiles: Vec<Tile> = Vec::new();
  let mut confirm_push = false;

  for counter in 0..25 {
    let tile_struct = Tile {
      tile_type: IsBomb::NotBomb,
      bombs_around: 0,
      tile_name: name_assignment(counter),
      display: String::from("▮"),
    };

    for bomb in &bombs {
      if bomb.tile_name == tile_struct.tile_name {
        confirm_push = false;
        break;
      } else {
        confirm_push = true;
      }
    }

    if confirm_push {
      tiles.push(tile_struct);
    }
  }

  for merge in bombs {
    tiles.push(merge);
  }

  tile_ordering(tiles)
}

fn tile_ordering(tile_set: Vec<Tile>) -> Vec<Tile> {
  let mut filtered_tile_set: Vec<Tile> = Vec::new();
  let mut name_sorter: u8 = 0;

  while name_sorter <= 24 {
    let name = name_assignment(name_sorter);
    for tile in &tile_set {
      if tile.tile_name == name {
        filtered_tile_set.push(tile.clone());
        name_sorter += 1;
      }
    }
  }

  filtered_tile_set
}
