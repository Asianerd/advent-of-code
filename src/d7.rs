use std::collections::HashMap;

pub fn p1(data: String) {
    let mut positions: Vec<i32> = vec![];
    for x in data.split(',') {
        positions.push(x.parse::<i32>().unwrap());
    }

    positions.sort();

    let minimum: i32 = positions[0];
    let maximum: i32 = positions[&positions.len() - 1];

    let mut lowest_fuel = 1000000000;

    for x in minimum..=maximum {
        if (find_total_fuel(&positions, &x)) <= lowest_fuel {
            lowest_fuel = find_total_fuel(&positions, &x);
        }
    }

    println!("{:?}", lowest_fuel);
}

fn find_total_fuel(pos: &Vec<i32>, target: &i32) -> i32 {
    let mut total = 0;

    for x in pos {
        total += (x - target).abs();
    }

    total
}

pub fn p2(data: String) {
    let mut memoization_table: HashMap<i32, i32> = HashMap::new();

    let mut positions: Vec<i32> = vec![];
    for x in data.split(',') {
        positions.push(x.parse::<i32>().unwrap());
    }

    positions.sort();

    let minimum: i32 = positions[0];
    let maximum: i32 = positions[&positions.len() - 1];

    let mut lowest_fuel = 1000000000;

    for x in minimum..=maximum {
        let f = find_total_fuel_accumulated(x, &positions, &mut memoization_table);
        if (f) <= lowest_fuel {
            lowest_fuel = f;
        }
        println!("Iteration : {:?}", x);
    }

    println!("{:?}", lowest_fuel);

    // 38.606 with assigning find_total_fuel_accumulated(..) to variable f instead of calling it twice
    // 39.051 with memoization
    // 45.665 without memoization
}

fn find_total_fuel_accumulated(target: i32, pos: &Vec<i32>, table: &mut HashMap<i32, i32>) -> i32 {
    let mut total = 0;

    for x in pos {
        total += get_accumulated_fuel(x, target, table);
    }

    total
}

fn get_accumulated_fuel(start: &i32, end: i32, table: &mut HashMap<i32, i32>) -> i32 {
    let distance = (start - end).abs();

    // if table.contains_key(&distance) {
    //     table.get(&distance);
    // }

    let mut fuel = 0;

    for x in 0..=distance {
        fuel += x;
    }
    table.insert(distance, fuel);
    fuel
}
