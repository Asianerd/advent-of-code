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

    let range = lines[0].len() - 1;
    let mut o2_lines: Vec<Vec<i32>> = lines.clone();
    for i in 0..=range {
        if o2_lines.len() == 1 {
            break;
        }

        let most_common: Vec<i32> = fetch_most_common(o2_lines.clone()).to_owned();

        let mut final_lines: Vec<Vec<i32>> = vec![];
        for line in &o2_lines {
            if line[i] == most_common[i] {
                final_lines.push(line.clone());
            }
        }
        o2_lines = final_lines.clone();
    }

    let mut co2_lines: Vec<Vec<i32>> = lines.clone();
    for i in 0..=range {
        if co2_lines.len() == 1 {
            break;
        }

        let least_common: Vec<i32> = fetch_least_common(co2_lines.clone()).to_owned();

        let mut final_lines: Vec<Vec<i32>> = vec![];
        for line in &co2_lines {
            if line[i] == least_common[i] {
                final_lines.push(line.clone());
            }
        }
        co2_lines = final_lines.clone();
    }


    let mut oxygen_generator_rating = "".to_string();
    for x in &o2_lines[0] {
        oxygen_generator_rating += x.to_string().as_str();
    }

    let mut co2_scrubber_rating = "".to_string();
    for x in &co2_lines[0] {
        co2_scrubber_rating += x.to_string().as_str();
    }


    println!("Oxygen generator rating : {}", i32::from_str_radix(oxygen_generator_rating.as_str(), 2).ok().unwrap());
    println!("Co2 scrubber rating : {}", i32::from_str_radix(co2_scrubber_rating.as_str(), 2).ok().unwrap());
    println!("Life support rating : {}", i32::from_str_radix(co2_scrubber_rating.as_str(), 2).ok().unwrap() * i32::from_str_radix(oxygen_generator_rating.as_str(), 2).ok().unwrap());
}

fn fetch_most_common(lines: Vec<Vec<i32>>) -> Vec<i32> {
    let mut most_common: Vec<i32> = vec![];
    let range = lines[0].len() - 1;
    for i in 0..=range {
        let mut x = 0;
        for l in &lines {
            x += match l[i] {
                0 => -1,
                1 => 1,
                _ => 0
            };
        }
        most_common.push(if x >= 0 { 1 } else { 0 });
    }

    return most_common;
}

fn fetch_least_common(lines: Vec<Vec<i32>>) -> Vec<i32> {
    let mut least_common: Vec<i32> = vec![];
    let range = lines[0].len() - 1;
    for i in 0..=range {
        let mut x = 0;
        for l in &lines {
            x += match l[i] {
                0 => -1,
                1 => 1,
                _ => 0
            };
        }
        least_common.push(if x < 0 { 1 } else { 0 });
    }

    return least_common;
}
