/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the VecDeque, and "Fig" and "Cherry" to the back.
Finally, it prints out the final fruit salad.

VecDeque is a double-ended queue, which is a data structure that allows you to push and pop from both ends.
*/

use rand::seq::SliceRandom; // for randomizing the order of the fruit
use rand::thread_rng;
use std::collections::VecDeque;

fn main() {
    let mut fruit = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to a VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();
    //let mut fruit = fruit.into_iter().collect::<VecDeque<_>>();

    // push some more fruit to the front and back
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

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