fn main() {
    // IMMUTABLE BINDING - The default in Rust
    // Once bound, the value cannot be changed
    let x = 5;
    // x = 6;  // ERROR! Cannot assign twice to immutable variable
    println!("Immutable x: {}", x);
    
    // MUTABLE BINDING - Explicitly allow changes with 'mut'
    let mut y = 5;
    y = 6;  // OK! This is allowed with 'mut'
    y += 10;
    println!("Mutable y: {}", y);
    
    // Immutable collections
    let mut _fruits = vec!["apple", "banana"];
    // _fruits.push("orange");  // OK because fruits is mutable
    
    // Shadowing - creating a new immutable binding with the same name
    let y = y + 1;  // Creates a new immutable binding that shadows the mutable one
    // y = 20;  // ERROR! Now y is immutable
    println!("Shadowed y: {}", y);
    
    // Immutable by default - demonstrates security
    let config = "production";
    // config = "development";  // ERROR! Cannot change configuration by accident
    println!("Config: {}", config);
    
    // Mutable when needed
    let mut counter = 0;
    for i in 1..=5 {
        counter += i;
    }
    println!("Counter: {}", counter);
}
