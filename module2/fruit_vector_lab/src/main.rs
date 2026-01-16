fn main() {
    // Create a mutable vector of fruits.
    let mut fruit_salad = vec!["apple", "banana", "cherry", "dates", "elderberries"];
    println!("Original fruit salad: {:?}", fruit_salad);

    fruit_salad.push("figs");
    fruit_salad.push("mansikka");
    fruit_salad.push("mustikka");

    // Remove the last fruit from the Vector using the pop method.
    fruit_salad.pop();

    // println!("Modified fruit salad: {:?}", fruit_salad);
    // Iterate through the Vector and print each fruit.
    for fruit in &fruit_salad {
        println!("{}", fruit);
    }
}