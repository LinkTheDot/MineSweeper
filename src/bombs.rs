use crate::inputs;
use crate::tiles::*;
use rand::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum IsBomb {
  Bomb,
  NotBomb,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
  pub tile_type: IsBomb,
  pub bombs_around: i8,
  pub tile_name: String,
  pub display: String,
}

pub fn bomb_assignment() -> Vec<Tile> {
  let mut assigned_bombs: Vec<Tile> = Vec::new();
  let mut bomb_count = inputs();
  let mut vector_bomb_count = 0;

  loop {
    let bomb_range = rand::thread_rng().gen_range(0..24);
    let name = name_assignment(bomb_range);
    let mut push_confirmation = true;

    let bomb = Tile {
      tile_type: IsBomb::Bomb,
      bombs_around: -10,
      tile_name: name,
      display: String::from("▮"),
    };

    for bomb_x in &assigned_bombs {
      if bomb.tile_name == bomb_x.tile_name {
        push_confirmation = false;
        break;
      } else {
        vector_bomb_count += 1;
      }
    }

    if vector_bomb_count == bomb_count {
      break;
    } else if push_confirmation {
      assigned_bombs.push(bomb);
    }

    vector_bomb_count -= vector_bomb_count;
  }

  assigned_bombs
}

pub fn name_assignment(bomb: u8) -> String {
  match bomb {
    0 => String::from("a1"),
    1 => String::from("a2"),
    2 => String::from("a3"),
    3 => String::from("a4"),
    4 => String::from("a5"),

    5 => String::from("b1"),
    6 => String::from("b2"),
    7 => String::from("b3"),
    8 => String::from("b4"),
    9 => String::from("b5"),

    10 => String::from("c1"),
    11 => String::from("c2"),
    12 => String::from("c3"),
    13 => String::from("c4"),
    14 => String::from("c5"),

    15 => String::from("d1"),
    16 => String::from("d2"),
    17 => String::from("d3"),
    18 => String::from("d4"),
    19 => String::from("d5"),

    20 => String::from("e1"),
    21 => String::from("e2"),
    22 => String::from("e3"),
    23 => String::from("e4"),
    24 => String::from("e5"),
    _ => panic!("incorrect tile range"),
  }
}

pub fn name_to_number(name: &String) -> usize {
  match name.as_str().trim() {
    "a1" => 0,
    "a2" => 1,
    "a3" => 2,
    "a4" => 3,
    "a5" => 4,

    "b1" => 5,
    "b2" => 6,
    "b3" => 7,
    "b4" => 8,
    "b5" => 9,

    "c1" => 10,
    "c2" => 11,
    "c3" => 12,
    "c4" => 13,
    "c5" => 14,

    "d1" => 15,
    "d2" => 16,
    "d3" => 17,
    "d4" => 18,
    "d5" => 19,

    "e1" => 20,
    "e2" => 21,
    "e3" => 22,
    "e4" => 23,
    "e5" => 24,
    _ => 100,
  }
}
