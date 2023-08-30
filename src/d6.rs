pub fn p1(data: String) {
    let mut memory: Vec<i32> = vec![];
    for x in data.split('\t') {
        memory.push(x.parse::<i32>().unwrap());
    }

    let mut count = 0;

    let mut states: Vec<Vec<i32>> = vec![];
    loop {
        if states.contains(&memory) {
            break;
        }
        states.push(memory.to_owned());
        count += 1;

        redistribute(&mut memory);
        println!("{memory:?}");
    }

    println!("Iterations : {count:?}");
}

fn redistribute(mem: &mut Vec<i32>) {
    let mut target_index = 0;

    let mut temp = mem.to_owned();
    temp.sort();
    let target_highest = temp[temp.len() - 1];

    for x in 0..(mem.len()) {
        if &mem[x] == &target_highest {
            target_index = x;
            break;
        }
    }

    let mut distribution_amount = mem[target_index];
    mem[target_index] = 0;
    let mut cursor: usize = target_index.to_owned();

    loop {
        if distribution_amount <= 0 {
            break;
        }
        distribution_amount -= 1;

        cursor += 1;
        if cursor >= mem.len() {
            cursor = 0;
        }

        mem[cursor] += 1;
    }
}

pub fn p2(data: String) {
    let mut memory: Vec<i32> = vec![];
    for x in data.split('\t') {
        memory.push(x.parse::<i32>().unwrap());
    }

    let mut count = 0;

    let mut states: Vec<Vec<i32>> = vec![];
    loop {
        if states.contains(&memory) {
            break;
        }
        states.push(memory.to_owned());
        // count += 1;

        redistribute(&mut memory);
        println!("{memory:?}");
    }
    println!("Half-lap completed");
    states = vec![];
    println!("Searching completed");
    loop {
        if states.contains(&memory) {
            break;
        }
        states.push(memory.to_owned());
        count += 1;

        redistribute(&mut memory);
        println!("{memory:?}");
    }
    println!("Full-lap completed");

    println!("Iterations : {count:?}");
}
