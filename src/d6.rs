pub fn p1(s: String) {
//     let s = "Time:      7  15   30
// Distance:  9  40  200".to_string();
// let s = "Time:      7  4   30
// Distance:  9  3  200".to_string();
    let times: Vec<i32> = s.split("\n").collect::<Vec<&str>>().iter().map(|x| x.clone().split_once(": ").unwrap().1.split(" ").filter(|i| i.len() >= 1).map(|i| i.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>().first().unwrap().clone().iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let distances: Vec<i32> = s.split("\n").collect::<Vec<&str>>().iter().map(|x| x.clone().split_once(": ").unwrap().1.split(" ").filter(|i| i.len() >= 1).map(|i| i.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>().last().unwrap().clone().iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    // shh
    // if it works it works

    let mut pairings: Vec<(i32, i32)> = vec![];

    for i in 0..times.len() {
        pairings.push((times[i], distances[i]));
    }

    let mut counts = 1;

    for (length, record) in pairings {
        let mut count = 0;
        let is_odd = (length % 2) == 1;

        for i in 0..=((length / 2) + (if is_odd { 0 } else { -1 })) {
            let distance = (length - i) * i;
            if distance > record {
                count += 1;
            }
        }

        counts *= count * 2 + (if is_odd {0} else { 1 });
    }

    println!("{counts}");
}

pub fn p2(s: String) {
    let time: i64 = s.split("\n").collect::<Vec<&str>>().first().unwrap().split_once(":").unwrap().1.to_string().replace(" ", "").parse::<i64>().unwrap();
    let distance: i64 = s.split("\n").collect::<Vec<&str>>().last().unwrap().split_once(":").unwrap().1.to_string().replace(" ", "").parse::<i64>().unwrap();

    let mut count = 0;
    let is_odd = (time % 2) == 1;

    for i in 0..=((time / 2) + (if is_odd { 0 } else { -1 })) {
        let d = (time - i) * i;
        if d > distance {
            count += 1;
        }
    }

    println!("{}", count * 2 + (if is_odd {0} else { 1 }));
}
