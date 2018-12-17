use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut input_string = String::new();
    file.read_to_string(&mut input_string).expect("Unable to read the file");

    let chars : [char; 26] = ['a','b','c', 'd', 'e','f','g','h','i','j','k','l', 'm','n','o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut backup = input_string;
    backup.remove(0);
    for c in chars.iter() {
        let mut input = backup.clone();
        input.retain(|x| x!= *c);
        input.retain(|x| x!= c.to_string().to_uppercase().chars().next().unwrap());
        let mut finished = false;
        let mut starting_point = 0;
        while !finished {
            let lenth = input.chars().count();
            let mut has_stopped_at_end = false;    
            for i in starting_point..lenth {
                let this_el = input.chars().nth(i).unwrap();
                let mut next_el = '1';
                if i+1 < lenth {
                    next_el = input.chars().nth(i+1).unwrap();
                }
                let is_a_match = can_destroy(this_el, next_el);
                if is_a_match {
                    //remove i and i+1
                    input.remove(i);
                    input.remove(i);
                    if i > 0 {
                        //continue at index where we removed char - 1
                        //we already checked up to i-1. 
                        //removing i may cause i-1 to destroy i+1 so we can carry on from there
                        starting_point = i-1;
                    }
                    break;
                }
                if i == lenth-1 {
                    finished = true;
                }
            }
        }
        println!("{:?} {:?}", c, input.chars().count());

    }
}

fn can_destroy(first: char, second: char) -> bool {
    if first.to_lowercase().to_string() == second.to_lowercase().to_string() {
            if (first.is_lowercase() && second.is_uppercase()) ||
            first.is_uppercase() && second.is_lowercase() {
        return true;
        }
    }
    false
}