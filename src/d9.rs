pub fn p1(data: String) {
    let mut parsed_string: String = "".to_string();
    let mut skip_next = false;
    for x in data.chars().to_owned() {
        if skip_next {
            skip_next = false;
            continue;
        }
        if x == '!' {
            skip_next = true;
            // do we omit the ! as well?
            // <!abc> -> <bc> ?
            // or
            // <!abc> -> <!bc> ?
            //continue;
        }

        parsed_string += x.to_string().as_str();
    }

    println!("{parsed_string}");

    let mut result: String = "".to_string();

    let mut in_garbage = false;
    for x in parsed_string.chars().to_owned() {
        if in_garbage {
            if x != '>' {
                continue;
            } else {
                in_garbage = false;
                continue;
            }
        } else {
            if x == '<' {
                in_garbage = true;
                continue;
            } else {
                if x == ',' {
                    continue;
                }
                result += x.to_string().as_str();
            }
        }
    }

    // [X] remove all !
    // [X] remove all garbage
    // [X] remove empty comma
    //  {<garbage>,{}} -> {,{}}
    //                    {,{}} -> {{}}

    let mut sum = 0;
    let mut depth = 0;
    for x in result.chars().to_owned() {
        if x == '{' {
            depth += 1;
        }

        if x == '}' {
            sum += depth;
            depth -= 1;
        }
    }

    println!("{result}");

    println!("{sum}");
}

pub fn p2(data: String) {
    let mut parsed_string: String = "".to_string();
    let mut skip_next = false;
    for x in data.chars().to_owned() {
        if skip_next {
            skip_next = false;
            continue;
        }
        if x == '!' {
            skip_next = true;
            // do we omit the ! as well?
            // <!abc> -> <bc> ?
            // or
            // <!abc> -> <!bc> ?
            continue;
        }

        parsed_string += x.to_string().as_str();
    }

    println!("{parsed_string}");

    let mut count = 0;

    let mut in_garbage = false;
    for x in parsed_string.chars().to_owned() {
        if in_garbage {
            if x != '>' {
                count += 1;
                continue;
            } else {
                in_garbage = false;
                continue;
            }
        } else {
            if x == '<' {
                in_garbage = true;
                continue;
            } else {
                if x == ',' {
                    continue;
                }
            }
        }
    }

    println!("Count : {count}");
}
