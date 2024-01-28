/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array . it can grow and shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

use rand::seq::SliceRandom; // for randomizing the order of the fruit
use rand::thread_rng; // for generating random numbers
use std::io;

fn init_fruit() -> Vec<String> {
    let refs = vec![
        "Apple",
        "Orange",
        "Banana",
        "Mango",
        "Pineapple",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Peach",
        "Pear",
        "Strawberry",
        "Kiwi",
        "Grape",
        "Plum",
        "Passion Fruit",
        "Lemon",
        "Lime",
        "Coconut",
        "Watermelon",
        "Cantaloupe",
        "Nut",
        "Peanut",
        "Cashew",
        "Almond",
        "Pistachio",
    ];
    refs.iter()
        .map(|s| s.to_string().to_ascii_lowercase())
        .collect()
}

fn make_salad(num: usize) -> Vec<String> {
    let mut result = Vec::new();
    let fruits = init_fruit();

    for _ in 0..num {
        println!("Please enter a fruit : ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string().to_ascii_lowercase();

        while !fruits.contains(&input) {
            println!("Please enter a valid fruit : ");
            input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input = input.trim().to_string().to_ascii_lowercase();
        }
        result.push(input);
    }
    // Scramble (shuffle) the result
    result.shuffle(&mut thread_rng());
    result
}

fn main() {
    let salad = make_salad(5);

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in salad.iter().enumerate() {
        if i != salad.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
