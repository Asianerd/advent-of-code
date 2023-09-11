use std::collections::HashMap;
use std::thread;

pub fn p1(data: String) {
    let mut walls: HashMap<i32, Wall> = HashMap::new();

//     let data = "0: 3
// 1: 2
// 4: 4
// 6: 4".to_string();
//     println!("{data}");

    for x in data
        .split("\n")
        .map(|i| i
            .split(": ")
            .map(|n| n.to_string())
            .collect::<Vec<String>>()) {

        walls.insert(x[0].parse::<i32>().unwrap(), Wall {
            number: x[0].parse::<i32>().unwrap(),
            depth: x[1].parse::<i32>().unwrap(),
            scanner: 0,
            direction: 1
        });
    }

    let mut severity: i32 = 0;

    let mut max = 0;
    for x in &walls {
        if x.0.clone() >= max {
            max = x.0.clone();
        }
    }

    for iteration in 0..=max {
        if walls.contains_key(&iteration) {
            if walls.get(&iteration).unwrap().scanner == 0 {
                severity += iteration * walls.get(&iteration).unwrap().depth;
            }
        }

        for x in &mut walls {
            x.1.move_scanner();
        }
    }

    println!("severity : {severity}");
}

pub fn p2(data: String) {
    let mut walls: HashMap<i32, Wall> = HashMap::new();

//     let data = "0: 3
// 1: 2
// 4: 4
// 6: 4".to_string();
//     println!("{data}");

    for x in data
        .split("\n")
        .map(|i| i
            .split(": ")
            .map(|n| n.to_string())
            .collect::<Vec<String>>()) {

        walls.insert(x[0].parse::<i32>().unwrap(), Wall {
            number: x[0].parse::<i32>().unwrap(),
            depth: x[1].parse::<i32>().unwrap(),
            scanner: 0,
            direction: 1
        });
    }

    let mut max = 0;
    for x in &walls {
        if x.0.clone() >= max {
            max = x.0.clone();
        }
    }

    for x in 0..5 {
        println!("Spawned thread {x}");

        let c = walls.clone();
        thread::spawn(move || {
            iterate(x.clone() * -50, (x.clone() - 1) * -50, c, max.clone());
        });
    }
}

fn iterate(start: i32, ending: i32, c: HashMap<i32, Wall>, max: i32) {
    let mut walls = c.clone();
    let mut offset: i32 = start.to_owned();
    loop {
        if offset <= ending {
            println!("Thread ended {start} - {ending}");
            return;
        }
        println!("Offset : {offset}");
        for x in &mut walls {
            x.1.scanner = 0;
            x.1.direction = 1;
        }
        let mut caught = false;
        offset -= 1;
        let mut pointer = offset;
        for _ in 0..=(max + offset.abs()) {
            pointer += 1;
            if (pointer >= 0) && walls.contains_key(&pointer) {
                if walls.get(&pointer).unwrap().scanner == 0 {
                    caught = true;
                    break;
                }
            }

            for x in &mut walls {
                x.1.move_scanner();
            }
        }
        if !caught {
            break;
        }
    }
    println!("Delay : {}", (offset + 1).abs());
}

#[derive(Debug, Clone, Copy)]
struct Wall {
    number: i32,
    depth: i32,
    scanner: i32,
    direction: i32
}
impl Wall {
    fn move_scanner(&mut self) {
        self.scanner += self.direction;
        if self.scanner >= self.depth {
            self.scanner = self.depth - 2;
            self.direction = -1;
        }
        if self.scanner <= 0 {
            self.scanner = 0;
            self.direction = 1;
        }
    }

    fn print_debug(&self) {
        print!("{}. ", self.number);
        for x in 0..self.depth {
            if x == self.scanner {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
