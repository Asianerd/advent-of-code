pub fn p1(s: String) {
    let mut collection: Vec<i32> = vec![];

    for x in s.split("\n") {
        let numbers: Vec<char> = x
            .to_string()
            .chars()
            .filter(|c| "1234567890"
                .to_string()
                .chars()
                .collect::<Vec<char>>()
                .contains(c)
            )
            .collect::<Vec<char>>();
        collection.push(format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap()).parse::<i32>().unwrap());
    }

    let mut sum = 0;
    for x in collection {
        sum += x;
    }

    println!("{sum}");
}

pub fn p2(s: String) {
    let mut collection: Vec<i32> = vec![];

    for y in s.split("\n") {
        let x = y
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
            .replace("zero", "zero0zero");
        // eightwo -> eigh2 {incorrect}

        // eightwo -> eightwo2two
        //         -> eight8eightwo2two {correct}

        let numbers: Vec<char> = x
            .to_string()
            .chars()
            .filter(|c| "1234567890"
                .to_string()
                .chars()
                .collect::<Vec<char>>()
                .contains(c)
            )
            .collect::<Vec<char>>();

        collection.push(format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap()).parse::<i32>().unwrap());
    }

    let mut sum = 0;
    for x in collection {
        sum += x;
    }

    println!("{sum}");
}
