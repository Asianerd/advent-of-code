pub fn p1(input_data: String) {
    let mut data: Vec<i32> = vec![];
    for x in input_data.to_string().split("\n") {
        data.push(x.parse::<i32>().unwrap());
    }

    let mut increases = 0;
    let mut prev = -999999999;
    for x in data {
        if prev == -999999999 {
            prev = x;
            continue;
        }
        if x > prev {
            increases += 1;
        }

        prev = x;
    }

    println!("Increases : {increases}");
}

pub fn p2(input_data: String) {
    let mut data: Vec<i32> = vec![];
    for x in input_data.to_string().split("\n") {
        data.push(x.parse::<i32>().unwrap());
    }

    let mut sums: Vec<i32> = vec![];
    for i in 0..=(data.len() - 3) {
        sums.push(data[i] + data[i + 1] + data[i + 2]);
    }

    let mut increase = 0;
    let mut prev = -9999999;
    for x in sums {
        if prev == -9999999 {
            prev = x;
            continue;
        }

        if x > prev {
            increase += 1;
        }
        prev = x;
    }

    println!("Increases : {increase}");
}
