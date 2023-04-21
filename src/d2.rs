pub fn p1(input_data: String) {
    let mut pos = (0, 0);

    for n in input_data.split("\n") {
        let a = match n.split(" ").collect::<Vec<&str>>()[0] {
            "forward" => (1, 0),
            "up" => (0, -1),
            "down" => (0, 1),
            _ => (0, 0)
        };

        //let b = n.split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
        let b = n
                        .split(" ")
                        .collect::<Vec<&str>>()[1]
                        .parse::<i32>()
                        .ok()
                        .unwrap();

        pos.0 += a.0 * b;
        pos.1 += a.1 * b;
    }

    println!("Position : ({}, {})", pos.0, pos.1);
    println!("Score : {}", pos.0 * pos.1);
}

pub fn p2(input_data: String) {
    let mut pos = (0, 0);
    let mut aim = 0;

    for n in input_data.split("\n") {
        let a = match n.split(" ").collect::<Vec<&str>>()[0] {
            "forward" => (1, 0),
            "up" => (0, -1),
            "down" => (0, 1),
            _ => (0, 0)
        };

        //let b = n.split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
        let b = n
                        .split(" ")
                        .collect::<Vec<&str>>()[1]
                        .parse::<i32>()
                        .ok()
                        .unwrap();

        pos.0 += a.0 * b;
        aim += a.1 * b;

        if a == (1, 0) {
            pos.1 += b * aim;
        }
    }

    println!("Position : ({}, {})", pos.0, pos.1);
    println!("Score : {}", pos.0 * pos.1);
}
