use std::collections::HashMap;

pub fn p1(input: String) {
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();

    for (index, s) in input.split("\n").enumerate() {
        let mut data: Vec<(i32, i32)> = vec![];
        let i = s.split(" -> ").collect::<Vec<&str>>();

        i.iter().for_each(|item| {
            data.push((
                item.split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(),
                item.split(",").collect::<Vec<&str>>()[1].parse::<i32>().unwrap()
            ));
        });

        if !((data[0].0 == data[1].0) || (data[0].1 == data[1].1)) {
            continue;
        }

        for y in range(data[0].1, data[1].1) {
            for x in range(data[0].0, data[1].0) {
                if grid.contains_key(&(x, y)) {
                    grid.insert((x, y), grid[&(x, y)] + 1);
                } else {
                    grid.insert((x, y), 1);
                }
            }
        }

        println!("Progress : {index}/{:?}", input.split("\n").count());
    }

    let mut total = 0;
    for x in grid {
        if x.1 >= 2 {
            total += 1;
        }
    }
    println!("{total}");
}

fn range(a: i32, b: i32) -> std::ops::RangeInclusive<i32> {
    if a > b {
        return b..=a;
    }
    a..=b
}

pub fn p2(input: String) {
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();

    for (index, s) in input.split("\n").enumerate() {
        let mut data: Vec<(i32, i32)> = vec![];
        let i = s.split(" -> ").collect::<Vec<&str>>();

        i.iter().for_each(|item| {
            data.push((
                item.split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(),
                item.split(",").collect::<Vec<&str>>()[1].parse::<i32>().unwrap()
            ));
        });

        if (data[0].0 == data[1].0) || (data[0].1 == data[1].1) {
            for y in range(data[0].1, data[1].1) {
                for x in range(data[0].0, data[1].0) {
                    if grid.contains_key(&(x, y)) {
                        grid.insert((x, y), grid[&(x, y)] + 1);
                    } else {
                        grid.insert((x, y), 1);
                    }
                }
            }   
        } else {
            let distance = (data[0].0 - data[1].0).abs();
            let increment = (
                if data[0].0 > data[1].0 { -1 } else { 1 },
                if data[0].1 > data[1].1 { -1 } else { 1 }
            );
            for d in 0..=distance {
                let final_pos = (
                    data[0].0 + (increment.0 * d),
                    data[0].1 + (increment.1 * d)
                );

                if grid.contains_key(&final_pos) {
                    grid.insert(final_pos, grid[&final_pos] + 1);
                } else {
                    grid.insert(final_pos, 1);
                }
            }
        }

        println!("Progress : {index}/{:?}", input.split("\n").count());
    }

    let mut total = 0;
    for x in grid {
        if x.1 >= 2 {
            total += 1;
        }
    }
    println!("{total}");
}
