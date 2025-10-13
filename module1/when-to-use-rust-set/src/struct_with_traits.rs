use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/* PartialEq and Eq are required for equality comparisons.
 * Hash is required to compute the hash value for the key.
 * You can also use #[derive(PartialEq, Eq, Hash)] if your
 * struct only contains hashable and comparable fields.
 */

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

// Implement PartialEq and Eq
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.age == other.age
    }
}

impl Eq for Person {}

// Implement Hash
impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.age.hash(state);
    }
}

pub fn run() {
    let mut people_map: HashMap<Person, String> = HashMap::new();

    let person1 = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let person2 = Person {
        name: "Bob".to_string(),
        age: 25,
    };

    people_map.insert(person1, "Engineer".to_string());
    people_map.insert(person2, "Designer".to_string());

    for (person, job) in &people_map {
        println!("{:?} is a {}", person, job);
    }
}