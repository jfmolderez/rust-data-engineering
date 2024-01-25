/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array . it can grow and shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

use rand::seq::SliceRandom; // for randomizing the order of the fruit
use rand::thread_rng; // for generating random numbers

fn main() {
    let mut fruit = vec![
        "Apple",
        "Orange",
        "Banana",
        "Mango",
        "Pineapple",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Peach",
    ];

    // Scramble (shuffle) the fruit
    fruit.shuffle(&mut thread_rng());

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
