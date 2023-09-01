pub fn p2(data: String) {
    let mut safe = 0;

    for x in data.split("\n") {
        let words: Vec<String> = x.to_string().split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
        let mut compromised = false;

        for w in &words {
            let mut count = 0;
            for c in &words {
                if w == c {
                    count += 1;
                } else {
                    if sort_string(w.to_owned()) == sort_string(c.to_owned()) {
                        count += 1;
                    }
                }
            }
            if count >= 2 {
                compromised = true;
            }
        }

        if !compromised {
            safe += 1;
        }
    }
println!("Valid : {safe}");
}

fn sort_string(input_string: String) -> String {
    let mut output = input_string.chars()
        .map(|x| x as u8)
        .collect::<Vec<u8>>();
    output.sort();
    output.into_iter().map(|x| x as char).collect::<String>()
}