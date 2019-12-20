use std::io;
use std::collections::HashMap;

fn main() {
    let mut num_args = String::new();

    io::stdin().read_line(&mut num_args).unwrap();

    let num_args: usize = num_args.trim().parse().unwrap();

    let mut seats = String::new();
    io::stdin().read_line(&mut seats).unwrap();
    
    let mut min_distance: usize = num_args;
    let mut last_seen = HashMap::new();
    for (i, s) in seats.trim().split_whitespace().enumerate() {
        let language: usize = s.trim().parse().unwrap();

        if !last_seen.contains_key(&language) {
            last_seen.insert(language, i);
            continue;
        }

        let distance: usize = i - last_seen[&language]; 
        
        if distance < min_distance {
            min_distance = distance;
        }
    }
    println!("{}", min_distance);

}
