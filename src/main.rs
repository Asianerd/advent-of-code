mod challenge_input;
mod d4;

fn main() {
    //println!("{:?}", " 122 3 134 1".to_string().split(" ").filter(|x| x.len() >= 1).collect::<Vec<&str>>());
    d4::p2(challenge_input::input(4));
}
