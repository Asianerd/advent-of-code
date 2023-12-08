use std::collections::HashMap;

pub fn p1(s: String) {
    let s = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    let s = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    let steps = s.split_once("\n\n").unwrap().0.chars().map(|c| if c == 'R' { 1usize } else { 0usize }).collect::<Vec<usize>>();

    let mut route: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in s.split_once("\n\n").unwrap().1.split("\n").map(|i| i.split_once(" = ").unwrap()).map(|i| (
        i.0, i.1.strip_prefix("(").unwrap().strip_suffix(")").unwrap().split(", ").collect::<Vec<&str>>()
        //     i.1.split_once(", ").unwrap().0.strip_prefix("(").unwrap(),
        //     i.1.split_once(", ").unwrap().1.strip_suffix(")").unwrap()
        // )
    )) {
        route.insert(line.0, line.1);
    }

    // println!("{route:?}");

    let mut location = "AAA";
    let mut index = 0usize;
    let mut count = 1;
    loop {
        location = route.get(&location).unwrap()[steps[index]];
        println!("L : {location}");

        index += 1usize;
        if index >= steps.len() {
            index = 0usize;
        }

        if location == "ZZZ" {
            break;
        }
        count += 1;
    }
    println!("{count}");

    // 16896 too low
    // 16897 correct answer 💀
}

pub fn p2(s: String) {
    let _s = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    let _s = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    let steps = s.split_once("\n\n").unwrap().0.chars().map(|c| if c == 'R' { 1usize } else { 0usize }).collect::<Vec<usize>>();

    let mut route: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut path_type: HashMap<&str, (i32, i32)> = HashMap::new();
    // A -> 0, Z -> 1, ? -> -1

    for line in s.split_once("\n\n").unwrap().1.split("\n").map(|i| i.split_once(" = ").unwrap()).map(|i| (
        i.0, i.1.strip_prefix("(").unwrap().strip_suffix(")").unwrap().split(", ").collect::<Vec<&str>>()
    )) {
        route.insert(line.0, line.1.clone());
        path_type.insert(line.0, (
            if line.1.clone()[0].ends_with("A") { 0 } else {
                if line.1.clone()[0].ends_with("Z") { 1 } else { -1 }
            },
            if line.1.clone()[1].ends_with("A") { 0 } else {
                if line.1.clone()[1].ends_with("Z") { 1 } else { -1 }
            }
        ));
    }

    println!("{route:?}");

    let mut locations: Vec<&str> = route
    .keys()
    .clone()
    .filter(|x| x
        .clone()
        .ends_with("A")
    )
    .map(|x| *x)
    .collect::<Vec<&str>>();

    println!("{locations:?}");

    let mut index = 0;
    let mut count: u64 = 0;
    loop {
        if count % 100000 == 0 {
            println!("{count} {locations:?}");
        }

        let direction = steps[index];

        let mut flag = true;

        for i in 0..locations.len() {
            let l = locations[i].clone();
            locations[i] = route[l][direction];

            if !locations[i].ends_with("Z") {
                flag = false;
            }
        }

        count += 1;

        if flag {
            // all locations end with Z
            break;
        }

        index += 1;
        if index >= steps.len() {
            index = 0;
        }
    }

    println!("Count : {count}");

    // 16563603485021 💀💀💀💀

}


