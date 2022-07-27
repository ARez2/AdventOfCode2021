use std::fs;


fn main() {
    let mut contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    contents = contents.replace("\r", "");
        
    let mut cont = contents.split("\n");
    
    let number_length = cont.nth(0).unwrap().len();
    println!("Number length: {}", number_length);
    println!();
    //return ;
    let mut digits = Vec::<Vec::<u8>>::new();
    //digits.resize(cont.clone().into_iter().count() + 1, vec![]);
    for line in contents.split("\n") {
        let mut digit = Vec::<u8>::new();
        for c in line.chars() {
            digit.push(c.to_digit(10).unwrap() as u8);
        };
        digits.push(digit);
    };

    let oxygen_generator_rating = get_rating(&digits.clone(), true);
    let co2_scrubber_rating = get_rating(&digits.clone(), false);
    
    let o2_number = &oxygen_generator_rating;
    let mut o2_string = String::new();
    for o2 in o2_number {
        o2_string = join_strings(&o2_string, &format!("{}", o2));
    };
    let co2_number = &co2_scrubber_rating;
    let mut co2_string = String::new();
    for co2 in co2_number {
        co2_string = join_strings(&co2_string, &format!("{}", co2));
    };

    let o2_int = isize::from_str_radix(&o2_string, 2).unwrap();
    let co2_int = isize::from_str_radix(&co2_string, 2).unwrap();
    let life_support = o2_int * co2_int;
    println!("Life Support: {}", life_support);
}


fn get_bits_for_position(position: usize, digits: &Vec<Vec<u8>>) -> (u8, u8) {
    let mut one_count = 0;
    let mut zero_count = 0;
    for digit in digits {
        let num = digit[position];
        if num == 1 {
            one_count += 1;
        } else if num == 0 {
            zero_count += 1;
        }
    };

    let mut max_val = 1;
    let mut min_val = 0;

    if zero_count > one_count {
        max_val = 0;
        min_val = 1;
    };
    if zero_count == one_count {
        max_val = 3;
        min_val = 3;
    }
    println!("{}, {}", one_count, zero_count);
    (max_val, min_val)
}


fn join_strings(str1: &String, str2: &String) -> String {
    format!("{}{}", str1.to_owned(), str2.to_owned())
}


fn get_rating(start_rating: &Vec<Vec<u8>>, get_max: bool) -> Vec<u8> {
    let mut cur_idx = 0;
    let mut length = start_rating.len();
    let mut rating = start_rating.clone();
    while length > 1 {
        let res = get_bits_for_position(cur_idx as usize, &rating);
        let mut target_rating = res.0;
        if !get_max {
            target_rating = res.1;
        }
        if target_rating == 3 {
            if !get_max {
                target_rating = 0;
            } else {
                target_rating = 1;
            }
        };
            
        let mut c = rating.clone();
        let mut digit_idx: i32 = length as i32 - 1;
        for digit in rating.iter().rev() {
            if digit[cur_idx] != target_rating {
                c.remove(digit_idx as usize);
                length -= 1;
            };
            digit_idx -= 1;
            // if digit[i] != co2_rating && co2_scrubber_ratings.len() > 1 {
                //     co2_scrubber_ratings.remove(i);
                // };
        };
        rating = c;
        cur_idx += 1;
    };
    rating.clone()[0].clone()
}