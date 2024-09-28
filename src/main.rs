use std::time::Instant;

mod ownership;
mod references_mutrefrences;
mod implementations;
mod structs_genrics;
// Define the macro
macro_rules! v {
    ($($x:expr),*) => {
        vec![$($x),*]
    };
}

fn get_first_word(sentence: String) -> String { 
    let mut ans = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}

fn main() {
    let greeting = String::from("hello world\n");
    print!("{}", greeting);
    let char1 = greeting.chars().nth(0);

    match char1 {
        Some(c) => print!("{}", c),
        None => print!("No character"),
    }
    
    let start_time = Instant::now();

    let sentence = String::from("my name is ayush");
    let first_word = get_first_word(sentence.clone());

    print!("First word is {}\n", first_word);
    
    let elapsed_time = start_time.elapsed();
    println!("Time taken: {:?}", elapsed_time);
    let numbers = v![1, 2, 3, 4, 5];
    println!("Numbers vector: {:?}", numbers);

    ownership::demonstrate_ownership();
    references_mutrefrences::showhowrefworks();
    implementations::move_around(implementations::Direction::North);
    
}
