pub fn p1(data: String) {
    let lengths = data.split(",").into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut pointer = 0;
    let mut skip_size = 0;
    let mut knot = (0..=255).collect::<Vec<i32>>();
    for l in &lengths {
        pointer = fix_index(pointer, knot.len() as i32) as i32;

        knot = reverse(pointer, l.to_owned(), &knot);

        pointer += skip_size + l;
        skip_size += 1;
    }

    println!("{:?} * {:?} = {:?}", knot[0], knot[1], knot[0] * knot[1]);
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

pub fn p2(data: String) {
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

    println!("Lengths : {lengths:?}");
    //let lengths = data.split(",").into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut pointer: i32 = 0;
    let mut skip_size: i32 = 0;
    let mut knot = (0..=255).collect::<Vec<i32>>();

    for iteration in 1..=64 {
        println!("Iteration : {iteration}");
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

    println!("Hex result : {hex_result:?}");

    println!("{:?}", hex_result.into_iter().map(|x| format!("{:02x}", x)).collect::<String>());
}
