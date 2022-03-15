use crate::bombs::*;
use std::io;

mod bombs;
mod tile_numbers;
mod tiles;

fn main() {
  let input = inputs();
  let mut tile_set = tile_numbers::tile_adder(input);
  let mut win_counter: u8 = 25 - input;
  let mut game_over = false;

  println!("");
  println!("use ! after an input to mark a tile as a bomb");
  println!("do the same to unmark a tile");
  println!("example - a1!");

  loop {
    println!("----------------------------------------");
    println!("----------------------------------------");
    println!("----------------------------------------");

    if game_over {
      for count in 0..25 {
        if tile_set[count].tile_type == IsBomb::Bomb {
          tile_set[count].display = String::from("ó");
        } else {
          tile_set[count].display = tile_set[count].bombs_around.to_string();
        }
      }
    }

    println!(
      "
x 1 2 3 4 5
A|{a1}|{a2}|{a3}|{a4}|{a5}
B|{b1}|{b2}|{b3}|{b4}|{b5}
C|{c1}|{c2}|{c3}|{c4}|{c5}
D|{d1}|{d2}|{d3}|{d4}|{d5}
E|{e1}|{e2}|{e3}|{e4}|{e5}",
      a1 = tile_set[0].display,
      a2 = tile_set[1].display,
      a3 = tile_set[2].display,
      a4 = tile_set[3].display,
      a5 = tile_set[4].display,
      b1 = tile_set[5].display,
      b2 = tile_set[6].display,
      b3 = tile_set[7].display,
      b4 = tile_set[8].display,
      b5 = tile_set[9].display,
      c1 = tile_set[10].display,
      c2 = tile_set[11].display,
      c3 = tile_set[12].display,
      c4 = tile_set[13].display,
      c5 = tile_set[14].display,
      d1 = tile_set[15].display,
      d2 = tile_set[16].display,
      d3 = tile_set[17].display,
      d4 = tile_set[18].display,
      d5 = tile_set[19].display,
      e1 = tile_set[20].display,
      e2 = tile_set[21].display,
      e3 = tile_set[22].display,
      e4 = tile_set[23].display,
      e5 = tile_set[24].display,
    );

    if game_over {
      println!("Game Over");
      break;
    }

    let mut chosen_tile = String::new();

    println!("choose a tile");
    io::stdin().read_line(&mut chosen_tile).unwrap();

    if chosen_tile.trim() == String::from("board") {
      game_over = true;
      continue;
    }

    let mut marker = false;

    if chosen_tile.len() == 4 {
      if &chosen_tile[2..3] == "!" {
        marker = true
      }
    }

    let chosen_tile = if chosen_tile.len() >= 3 {
      bombs::name_to_number(&chosen_tile[0..2].to_string())
    } else {
      100
    };

    if chosen_tile < 100 {
      if tile_set[chosen_tile].display.trim() == String::from("!") {
        if marker {
          tile_set[chosen_tile].display = String::from("▮");
          println!("marked tile reverted");
          continue;
        }

        println!("chose marked tile");
        continue;
      }
    }

    if marker {
      tile_set[chosen_tile].display = String::from("!");
    } else if chosen_tile < 100 {
      if tile_set[chosen_tile].tile_type == IsBomb::Bomb {
        game_over = true;
        continue;
      } else {
        tile_set[chosen_tile].display = tile_set[chosen_tile].bombs_around.to_string();
        win_counter -= 1;
      }
    } else {
      println!("incorrect input");
    }

    if win_counter == 0 {
      println!("");
      println!("board cleared!");
      game_over = true;
    }
  }
}

fn inputs() -> u8 {
  let mut bomb_count = String::new();

  println!("Type desired bomb count.");
  io::stdin()
    .read_line(&mut bomb_count)
    .expect("Line Failure");

  let bomb_count: u8 = match bomb_count.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      panic!("incorrect input")
    }
  };

  if bomb_count >= 24 {
    panic!("too many bombs")
  } else {
    bomb_count
  }
}
