use std::collections::HashMap;

pub fn p1(data: String) {
    let mut directional_vectors: HashMap<Direction, (i32, i32)> = HashMap::new();
    directional_vectors.insert(Direction::N, (0, 1));
    directional_vectors.insert(Direction::NE, (1, 1));
    directional_vectors.insert(Direction::SE, (1, 0));
    directional_vectors.insert(Direction::S, (0, -1));
    directional_vectors.insert(Direction::SW, (-1, -1));
    directional_vectors.insert(Direction::NW, (-1, 0));

    let raw_moves = data.split(",").map(|x| match x {
        "n" => Direction::N,
        "nw" => Direction::NW,
        "ne" => Direction::NE,
        "sw" => Direction::SW,
        "se" => Direction::SE,
        "s" => Direction::S,
        _ => Direction::N,
    }).collect::<Vec<Direction>>();

    let mut moves: HashMap<Direction, i32> = HashMap::new();
    for x in [Direction::N, Direction::NW, Direction::NE, Direction::SW, Direction::SE, Direction::S] {
        moves.insert(x, 0);
    }
    for x in &raw_moves {
        moves.insert(x.clone(), moves.get(&x).unwrap() + 1);
    }

    println!("{:?}", moves);

    let mut position = (0, 0);
    for x in &moves {
        position.0 += directional_vectors.get(x.0).unwrap().0 * x.1;
        position.1 += directional_vectors.get(x.0).unwrap().1 * x.1;
    }

    println!("{:?}", position);

    // distance is the largest of x, y or |x-y|
    let diff_diff = (position.0 - position.1).abs();
    println!("X : {}\nY : {}\n|x-y| : {diff_diff}", position.0, position.1);
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Direction {
         N,
    NW,     NE,

    SW,     SE,
         S
}

pub fn p2(data: String) {
    let mut directional_vectors: HashMap<Direction, (i32, i32)> = HashMap::new();
    directional_vectors.insert(Direction::N, (0, 1));
    directional_vectors.insert(Direction::NE, (1, 0));
    directional_vectors.insert(Direction::SE, (1, -1));
    directional_vectors.insert(Direction::S, (0, -1));
    directional_vectors.insert(Direction::SW, (-1, 0));
    directional_vectors.insert(Direction::NW, (-1, 1));

    let raw_moves = data.split(",").map(|x| match x {
        "n" => Direction::N,
        "nw" => Direction::NW,
        "ne" => Direction::NE,
        "sw" => Direction::SW,
        "se" => Direction::SE,
        "s" => Direction::S,
        _ => Direction::N,
    }).collect::<Vec<Direction>>();

    let mut highest_distance = 0;

    let mut position = (0, 0);
    for x in raw_moves {
        position.0 += directional_vectors.get(&x).unwrap().0;
        position.1 += directional_vectors.get(&x).unwrap().1;

        let new_distance = distance(position.0, position.1);
        if new_distance > highest_distance {
            highest_distance = new_distance;
        }
    }
    
    println!("Distance : {highest_distance}");
    //6811
}

fn distance(i_x: i32, i_y: i32) -> i32 {
    (i_x.abs() + i_y.abs() + (i_x + i_y).abs()) / 2
}
