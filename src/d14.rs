use std::collections::HashMap;

pub fn p1(data: String) {
    let mut sum = 0;
    for x in 0..=127 {
        let result = hex_convert(format!("{data}-{x}").to_string()).chars().into_iter().map(|x| hex_to_bin(x)).collect::<String>();
        for c in result.chars() {
            if c == '1' {
                sum += 1;
            }
        }
    }

    println!("Sum : {sum}");
}

pub fn p2(data: String) {
    println!("\n");
    let mut cells: HashMap<(i32, i32), bool> = HashMap::new();
    
    for y in 0..=127 {
        let result = hex_convert(
                format!("{data}-{y}")
            )
            .chars()
            .into_iter()
            .map(|x| hex_to_bin(x))
            .collect::<String>();
        for (x, c) in result.chars().enumerate() {
            if c == '0' {
                continue;
            }
            cells.insert((x as i32, y), false);
        }
    }

    let mut group_count = 0;
    let mut queue: Vec<(i32, i32)> = vec![];
    loop {
        println!("Queue : {:?} items", queue.len());
        //get_field(&queue, Some((5, 5)));
        if queue.len() <= 0 {
            let mut t = (-1, -1);
            let mut flag = false;
            for c in &cells {
                if !c.1 {
                    println!("Added {:?} to the queue", c);
                    t = c.0.to_owned();
                    queue.push(c.0.clone());
                    flag = true;
                }
                if flag {
                    break;
                }
            }
            if flag {
                cells.insert(t, true);
            }
            if !flag {
                // all cells traversed
                break;
            }
            // queue is empty, fill it again
            group_count += 1;
        }

        let parent = queue[0];
        queue.remove(0);
        let neighbours = get_neighbours(parent.clone(), &cells);
        queue.extend(neighbours.clone());
        for x in neighbours {
            cells.insert(x.clone(), true);
        }
    }
    println!("Groups found : {group_count}");

    fn get_neighbours(target: (i32, i32), c: &HashMap<(i32, i32), bool>) -> Vec<(i32, i32)> {
        let mut result: Vec<(i32, i32)> = vec![];

        for i in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            if !c.contains_key(&(target.0 + i.0, target.1 + i.1)) {
                continue;
            }
            if *c.get(&(target.0 + i.0, target.1 + i.1)).unwrap() {
                continue;
            }
            result.push((target.0 + i.0, target.1 + i.1));
        }

        result
    }

    // 5 seconds to fetch everything
    // 8216 cells
}

#[allow(dead_code)]
fn get_field(c: &Vec<(i32, i32)>, size: Option<(i32, i32)>) {
    let mut result = "".to_string();
    for y in 0..=(if size.is_none() { 128 } else { size.unwrap().0 }) {
        for x in 0..=(if size.is_none() { 128 } else { size.unwrap().1 }) {
            if c.contains(&(x, y)) {
                result += "X";
            } else {
                result += ".";
            }
        }
        result += "\n";
    }
    println!("{result}");
}

#[allow(dead_code)]
fn get_field_detailed(c: &HashMap<(i32, i32), bool>, size: Option<(i32, i32)>) {
    let mut result = "".to_string();
    for y in 0..=(if size.is_none() { 128 } else { size.unwrap().0 }) {
        for x in 0..=(if size.is_none() { 128 } else { size.unwrap().1 }) {
            if c.contains_key(&(x, y)) {
                if *c.get(&(x, y)).unwrap() {
                    result += "T";
                } else {
                    result += "X";
                }
            } else {
                result += ".";
            }
        }
        result += "\n";
    }
    println!("{result}");
}

fn hex_to_bin(c: char) -> String {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'a' => "1010",
        'b' => "1011",
        'c' => "1100",
        'd' => "1101",
        'e' => "1110",
        'f' => "1111",
        _ => panic!("INVALID HEX")
    }.to_string()
}


fn hex_convert(data: String) -> String {
    let ending = "17,31,73,47,23"
        .to_string()
        .split(",")
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut lengths = data.chars().into_iter().map(|x| ((x as u8) as i32)).collect::<Vec<i32>>();

    for x in ending {
        lengths.push(x);
    }

    //println!("Lengths : {lengths:?}");
    //let lengths = data.split(",").into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut pointer: i32 = 0;
    let mut skip_size: i32 = 0;
    let mut knot = (0..=255).collect::<Vec<i32>>();

    for _ in 1..=64 {
        for l in &lengths {
            pointer = fix_index(pointer, knot.len() as i32) as i32;

            knot = reverse(pointer, l.to_owned(), &knot);

            pointer += skip_size + l;
            skip_size += 1;
        }
    }

    let mut hex_arrangement: Vec<Vec<i32>> = vec![];
    for x in 0..16 {
        hex_arrangement.push(vec![]);
        for y in 0..16 {
            hex_arrangement[x].push(knot[(x * 16) + y]);
        }
    }

    let mut hex_result: Vec<i32> = vec![];
    for x in hex_arrangement {
        let mut result = 0;
        for y in x {
            result ^= y;
        }

        hex_result.push(result);
    }

    //println!("Hex result : {hex_result:?}");

    //println!("{:?}", hex_result.into_iter().map(|x| format!("{:02x}", x)).collect::<String>());
    hex_result.into_iter().map(|x| format!("{:02x}", x)).collect::<String>()
}

fn reverse(start: i32, amount: i32, collection: &Vec<i32>) -> Vec<i32> {
    // X X X X X X X X X X X X X X
    //     ^         |
    //   start       |
    //     |--amount-|

    // will overshoot, so make the start as the first index
    // 0 1 2 3 4  ~~>  3 4 0 1 2
    // --|   |--       |-----|

    let mut result: Vec<i32> = vec![];
    for x in start..((collection.len() as i32) + start) {
        result.push(collection[fix_index(x, collection.len() as i32)]);
    }

    let mut reverse_result = (&result[0..(amount as usize)]).clone().to_owned();
    reverse_result.reverse();

    for r in 0..(reverse_result.len() as i32) {
        result[r as usize] = reverse_result[r as usize];
    }

    for _ in 0..start {
        result.insert(0, result[result.len() - 1]);
        result.remove(result.len() - 1);
    }

    result
}

fn fix_index(i: i32, l: i32) -> usize {
    if i < 0 {
        return fix_index(l - i, l) as usize;
    }
    if i >= l {
        return fix_index(i - l, l) as usize;
    }
    i as usize
}
