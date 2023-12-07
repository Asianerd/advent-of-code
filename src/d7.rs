use std::{collections::HashMap, cmp::Ordering};

pub fn p1(s: String) {
    let _s = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    //                      type, hand,      bid
    let mut collection: Vec<(i32, Vec<i32>, i32)> = vec![];
    for x in s.split("\n") {
        // println!("{x} {:?}", [
        //     "High card",
        //     "One pair",
        //     "Two pair",
        //     "Three one of a kind",
        //     "Full house",
        //     "Four one of a kind",
        //     "Five one of a kind",
        // ][discern_hand(convert_all(x.split_once(" ").unwrap().0.to_string())) as usize]);

        let hand = convert_all(x.split_once(" ").unwrap().0.to_string());

        collection.push((
            discern_hand(hand.clone()),
            hand.clone(),
            x.split_once(" ").unwrap().1.parse::<i32>().unwrap()
        ));
    }

    collection.sort_by(|a, b| compare_hand(a, b));

    // println!("{collection:?}");

    let mut sum = 0;

    for (index, x) in collection.iter().enumerate() {
        sum += x.2 * (index as i32 + 1);
    }

    println!("{sum}");
}

pub fn p2(s: String) {
    let _s = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    let _s = "22222 1
AAKKJ 1
AJJJK 1
AAJKK 1
AAAJK 1
AJJKK 1
JJJJJ 1";
    let _s = "J2345 123
AAAA3 123
33333 200
QQQQQ 200
JJJJJ 123";
    //                      type, hand,      bid
    let mut collection: Vec<(i32, Vec<i32>, i32)> = vec![];
    for x in s.split("\n") {
        let hand = convert_all(x.split_once(" ").unwrap().0.to_string());

        collection.push((
            discern_hand(transform_joker(hand.clone())),
            hand.clone(),
            x.split_once(" ").unwrap().1.parse::<i32>().unwrap()
        ));
    }

    collection.sort_by(|a, b| compare_hand(a, b));

    for x in &collection {
        // println!("{:?}", x.1);

        println!("{:?} {}", x.1,[
            "High card",
            "One pair",
            "Two pair",
            "Three of a kind",
            "Full house",
            "Four of a kind",
            "Five of a kind",
        ][x.0 as usize]);
    }

    // println!("{collection:?}");

    let mut sum = 0;

    for (index, x) in collection.iter().enumerate() {
        sum += x.2 * (index as i32 + 1);
    }

    println!("{sum}");
    // 251536706 not the right answer 4:20
    // 251528448 not the right answer 4:47
    // 251527174 too high

    // 251501017 not the right answer

    // 251421071 correct answer

    // 251397332 too low
    // 251364695 too low
}

fn transform_joker(hand: Vec<i32>) -> Vec<i32> {
    let hash = to_hash(hand.clone());

    // jokers are 10

    if !hash.contains_key(&10) {
        return hand.clone();
    }

    let mut possibilities: Vec<(i32, Vec<i32>, i32)> = vec![];

    for c in 1..=13 {
        let mut h: Vec<i32> = vec![];
        for i in &hand {
            h.push(if i == &10 { c } else { i.clone() });
        }

        possibilities.push((
            discern_hand(h.clone()),
            h.clone(),
            0
        ));
    }

    possibilities.sort_by(|a, b| compare_hand(a, b));

    possibilities.last().unwrap().1.clone()

    // let mut freq_card = 0;
    // let mut freq = 0;
    // let joker_amount = hash.get(&10).unwrap().clone();





    // for x in &hash {
    //     if x.0 == &10 {
    //         continue;
    //     }

    //     if x.1 > &freq {
    //         freq = x.1.clone();
    //         continue;
    //     }
    // }

    // if (joker_amount != 2) && (hash.keys().len() == 3) {
    //     // take highest value
    //     for x in &hash {
    //         if x.0 == &10 {
    //             continue;
    //         }
    
    //         if x.1 == &freq {
    //             if x.0 > &freq_card {
    //                 freq_card = x.0.clone();
    //             }
    //         }
    //     }
    // } else {
    //     // take most frequent
    //     for x in &hash {
    //         if x.0 == &10 {
    //             continue;
    //         }
    
    //         if x.1 == &freq {
    //             freq_card = x.0.clone();
    //         }
    //     }
    // }

    // hand.iter().map(|x| if x == &10 { freq_card } else { x.clone() }).collect::<Vec<i32>>()
}

fn compare_hand(a: &(i32, Vec<i32>, i32), b: &(i32, Vec<i32>, i32)) -> Ordering {
    // true: a > b
    if a.0 > b.0 {
        return Ordering::Greater;
    } else if a.0 < b.0 {
        return Ordering::Less;
    }

    for i in 0..5 {
        if a.1[i] == b.1[i] {
            continue;
        }
        if a.1[i] > b.1[i] {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}

fn convert_all(s: String) -> Vec<i32> {
    return s.chars().map(|x| convert_card(x)).collect::<Vec<i32>>()
}

fn convert_card(c: char) -> i32 {
    match c {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        _ => c.to_string().parse::<i32>().unwrap() - 1
    }
}

// 6 Five of a kind, where all five cards have the same label: AAAAA

// 5 Four of a kind, where four cards have the same label and one card has a different label: AA8AA
// 4 Full house, where three cards have the same label, and the remaining two cards share a different label: 23332

// 3 Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
// 2 Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432

// 1 One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4

// 0 High card, where all cards' labels are distinct: 23456

fn discern_hand(cards: Vec<i32>) -> i32 {
    let hashed = to_hash(cards);

    match hashed.keys().len() as i32 {
        5 => 0, // high card
        4 => 1, // one pair
        3 => {
            // 3 oak: TTT98
            // T:3, 9:1, 8:1
            // two pair: 23432
            // 2:2, 3:2, 4:1

            for x in hashed.values() {
                if x == &3 {
                    return 3;
                }
            }

            2
        },
        2 => {
            // 4 oak
            // full house
            for x in hashed.values() {
                if x == &4 {
                    return 5;
                }
            }
            
            4
        },
        1 => 6, // 5 oak
        _ => -1 // invalid
    }
}

fn to_hash(cards: Vec<i32>) -> HashMap<i32, i32> {
    let mut result: HashMap<i32, i32> = HashMap::new();
    for x in cards {
        if result.contains_key(&x) {
            result.insert(x, result.get(&x).unwrap() + 1);
            continue;
        }
        result.insert(x, 1);
    }
    result
}
