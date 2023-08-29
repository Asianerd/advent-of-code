pub fn _p1(input: String) {
    let mut fishes: Vec<i32> = vec![];

    for x in input.split(",") {
        fishes.push(x.parse::<i32>().unwrap());
    }

    for i in 0..=79 {
        println!("Day : {i}");
        let copy: Vec<i32> = fishes.to_vec();
        fishes = vec![];

        for x in copy {
            if x == 0 {
                fishes.push(6);
                fishes.push(8);
            } else {
                fishes.push(x - 1);
            }
        }
    }

    println!("Final length : {:?}", fishes.len());
}

// 0, 1, 2, 3, 4, 5, 6, 7, 8
//    1, 2, 3, 4, 5, 6, 7, 8
// 1, 2, 3, 4, 5, 6, 7, 8, 8*
//               +0

pub fn p2(input: String) {
    let mut fishes: Vec<i64> = vec![];

    for _ in 0..=8 {
        fishes.push(0);
    }

    for x in input.split(",") {
        let i = x.parse::<i64>().unwrap();
        fishes[i as usize] += 1;
    }

    for _i in 0..256 {
        for item in &fishes {
            print!(": {item} :");
        }
        println!();

        let ripe = fishes[0];
        fishes.remove(0);
        
        fishes.push(ripe);
        fishes[6] += ripe;
    }

    let mut sum = 0;
    for x in &fishes {
        sum += x;
    }
    println!("Final count = {sum}");
}