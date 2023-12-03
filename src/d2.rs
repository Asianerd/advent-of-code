pub fn p1(s: String) {
//     let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let mut final_result: Vec<bool> = vec![];

    for lines in s.split("\n").map(|x| x
            .split(": ").last().unwrap()
            .split("; ")
            .collect::<Vec<&str>>()
        ).collect::<Vec<Vec<&str>>>()
        {
        let mut result: Vec<i32> = vec![0, 0, 0];

        for segment in lines.into_iter().map(|x| x.split(", ").collect::<Vec<&str>>()) {

            for item in segment {
                let (number_raw, colour) = item.split_once(" ").unwrap();
                let number = number_raw.parse::<i32>().unwrap();

                let index = match colour.to_string().chars().collect::<Vec<char>>().first().unwrap() {
                    'r' => 0usize,
                    'g' => 1usize,
                    'b' => 2usize,
                    _ => 0usize
                };

                if number > result[index] {
                    result[index] = number;
                }
            }
        }

        final_result.push(
            (result[0] <= 12) &
            (result[1] <= 13) &
            (result[2] <= 14)
        );
    }

    let mut sum = 0;

    for (index, x) in final_result.iter().enumerate() {
        if *x {
            sum += index as i32 + 1;
        }
    }

    println!("{sum}");
    // 2496 X
    // 2545
}

pub fn p2(s: String) {
    //     let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    
        let mut final_result: Vec<i32> = vec![];
    
        for lines in s.split("\n").map(|x| x
            .split(": ").last().unwrap()
            .split("; ")
            .collect::<Vec<&str>>()
        ).collect::<Vec<Vec<&str>>>() {
            let mut result: Vec<i32> = vec![0, 0, 0];

            for segment in lines.into_iter().map(|x| x.split(", ").collect::<Vec<&str>>()) {
                for item in segment {
                    let (number_raw, colour) = item.split_once(" ").unwrap();
                    let number = number_raw.parse::<i32>().unwrap();
    
                    let index = match colour.to_string().chars().collect::<Vec<char>>().first().unwrap() {
                        'r' => 0usize,
                        'g' => 1usize,
                        'b' => 2usize,
                        _ => 0usize
                    };
    
                    if number > result[index] {
                        result[index] = number;
                    }
                }
            }
    
            final_result.push(
                result[0] * result[1] * result[2]
            );
        }
    
        let mut sum = 0;
    
        for x in final_result {
            sum += x;
        }
    
        println!("{sum}");
        // 78111
    }
