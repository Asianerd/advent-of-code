use std::collections::HashMap;

pub fn p1(data: String) {
    let instructions: Vec<String> = data.split("\n").into_iter().map(|x| x.to_string()).collect::<Vec<String>>();

    let mut registers: HashMap<String, i64> = HashMap::new();
    for i in &instructions {
        let instruction = i.split(" ").into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let target_register = instruction[0].clone();

        fetch(&target_register, &mut registers);

        let sign = if instruction[1] == "dec".to_string() { -1 } else { 1 };

        let compare_register = instruction[4].clone();
        let compare_gate = instruction[5].clone();
        let compare_value = instruction[6].clone().parse::<i64>().unwrap();

        let flag = match compare_gate.as_str() {
            ">" => fetch(&compare_register, &mut registers) > compare_value,
            "<" => fetch(&compare_register, &mut registers) < compare_value,
            ">=" => fetch(&compare_register, &mut registers) >= compare_value,
            "<=" => fetch(&compare_register, &mut registers) <= compare_value,
            "==" => fetch(&compare_register, &mut registers) == compare_value,
            "!=" => fetch(&compare_register, &mut registers) != compare_value,
            _ => panic!("unknown gate encountered")
        };

        if flag {
            let new_value = registers.get(&target_register).unwrap() + (instruction[2].parse::<i64>().unwrap() * sign);
            registers.insert(target_register.clone(), new_value);
        }
    }

    let mut result = registers.clone().into_iter().collect::<Vec<(String, i64)>>();
    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for x in result {
        println!("{} : {}", x.0, x.1);
    }
}

fn fetch(name: &String, register: &mut HashMap<String, i64>) -> i64 {
    if !register.contains_key(name) {
        register.insert(name.clone(), 0);
    }

    register.get(name).unwrap().to_owned()
}


pub fn p2(data: String) {
    let instructions: Vec<String> = data.split("\n").into_iter().map(|x| x.to_string()).collect::<Vec<String>>();

    let mut highest_value = 0;

    let mut registers: HashMap<String, i64> = HashMap::new();
    for i in &instructions {
        let instruction = i.split(" ").into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let target_register = instruction[0].clone();

        fetch(&target_register, &mut registers);

        let sign = if instruction[1] == "dec".to_string() { -1 } else { 1 };

        let compare_register = instruction[4].clone();
        let compare_gate = instruction[5].clone();
        let compare_value = instruction[6].clone().parse::<i64>().unwrap();

        let flag = match compare_gate.as_str() {
            ">" => fetch(&compare_register, &mut registers) > compare_value,
            "<" => fetch(&compare_register, &mut registers) < compare_value,
            ">=" => fetch(&compare_register, &mut registers) >= compare_value,
            "<=" => fetch(&compare_register, &mut registers) <= compare_value,
            "==" => fetch(&compare_register, &mut registers) == compare_value,
            "!=" => fetch(&compare_register, &mut registers) != compare_value,
            _ => panic!("unknown gate encountered")
        };

        if flag {
            let new_value = registers.get(&target_register).unwrap() + (instruction[2].parse::<i64>().unwrap() * sign);
            if new_value > highest_value {
                highest_value = new_value;
            }
            registers.insert(target_register.clone(), new_value);
        }
    }

    // let mut result = registers.clone().into_iter().collect::<Vec<(String, i64)>>();
    // result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    // for x in result {
    //     println!("{} : {}", x.0, x.1);
    // }

    println!("{highest_value}");
}
