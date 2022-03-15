use crate::bombs::*;
use crate::tiles::*;

pub fn tile_adder(input: u8) -> Vec<Tile> {
  let mut tile_set = tile_builder(input);

  for counter in 0..25 {
    if tile_set[counter].tile_type == IsBomb::Bomb {
      match tile_set[counter].tile_name.as_str() {
        "a1" => {
          tile_set[1].bombs_around += 1;
          tile_set[5].bombs_around += 1;
          tile_set[6].bombs_around += 1;
        }
        "a2" => {
          tile_set[0].bombs_around += 1;
          tile_set[2].bombs_around += 1;
          tile_set[5].bombs_around += 1;
          tile_set[6].bombs_around += 1;
          tile_set[7].bombs_around += 1;
        }
        "a3" => {
          tile_set[1].bombs_around += 1;
          tile_set[3].bombs_around += 1;
          tile_set[6].bombs_around += 1;
          tile_set[7].bombs_around += 1;
          tile_set[8].bombs_around += 1;
        }
        "a4" => {
          tile_set[2].bombs_around += 1;
          tile_set[4].bombs_around += 1;
          tile_set[7].bombs_around += 1;
          tile_set[8].bombs_around += 1;
          tile_set[9].bombs_around += 1;
        }
        "a5" => {
          tile_set[3].bombs_around += 1;
          tile_set[8].bombs_around += 1;
          tile_set[9].bombs_around += 1;
        }
        "b1" => {
          tile_set[0].bombs_around += 1;
          tile_set[1].bombs_around += 1;
          tile_set[6].bombs_around += 1;
          tile_set[10].bombs_around += 1;
          tile_set[11].bombs_around += 1;
        }
        "b2" => {
          tile_set[0].bombs_around += 1;
          tile_set[1].bombs_around += 1;
          tile_set[2].bombs_around += 1;
          tile_set[5].bombs_around += 1;
          tile_set[7].bombs_around += 1;
          tile_set[10].bombs_around += 1;
          tile_set[11].bombs_around += 1;
          tile_set[12].bombs_around += 1;
        }
        "b3" => {
          tile_set[1].bombs_around += 1;
          tile_set[2].bombs_around += 1;
          tile_set[3].bombs_around += 1;
          tile_set[6].bombs_around += 1;
          tile_set[8].bombs_around += 1;
          tile_set[11].bombs_around += 1;
          tile_set[12].bombs_around += 1;
          tile_set[13].bombs_around += 1;
        }
        "b4" => {
          tile_set[2].bombs_around += 1;
          tile_set[3].bombs_around += 1;
          tile_set[4].bombs_around += 1;
          tile_set[7].bombs_around += 1;
          tile_set[9].bombs_around += 1;
          tile_set[12].bombs_around += 1;
          tile_set[13].bombs_around += 1;
          tile_set[14].bombs_around += 1;
        }
        "b5" => {
          tile_set[3].bombs_around += 1;
          tile_set[4].bombs_around += 1;
          tile_set[8].bombs_around += 1;
          tile_set[13].bombs_around += 1;
          tile_set[14].bombs_around += 1;
        }
        "c1" => {
          tile_set[5].bombs_around += 1;
          tile_set[6].bombs_around += 1;
          tile_set[11].bombs_around += 1;
          tile_set[15].bombs_around += 1;
          tile_set[16].bombs_around += 1;
        }
        "c2" => {
          tile_set[5].bombs_around += 1;
          tile_set[6].bombs_around += 1;
          tile_set[7].bombs_around += 1;
          tile_set[10].bombs_around += 1;
          tile_set[12].bombs_around += 1;
          tile_set[15].bombs_around += 1;
          tile_set[16].bombs_around += 1;
          tile_set[17].bombs_around += 1;
        }
        "c3" => {
          tile_set[6].bombs_around += 1;
          tile_set[7].bombs_around += 1;
          tile_set[8].bombs_around += 1;
          tile_set[11].bombs_around += 1;
          tile_set[13].bombs_around += 1;
          tile_set[16].bombs_around += 1;
          tile_set[17].bombs_around += 1;
          tile_set[18].bombs_around += 1;
        }
        "c4" => {
          tile_set[7].bombs_around += 1;
          tile_set[8].bombs_around += 1;
          tile_set[9].bombs_around += 1;
          tile_set[12].bombs_around += 1;
          tile_set[14].bombs_around += 1;
          tile_set[17].bombs_around += 1;
          tile_set[18].bombs_around += 1;
          tile_set[19].bombs_around += 1;
        }
        "c5" => {
          tile_set[8].bombs_around += 1;
          tile_set[9].bombs_around += 1;
          tile_set[13].bombs_around += 1;
          tile_set[18].bombs_around += 1;
          tile_set[19].bombs_around += 1;
        }
        "d1" => {
          tile_set[10].bombs_around += 1;
          tile_set[11].bombs_around += 1;
          tile_set[16].bombs_around += 1;
          tile_set[20].bombs_around += 1;
          tile_set[21].bombs_around += 1;
        }
        "d2" => {
          tile_set[10].bombs_around += 1;
          tile_set[11].bombs_around += 1;
          tile_set[12].bombs_around += 1;
          tile_set[15].bombs_around += 1;
          tile_set[17].bombs_around += 1;
          tile_set[20].bombs_around += 1;
          tile_set[21].bombs_around += 1;
          tile_set[22].bombs_around += 1;
        }
        "d3" => {
          tile_set[11].bombs_around += 1;
          tile_set[12].bombs_around += 1;
          tile_set[13].bombs_around += 1;
          tile_set[16].bombs_around += 1;
          tile_set[18].bombs_around += 1;
          tile_set[21].bombs_around += 1;
          tile_set[22].bombs_around += 1;
          tile_set[23].bombs_around += 1;
        }
        "d4" => {
          tile_set[12].bombs_around += 1;
          tile_set[13].bombs_around += 1;
          tile_set[14].bombs_around += 1;
          tile_set[17].bombs_around += 1;
          tile_set[19].bombs_around += 1;
          tile_set[22].bombs_around += 1;
          tile_set[23].bombs_around += 1;
          tile_set[24].bombs_around += 1;
        }
        "d5" => {
          tile_set[13].bombs_around += 1;
          tile_set[14].bombs_around += 1;
          tile_set[18].bombs_around += 1;
          tile_set[23].bombs_around += 1;
          tile_set[24].bombs_around += 1;
        }
        "e1" => {
          tile_set[15].bombs_around += 1;
          tile_set[16].bombs_around += 1;
          tile_set[21].bombs_around += 1;
        }
        "e2" => {
          tile_set[15].bombs_around += 1;
          tile_set[16].bombs_around += 1;
          tile_set[17].bombs_around += 1;
          tile_set[20].bombs_around += 1;
          tile_set[22].bombs_around += 1;
        }
        "e3" => {
          tile_set[16].bombs_around += 1;
          tile_set[17].bombs_around += 1;
          tile_set[18].bombs_around += 1;
          tile_set[21].bombs_around += 1;
          tile_set[23].bombs_around += 1;
        }
        "e4" => {
          tile_set[17].bombs_around += 1;
          tile_set[18].bombs_around += 1;
          tile_set[19].bombs_around += 1;
          tile_set[22].bombs_around += 1;
          tile_set[24].bombs_around += 1;
        }
        "e5" => {
          tile_set[18].bombs_around += 1;
          tile_set[19].bombs_around += 1;
          tile_set[23].bombs_around += 1;
        }
        _ => panic!("invalid tile input"),
      }
    }
  }

  tile_set
}
