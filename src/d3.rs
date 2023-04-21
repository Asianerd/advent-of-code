pub fn p1(input_data: String) {
    let mut gamma_rate_string = "".to_string();
    let mut epsilon_rate_string = "".to_string();

    let mut count: Vec<i32> = vec![];
    for _ in input_data.split("\n").collect::<Vec<&str>>()[0].chars() {
        count.push(0);
    }
    for line in input_data.split("\n") {
        for (index, c) in line.chars().enumerate() {
            count[index] += match c {
                '0' => -1,
                '1' => 1,
                _ => 0
            };
        }
    }

    for i in count {
        gamma_rate_string += (if i < 0 { 0 } else { 1 }).to_string().as_str();
        epsilon_rate_string += (if i < 0 { 1 } else { 0 }).to_string().as_str();
    }

    let gamma_rate = i32::from_str_radix(&gamma_rate_string, 2).unwrap();
    let epsilon_rate = i32::from_str_radix(&epsilon_rate_string, 2).unwrap();

    println!("Gamma rate : {gamma_rate}");
    println!("Epsilon rate : {epsilon_rate}");
    println!("Power consumption : {}", gamma_rate * epsilon_rate);

}

pub fn p2(input_data: String) {
    let mut lines: Vec<Vec<i32>> = vec![];
    for line in input_data.split("\n") {
        let mut l: Vec<i32> = vec![];
        for c in line.chars() {
            l.push(c.to_string().parse().ok().unwrap());
        }
        lines.push(l);
    }

    let mut most_common: Vec<i32> = vec![];
    let range = lines[0].len();
    for i in 0..=range {
        let mut x = 0;
        for l in &lines {
            x += match l[i] {
                0 => -1,
                1 => 1,
                _ => 0
            };
        }
        most_common.push((if x > 0 { 1 } else { 0 }));
    }
}
