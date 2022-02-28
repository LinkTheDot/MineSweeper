use crate::bombs::*;
use crate::tiles::*;

pub fn tile_adder() -> Vec<Tile> {
  let mut tile_set = tile_builder();
  let mut numbered_tile_set = tile_set.clone();
  let mut counter = 0;

  for tile in &tile_set {
    if tile.tile_type == IsBomb::Bomb {
      match tile.tile_name.as_str() {
        "a1" => {
          numbered_tile_set[1].bombs_around += 1;
          numbered_tile_set[5].bombs_around += 1;
          numbered_tile_set[6].bombs_around += 1;
        }
        "a2" => {
          numbered_tile_set[0].bombs_around += 1;
          numbered_tile_set[2].bombs_around += 1;
          numbered_tile_set[5].bombs_around += 1;
          numbered_tile_set[6].bombs_around += 1;
          numbered_tile_set[7].bombs_around += 1;
        }
        "a3" => {
          numbered_tile_set[1].bombs_around += 1;
          numbered_tile_set[3].bombs_around += 1;
          numbered_tile_set[6].bombs_around += 1;
          numbered_tile_set[7].bombs_around += 1;
          numbered_tile_set[8].bombs_around += 1;
        }
        "a4" => {
          numbered_tile_set[2].bombs_around += 1;
          numbered_tile_set[4].bombs_around += 1;
          numbered_tile_set[7].bombs_around += 1;
          numbered_tile_set[8].bombs_around += 1;
          numbered_tile_set[9].bombs_around += 1;
        }
        "a5" => {
          numbered_tile_set[3].bombs_around += 1;
          numbered_tile_set[8].bombs_around += 1;
          numbered_tile_set[9].bombs_around += 1;
        }
        "b1" => {
          numbered_tile_set[0].bombs_around += 1;
          numbered_tile_set[1].bombs_around += 1;
          numbered_tile_set[6].bombs_around += 1;
          numbered_tile_set[10].bombs_around += 1;
          numbered_tile_set[11].bombs_around += 1;
        }
        "b2" => {
          numbered_tile_set[0].bombs_around += 1;
          numbered_tile_set[1].bombs_around += 1;
          numbered_tile_set[2].bombs_around += 1;
          numbered_tile_set[5].bombs_around += 1;
          numbered_tile_set[7].bombs_around += 1;
          numbered_tile_set[10].bombs_around += 1;
          numbered_tile_set[11].bombs_around += 1;
          numbered_tile_set[12].bombs_around += 1;
        }
        "b3" => {
          numbered_tile_set[1].bombs_around += 1;
          numbered_tile_set[2].bombs_around += 1;
          numbered_tile_set[3].bombs_around += 1;
          numbered_tile_set[6].bombs_around += 1;
          numbered_tile_set[8].bombs_around += 1;
          numbered_tile_set[11].bombs_around += 1;
          numbered_tile_set[12].bombs_around += 1;
          numbered_tile_set[13].bombs_around += 1;
        }
        "b4" => {
          numbered_tile_set[2].bombs_around += 1;
          numbered_tile_set[3].bombs_around += 1;
          numbered_tile_set[4].bombs_around += 1;
          numbered_tile_set[7].bombs_around += 1;
          numbered_tile_set[9].bombs_around += 1;
          numbered_tile_set[12].bombs_around += 1;
          numbered_tile_set[13].bombs_around += 1;
          numbered_tile_set[14].bombs_around += 1;
        }
        "b5" => {
          numbered_tile_set[3].bombs_around += 1;
          numbered_tile_set[4].bombs_around += 1;
          numbered_tile_set[8].bombs_around += 1;
          numbered_tile_set[13].bombs_around += 1;
          numbered_tile_set[14].bombs_around += 1;
        }
        "c1" => {
          numbered_tile_set[5].bombs_around += 1;
          numbered_tile_set[6].bombs_around += 1;
          numbered_tile_set[11].bombs_around += 1;
          numbered_tile_set[15].bombs_around += 1;
          numbered_tile_set[16].bombs_around += 1;
        }
        "c2" => {
          numbered_tile_set[5].bombs_around += 1;
          numbered_tile_set[6].bombs_around += 1;
          numbered_tile_set[7].bombs_around += 1;
          numbered_tile_set[10].bombs_around += 1;
          numbered_tile_set[12].bombs_around += 1;
          numbered_tile_set[15].bombs_around += 1;
          numbered_tile_set[16].bombs_around += 1;
          numbered_tile_set[17].bombs_around += 1;
        }
        "c3" => {
          numbered_tile_set[6].bombs_around += 1;
          numbered_tile_set[7].bombs_around += 1;
          numbered_tile_set[8].bombs_around += 1;
          numbered_tile_set[11].bombs_around += 1;
          numbered_tile_set[13].bombs_around += 1;
          numbered_tile_set[16].bombs_around += 1;
          numbered_tile_set[17].bombs_around += 1;
          numbered_tile_set[18].bombs_around += 1;
        }
        "c4" => {
          numbered_tile_set[7].bombs_around += 1;
          numbered_tile_set[8].bombs_around += 1;
          numbered_tile_set[9].bombs_around += 1;
          numbered_tile_set[12].bombs_around += 1;
          numbered_tile_set[14].bombs_around += 1;
          numbered_tile_set[17].bombs_around += 1;
          numbered_tile_set[18].bombs_around += 1;
          numbered_tile_set[19].bombs_around += 1;
        }
        "c5" => {
          numbered_tile_set[8].bombs_around += 1;
          numbered_tile_set[9].bombs_around += 1;
          numbered_tile_set[13].bombs_around += 1;
          numbered_tile_set[18].bombs_around += 1;
          numbered_tile_set[19].bombs_around += 1;
        }
        "d1" => {
          numbered_tile_set[10].bombs_around += 1;
          numbered_tile_set[11].bombs_around += 1;
          numbered_tile_set[16].bombs_around += 1;
          numbered_tile_set[20].bombs_around += 1;
          numbered_tile_set[21].bombs_around += 1;
        }
        "d2" => {
          numbered_tile_set[10].bombs_around += 1;
          numbered_tile_set[11].bombs_around += 1;
          numbered_tile_set[12].bombs_around += 1;
          numbered_tile_set[15].bombs_around += 1;
          numbered_tile_set[17].bombs_around += 1;
          numbered_tile_set[20].bombs_around += 1;
          numbered_tile_set[21].bombs_around += 1;
          numbered_tile_set[22].bombs_around += 1;
        }
        "d3" => {
          numbered_tile_set[11].bombs_around += 1;
          numbered_tile_set[12].bombs_around += 1;
          numbered_tile_set[13].bombs_around += 1;
          numbered_tile_set[16].bombs_around += 1;
          numbered_tile_set[18].bombs_around += 1;
          numbered_tile_set[21].bombs_around += 1;
          numbered_tile_set[22].bombs_around += 1;
          numbered_tile_set[23].bombs_around += 1;
        }
        "d4" => {
          numbered_tile_set[12].bombs_around += 1;
          numbered_tile_set[13].bombs_around += 1;
          numbered_tile_set[14].bombs_around += 1;
          numbered_tile_set[17].bombs_around += 1;
          numbered_tile_set[19].bombs_around += 1;
          numbered_tile_set[22].bombs_around += 1;
          numbered_tile_set[23].bombs_around += 1;
          numbered_tile_set[24].bombs_around += 1;
        }
        "d5" => {
          numbered_tile_set[13].bombs_around += 1;
          numbered_tile_set[14].bombs_around += 1;
          numbered_tile_set[18].bombs_around += 1;
          numbered_tile_set[23].bombs_around += 1;
          numbered_tile_set[24].bombs_around += 1;
        }
        "e1" => {
          numbered_tile_set[15].bombs_around += 1;
          numbered_tile_set[16].bombs_around += 1;
          numbered_tile_set[21].bombs_around += 1;
        }
        "e2" => {
          numbered_tile_set[15].bombs_around += 1;
          numbered_tile_set[16].bombs_around += 1;
          numbered_tile_set[17].bombs_around += 1;
          numbered_tile_set[20].bombs_around += 1;
          numbered_tile_set[22].bombs_around += 1;
        }
        "e3" => {
          numbered_tile_set[16].bombs_around += 1;
          numbered_tile_set[17].bombs_around += 1;
          numbered_tile_set[18].bombs_around += 1;
          numbered_tile_set[21].bombs_around += 1;
          numbered_tile_set[23].bombs_around += 1;
        }
        "e4" => {
          numbered_tile_set[17].bombs_around += 1;
          numbered_tile_set[18].bombs_around += 1;
          numbered_tile_set[19].bombs_around += 1;
          numbered_tile_set[22].bombs_around += 1;
          numbered_tile_set[24].bombs_around += 1;
        }
        "e5" => {
          numbered_tile_set[18].bombs_around += 1;
          numbered_tile_set[19].bombs_around += 1;
          numbered_tile_set[23].bombs_around += 1;
        }
        _ => panic!("invalid tile input"),
      }
    }
  }

  numbered_tile_set
}
