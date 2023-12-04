fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    let max_element = numbers.iter().max();

    match max_element {
        Some(m) => println!("Max element is {}", m),
        None => println!("Vec is empty"),
    }
}
