pub fn p1(s: String) {
    let mut score = 0;

    for line in s.split("\n").map(|x| x.split_once(": ").unwrap().1.split_once(" | ").unwrap()) {
        let winning = line.0.split(" ").filter(|x| x.len() >= 1).map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let hand = line.1.split(" ").filter(|x| x.len() >= 1).map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let mut count: i32 = 0;

        for x in hand {
            if winning.contains(&x) {
                count += 1;
            }
        }

        if count >= 1 {
            score += 2i32.pow((count - 1) as u32);
        }
    }

    println!("Score : {score}");
}

pub fn p2(s: String) {
    let mut cards: Vec<(i32, i32)> = vec![];
    //        no of matches, no of copies
    for line in s.split("\n").map(|x| x.split_once(": ").unwrap().1.split_once(" | ").unwrap()) {
        let winning = line.0.split(" ").filter(|x| x.len() >= 1).map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let hand = line.1.split(" ").filter(|x| x.len() >= 1).map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let count = hand.iter().filter(|x| winning.contains(x)).count() as i32;

        cards.push((count, 1));
    }

    for index in 0..cards.len() {
        let card = cards[index];
        
        for i in (index + 1usize)..(index + 1usize + card.0 as usize) {
            cards[i].1 += card.1;
        }
    }

    let mut count = 0;
    for x in cards {
        count += x.1;
    }
    println!("{count}");
}
