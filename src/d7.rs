use std::collections::HashMap;

pub fn p1(data: String) {
    let mut raw: Vec<String> = vec![];
    for x in data.split("\n") {
        if x.split("->").collect::<Vec<&str>>().len() >= 2 {
            raw.push(x.to_string());
        }
    }

    raw = raw.into_iter().map(|x| x.trim_end().to_string()).map(|x| x.trim_start().to_string()).collect::<Vec<String>>();

    let mut children: Vec<String> = vec![];
    let mut parents: Vec<String> = vec![];
    for x in raw {
        let c = x.split(" -> ").collect::<Vec<&str>>()[1].split(", ").collect::<Vec<&str>>().into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
        parents.push(x.split(" ").collect::<Vec<&str>>()[0].to_string());
        for i in c {
            children.push(i);
        }
    }

    println!("{parents:?}");
    println!("{children:?}");

    for p in parents {
        if !(children.contains(&p)) {
            println!("Found : {p}");
        }
    }
}

pub fn p2(data: String) {
    // OOP approach because rust is fast anyways
    #[derive(Clone, Debug)]
    struct Process {
        name: String,
        children: Vec<String>,

        weight: i32,
        
        child_weight: i32,
        weight_fetched: bool,
    }
    impl Process {
        fn fetch_weight(&mut self, collection: &HashMap<String, Process>) {
            if self.weight_fetched {
                return;
            }

            if self.children.len() <= 0 {
                self.child_weight = 0;
                self.weight_fetched = true;
            }

            // only change weight when all children have weight fetched
            for c in &self.children {
                if !collection.get(c).unwrap().weight_fetched {
                    // if the child's weight isnt fetched, exit the loop
                    return;
                }
            }

            self.child_weight = 0;
            for c in &self.children {
                let child_reference = collection.get(c).unwrap();
                self.child_weight += child_reference.weight + child_reference.child_weight;
            }
            self.weight_fetched = true;
        }

        fn calculated_weight(&self) -> i32 {
            self.weight + self.child_weight
        }
    }

    let raw = data.split("\n").into_iter().map(|x| x.trim_end().to_string()).map(|x| x.trim_start().to_string()).collect::<Vec<String>>();

    let mut collection: HashMap<String, Process> = HashMap::new();
    for x in raw {
        let mut c = vec![];
        if x.split(" -> ").collect::<Vec<&str>>().len() >= 2 {
            c = x.split(" -> ").collect::<Vec<&str>>()[1].split(", ").collect::<Vec<&str>>().into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
        }
        let parent_weight = x.split(" ").collect::<Vec<&str>>()[1].trim_matches('(').trim_matches(')').parse::<i32>().unwrap();
        collection.insert(
            x.split(" ").collect::<Vec<&str>>()[0].to_string(),
            Process {
                name: x.split(" ").collect::<Vec<&str>>()[0].to_string(),
                children: c,
                weight: parent_weight,
                child_weight: 0,
                weight_fetched: false
            }
        );
    }

    loop {
        let mut completion_flag = true;
        for x in &collection {
            if !(x.1.weight_fetched) {
                completion_flag = false;
                break;
            }
        }
        if completion_flag {
            // exit the loop once all weights are fetched
            break;
        }

        // this part only runs when all the weights arent fetched
        let copied_hashmap = collection.to_owned();
        for x in &mut collection {
            x.1.fetch_weight(&copied_hashmap);
        }
    }

    //println!("{:?}", collection);
    let mut suspects: Vec<Process> = vec![];
    for x in &collection {
        if x.1.children.len() < 1 {
            continue;
        }
        //println!("{:?} = {:?}", x.1.name, x.1.calculated_weight());
        let p = x.1;
        let mut average: f64 = 0.0;
        for c in &p.children {
            average += collection.get(c).unwrap().calculated_weight() as f64;
        }
        average /= p.children.len() as f64;
        if (average as i32) != collection.get(&p.children[0]).unwrap().calculated_weight() {
            println!("FOUND {:?} : {:?} = {:?}", p.name, p.children, p.calculated_weight());
            suspects.push(p.to_owned());
        }
    }

    // everything beyond this point is bs

    // for x in suspects {
    //     println!("{x:?}");
    // }

    println!("{:?}", collection.get("utnrb").unwrap());
    let p = collection.get("lahahn").unwrap();
    for x in &p.children {
        println!("{:?} : {:?}", collection.get(x).unwrap().name, collection.get(x).unwrap().calculated_weight());
    }
    let p = collection.get("utnrb").unwrap();
    for x in &p.children {
        println!("{:?} : {:?}", collection.get(x).unwrap().name, collection.get(x).unwrap().calculated_weight());
    }

    // for x in ["lqwwuqk", "shypdye", "javnv"] {
    //     println!("{:?}", collection.get(x).unwrap().calculated_weight());
    // }
    //2741
}
