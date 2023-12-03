use std::collections::HashMap;

pub fn p1(s: String) {
    let _s = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".to_string();

    let mut collection: HashMap<i32, Vec<(i32, (i32, i32))>> = HashMap::new();

    for (y, line) in s.split("\n").map(|x| format!(".{x}.")).collect::<Vec<String>>().iter().enumerate() {
        //467..114..
        let mut current_pool: Vec<(i32, (i32, i32))> = vec![];
        let mut current_number = "".to_string();
        let mut starting_x: i32 = -1;
        for (x, c) in line.chars().collect::<Vec<char>>().iter().enumerate() {
            if c.is_alphanumeric() {
                if starting_x == -1 {
                    starting_x = x as i32;
                }
                current_number += &c.to_string();
                continue;
            } else {
                if current_number.len() >= 1 {
                    current_pool.push(
                        (
                            current_number.parse::<i32>().unwrap(),
                            (
                                starting_x,
                                (x as i32) - 1)
                            )
                        );
                    current_number = "".to_string();
                    starting_x = -1;
                }
            }
        }
        if current_pool.len() == 0 {
            continue;
        }
        collection.insert(y as i32, current_pool.clone());
        //println!("{line} {current_pool:?}");
        // for item in current_pool {
        //     collection.insert(y as i32, item.clone());
        // }
    }

    // println!("{collection:?}");

    let mut passes: Vec<i32> = vec![];

    for (y, line) in s.split("\n").map(|x| format!(".{x}.")).collect::<Vec<String>>().iter().enumerate() {
        for (x, c) in line.chars().collect::<Vec<char>>().iter().enumerate() {
            if c == &'.' {
                continue;
            }
            if c.is_alphanumeric() {
                continue;
            }
            //println!("{c} {y} {x}");

            for comparison in (y as i32 - 1)..=(y as i32 + 1) {
                if !collection.contains_key(&comparison) {
                    continue;
                }

                for item in collection.get(&comparison).unwrap() {
                    if matches(item.clone(), x as i32) {
                        passes.push(item.0);
                    }
                }
            }
        }
    }

    println!("{passes:?}");

    let mut sum = 0;

    for x in passes {
        sum += x;
    }

    println!("Sum : {sum}");
    // 525911
    // 521979 too low
    // 423502 too low
}

fn matches(item: (i32, (i32, i32)), at: i32) -> bool {
    // .467..114..
    // .......*...
    // 0123456789
    //      5 7 9
    // 5 <= 7 <= 9
    ((item.1.0 - 1) <= at) && (at <= (item.1.1 + 1))
}

pub fn p2(s: String) {
    let _s = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".to_string();

    let mut collection: HashMap<i32, Vec<(i32, (i32, i32))>> = HashMap::new();

    for (y, line) in s.split("\n").map(|x| format!(".{x}.")).collect::<Vec<String>>().iter().enumerate() {
        //467..114..
        let mut current_pool: Vec<(i32, (i32, i32))> = vec![];
        let mut current_number = "".to_string();
        let mut starting_x: i32 = -1;
        for (x, c) in line.chars().collect::<Vec<char>>().iter().enumerate() {
            if c.is_alphanumeric() {
                if starting_x == -1 {
                    starting_x = x as i32;
                }
                current_number += &c.to_string();
                continue;
            } else {
                if current_number.len() >= 1 {
                    current_pool.push(
                        (
                            current_number.parse::<i32>().unwrap(),
                            (
                                starting_x,
                                (x as i32) - 1)
                            )
                        );
                    current_number = "".to_string();
                    starting_x = -1;
                }
            }
        }
        if current_pool.len() == 0 {
            continue;
        }
        collection.insert(y as i32, current_pool.clone());
        //println!("{line} {current_pool:?}");
        // for item in current_pool {
        //     collection.insert(y as i32, item.clone());
        // }
    }

    // println!("{collection:?}");

    let mut total = 0;

    for (y, line) in s.split("\n").map(|x| format!(".{x}.")).collect::<Vec<String>>().iter().enumerate() {
        for (x, c) in line.chars().collect::<Vec<char>>().iter().enumerate() {
            if c != &'*' {
                continue;
            }

            let mut gears: Vec<i32> = vec![];

            for comparison in (y as i32 - 1)..=(y as i32 + 1) {
                if !collection.contains_key(&comparison) {
                    continue;
                }

                for item in collection.get(&comparison).unwrap() {
                    if matches(item.clone(), x as i32) {
                        gears.push(item.0);
                    }
                }
            }

            if gears.len() != 2 {
                continue;
            }

            println!("{gears:?}");

            total += gears[0] * gears[1];
        }
    }

    println!("Sum : {total}");
}
