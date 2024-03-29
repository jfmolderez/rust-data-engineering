use std::collections::HashMap;

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2];
    let result = logic(numbers);
    println!("{:?}", result);
}
