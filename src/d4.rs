struct Grid {
    content: Vec<Vec<(i32, bool)>>,
    won: bool
}
impl Grid {
    fn mark(&mut self, n: i32) {
        for y in 0..self.content.len() {
            for x in 0..self.content[y].len() {
                if self.content[y][x].0 == n {
                    self.content[y][x].1 = true;
                }
            }
        }
    }

    fn check(&mut self) -> (bool, i32) { // true if found, i32 is the sum of all unmarked numbers
        let mut result = (false, 0);
        let mut grid_found = false;
        for y in 0..self.content.len() {
            let mut found = true;
            for x in 0..self.content[0].len() {
                if !self.content[y][x].1 {
                    found = false;
                }
            }
            if found {
                grid_found = true;
                break;
            }
        }

        for x in 0..self.content[0].len() {
            let mut found = true;
            for y in 0..self.content.len() {
                if !self.content[y][x].1 {
                    found = false;
                }
            }
            if found {
                grid_found = true;
                break;
            }
        }

        if grid_found {
            result.0 = true;
            for row in &self.content {
                for item in row {
                    if !item.1 {
                        result.1 += item.0;
                    }
                }
            }
        }

        if result.0 {
            self.won = true;
        }

        return result;
    }
}

pub fn p1(input_data: String) {
    let mut bingo_queue: Vec<i32> = vec![];
    for x in
        input_data
            .split("\n")
            .collect::<Vec<&str>>()[0]
            .split(",")
            {
        bingo_queue.push(x.parse().unwrap());
    }

    let mut trimmed_data:Vec<String> = vec![];
    for x in input_data.split("\n\n").enumerate() {
        if x.0 == 0 {
            continue;
        }
        trimmed_data.push(x.1.to_string());
    }

    let mut grids: Vec<Grid> = vec![];

    for d in trimmed_data {
        let mut content: Vec<Vec<(i32, bool)>> = vec![];

        for row in d.split("\n") {
            let mut new_row: Vec<(i32, bool)> = vec![];
            for item in row.split(" ") {
                if item == "" {
                    continue;
                }
                new_row.push((item.parse().ok().unwrap(), false));
            }
            content.push(new_row.clone());
        }

        grids.push(Grid {
            content: content.clone(),
            won: false
        });
    }

    let mut found = false;
    for x in bingo_queue {
        for g in 0..grids.len() {
            grids[g].mark(x);

            let result = grids[g].check();

            if result.0 {
                println!("{}", result.1 * x);
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
}

pub fn p2(input_data: String) {
    let mut bingo_queue: Vec<i32> = vec![];
    for x in
        input_data
            .split("\n")
            .collect::<Vec<&str>>()[0]
            .split(",")
            {
        bingo_queue.push(x.parse().unwrap());
    }

    let mut trimmed_data:Vec<String> = vec![];
    for x in input_data.split("\n\n").enumerate() {
        if x.0 == 0 {
            continue;
        }
        trimmed_data.push(x.1.to_string());
    }

    let mut grids: Vec<Grid> = vec![];

    for d in trimmed_data {
        let mut content: Vec<Vec<(i32, bool)>> = vec![];

        for row in d.split("\n") {
            let mut new_row: Vec<(i32, bool)> = vec![];
            for item in row.split(" ") {
                if item == "" {
                    continue;
                }
                new_row.push((item.parse().ok().unwrap(), false));
            }
            content.push(new_row.clone());
        }

        grids.push(Grid {
            content: content.clone(),
            won: false
        });
    }

    let mut final_score = 0;
    for x in bingo_queue {
        for g in 0..grids.len() {
            if grids[g].won {
                continue;
            }

            grids[g].mark(x);

            let result = grids[g].check();

            if result.0 {
                final_score = result.1 * x;
            }
        }
    }
    println!("{}", final_score);
}