pub fn p1(data: String) {
    let target = data.parse::<i32>().unwrap();
    println!("{target:?}");
    //target = 1024;

    let mut x = 0;
    let mut y = 0;
    let mut value = 1;
    let mut i = 0;

    loop {
        let sign = if (i % 2) != 0 { 1 } else { -1 };

        x += i * sign;
        y += i * sign;

        value += i * 2;
        println!("({}, {}) ~> {value}", i * sign, i * sign);

        if value >= target {
            println!("Overshot!");
            if value == target {
                break;
            } else {
                let mut found_flag = false;
                for _ in 0..i {
                    y -= sign;
                    value -= 1;
                    //println!("Decrement y");
                    if value == target {
                        found_flag = true;
                        break;
                    }
                }
                if found_flag {
                    break;
                }
                for _ in 0..i {
                    x -= sign;
                    value -= 1;
                    //println!("Decrement x");
                    if value == target {
                        break;
                    }
                }
                break;
            }
        }

        i += 1;
    }

    println!("Distance : {:?}", x.abs() + y.abs());
}

pub fn p2(data: String) {
    let target = data.parse::<i32>().unwrap();

    let mut tiles: Vec<Tile> = vec![];
    struct Tile {
        x: i32,
        y: i32,
        value: i32
    }
    impl Tile {
        fn fetch_value(&mut self, collection: &Vec<Tile>) {
            for t in collection {
                if (self.x - t.x).abs() <= 1 {
                    if (self.y - t.y).abs() <= 1 {
                        if (self.x == t.x) && (self.y == t.y) {
                            continue;
                        }

                        self.value += t.value;
                    }
                }
            }
        }
    }

    tiles.push(Tile {
        x:0,
        y:0,
        value: 1
    });

    let mut x = 0;
    let mut y = 0;

    let mut step_counter = 0;
    let mut increment_amount = 1;
    loop {
        let sign = if (increment_amount % 2) != 0 { 1 } else { -1 };

        if (step_counter + 1) <= increment_amount {
            x += sign;
        } else {
            y += sign;
        }

        let mut t = Tile {
            x:x,
            y:y,
            value:0
        };
        t.fetch_value(&tiles);
        if t.value >= target {
            println!("FOUND : {:?}", t.value);
            break;
        }
        tiles.push(t);

        println!("Collection size : {:?}", tiles.len());

        step_counter += 1;
        if step_counter >= (increment_amount * 2) {
            step_counter = 0;
            increment_amount += 1;
        }
    }
}
