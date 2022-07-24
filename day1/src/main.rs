use std::{fs, collections::VecDeque};

struct SlidingWindow {
    values: Vec<i64>,
}

impl SlidingWindow {
    fn sum(&self) -> i64 {
        let mut sum = 0;
        for val in self.values.iter() {
            sum += val;
        };
        sum
    }

    fn is_filled(&self) -> bool {
        self.values.len() == 3
    }
}


fn main() {
    let depths = read_file("input.txt");
    
    let mut sliding_windows = VecDeque::<SlidingWindow>::new();
    let mut last_window_sum = 0;
    // First one doesnt count
    let mut times_increased = -1;
    for depth in depths {
        let new_window = SlidingWindow {
            values: vec![depth],
        };
        let mut pop_first = false;
        for window in sliding_windows.iter_mut() {
            if !window.is_filled() {
                window.values.push(depth);
            }
            if window.is_filled() {
                pop_first = true;
                let sum = window.sum();
                if sum > last_window_sum {
                    times_increased += 1;
                }
                last_window_sum = sum;
            }
        }
        sliding_windows.push_back(new_window);
        if pop_first {
            sliding_windows.pop_front();
        }
    }
    println!("{}", times_increased);
}


fn read_file(filename: &str) -> Vec<i64> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut vec = Vec::<i64>::new();

    for line in contents.split("\n") {
        let trimmed = line.trim();
        let line_res = trimmed.parse::<i64>();
        match line_res {
            Ok(_) => vec.push(line_res.unwrap()),
            Err(err) => {
                println!("An error has occured on line {:?}: {}", trimmed, err);
                return vec;
            },
        }
    }
    vec
}