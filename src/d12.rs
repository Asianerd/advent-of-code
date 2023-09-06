pub fn p1(data: String) {
    let mut raw_pipes: Vec<Vec<i32>> = vec![];
    for x in &(data.split("\n").map(|x| x.to_string()).collect::<Vec<String>>()) {
        let mut t = x.split("<-> ").collect::<Vec<&str>>()[1].to_string().split(", ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        t.push(x.split(" <->").collect::<Vec<&str>>()[0].to_string().parse::<i32>().unwrap());
        raw_pipes.push(t.to_owned());
    }

    //println!("{raw_pipes:?}");
    // 0 <-> 123
    // 4 <-> 356
    // parent <-> children
    // add children to group where parent is
    // groups = [[0,1,2,3], [7,8,9,10]]
    // 4 <-> 5,6
    // groups = [[0,1,2,3,4,5,6], [7,8,9,10]]
    //                    + + +
    // combine groups that share any values
    // [[0,1,2,3], [3,4,5,6]] -> [[0,1,2,3,4,5,6]]

    let mut groups: Vec<Vec<i32>> = vec![];
    for x in &raw_pipes {
        let mut flag = false;
        for g in &mut groups {
            if contains(&x, g) {
                flag = true;
                append_list(&x, g);
                break;
            }
        }
        if !flag {
            groups.push(eliminate_duplicates(x));
        }
    }

    // 03       <- A
    // ^
    // 34567    <- B
    // nothing found
    // 
    // 03
    //  ^
    // 34567 <- found
    // 
    // 034567
    // {moved 34567 to first list}

    // 03   <- A
    // 
    // 123  <- B    *FOUND
    // 456  <-
    // 
    // 0312 
    // 
    // 456

    loop {
        let mut found = false;

        let mut a = 0usize;
        let mut b = 0usize;
        for g in groups.iter().enumerate() {
            for sub in groups.iter().enumerate() {
                if g.0 == sub.0 { continue; }

                if contains(g.1, sub.1) {
                    a = g.0;
                    b = sub.0;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }

        if found {
            println!("{:?} : {:?}, {:?}", groups.len(), a, b);

            let mut result = groups[a].to_owned();
            append_list(&groups[b], &mut result);

            if b > a {
                groups.remove(b);
                groups.remove(a);
            } else {
                groups.remove(a);
                groups.remove(b);
            }
            groups.push(result);
        } else {
            break;
        }
    }

    for x in groups.iter().enumerate() {
        if x.1.contains(&0) {
            println!("CONTAINS!");
            println!("{:?}. {:?} {:?}", x.0, x.1.len(), x.1);
        }
    }
}

pub fn p2(data: String) {
    //     let data = "0 <-> 2
    // 1 <-> 1
    // 2 <-> 0, 3, 4
    // 3 <-> 2, 4
    // 4 <-> 2, 3, 6
    // 5 <-> 6
    // 6 <-> 4, 5".to_string();
        let mut raw_pipes: Vec<Vec<i32>> = vec![];
        for x in &(data.split("\n").map(|x| x.to_string()).collect::<Vec<String>>()) {
            let mut t = x.split("<-> ").collect::<Vec<&str>>()[1].to_string().split(", ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            t.push(x.split(" <->").collect::<Vec<&str>>()[0].to_string().parse::<i32>().unwrap());
            raw_pipes.push(t.to_owned());
        }
    
        //println!("{raw_pipes:?}");
        // 0 <-> 123
        // 4 <-> 356
        // parent <-> children
        // add children to group where parent is
        // groups = [[0,1,2,3], [7,8,9,10]]
        // 4 <-> 5,6
        // groups = [[0,1,2,3,4,5,6], [7,8,9,10]]
        //                    + + +
        // combine groups that share any values
        // [[0,1,2,3], [3,4,5,6]] -> [[0,1,2,3,4,5,6]]
    
        let mut groups: Vec<Vec<i32>> = vec![];
        for x in &raw_pipes {
            let mut flag = false;
            for g in &mut groups {
                if contains(&x, g) {
                    flag = true;
                    append_list(&x, g);
                    break;
                }
            }
            if !flag {
                groups.push(eliminate_duplicates(x));
            }
        }
    
        // 03       <- A
        // ^
        // 34567    <- B
        // nothing found
        // 
        // 03
        //  ^
        // 34567 <- found
        // 
        // 034567
        // {moved 34567 to first list}
    
        // 03   <- A
        // 
        // 123  <- B    *FOUND
        // 456  <-
        // 
        // 0312 
        // 
        // 456
    
        loop {
            let mut found = false;
    
            let mut a = 0usize;
            let mut b = 0usize;
            for g in groups.iter().enumerate() {
                for sub in groups.iter().enumerate() {
                    if g.0 == sub.0 { continue; }
    
                    if contains(g.1, sub.1) {
                        a = g.0;
                        b = sub.0;
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
    
            if found {
                println!("{:?} : {:?}, {:?}", groups.len(), a, b);
    
                let mut result = groups[a].to_owned();
                append_list(&groups[b], &mut result);
    
                if b > a {
                    groups.remove(b);
                    groups.remove(a);
                } else {
                    groups.remove(a);
                    groups.remove(b);
                }
                groups.push(result);
            } else {
                break;
            }
        }

        println!("Number of groups : {}", groups.len());
    }

fn append_list(items: &Vec<i32>, collection: &mut Vec<i32>) {
    for x in items {
        if collection.contains(&x) {
            continue;
        }
        collection.push(x.to_owned());
    }
}

fn eliminate_duplicates(collection: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for x in collection {
        if result.contains(x) {
            continue;
        }
        result.push(x.to_owned());
    }
    result
}

fn contains(items: &Vec<i32>, collection: &Vec<i32>) -> bool {
    for x in items {
        if collection.contains(x) {
            return true;
        }
    }
    false
}
