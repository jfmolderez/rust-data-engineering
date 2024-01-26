/*
Remember that linkedlist has a higher memory overhead and worse cache locality than Vec or VecDeque
A linked list is a doubly-linked list, which means that each element has a pointer to the next element and the previous element.
A great example of when to use a linked list is when you need to insert or remove elements from the middle of the list.
*/

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::LinkedList;

fn main() {
    let mut fruit = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Randomize the order of the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // convert it back to a linked list
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    // add fruits to the both ends of the list
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i < fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
