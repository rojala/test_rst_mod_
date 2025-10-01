// Tests for a custom Rust struct that implements
// Ord and other traits to be usable in BTreeMap and BTreeSet.
use std::collections::BTreeSet;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::fmt;

#[derive(Eq, Clone)]
struct CustomStruct {
    id: u32,
    name: String,
}
impl Ord for CustomStruct {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialOrd for CustomStruct {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for CustomStruct {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Hash for CustomStruct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl fmt::Debug for CustomStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomStruct {{ id: {}, name: {} }}", self.id, self.name)
    }
}

pub fn test_custom_struct() {
    let mut btree_set = BTreeSet::new();
    let mut btree_map = std::collections::BTreeMap::new();

    let item1 = CustomStruct { id: 1, name: "Item1".to_string() };
    let item2 = CustomStruct { id: 2, name: "Item2".to_string() };
    let item3 = CustomStruct { id: 1, name: "Item3".to_string() }; // Same id as item1

    btree_set.insert(item1.clone());
    btree_set.insert(item2.clone());
    btree_set.insert(item3.clone()); // Should not be added, same id as item1

    btree_map.insert(item1.clone(), "First Item");
    btree_map.insert(item2.clone(), "Second Item");
    btree_map.insert(item3.clone(), "Third Item"); // Should overwrite the value for item1

    println!("BTreeSet contents:");
    for item in &btree_set {
        println!("{:?}", item);
    }

    println!("\nBTreeMap contents:");
    for (key, value) in &btree_map {
        println!("{:?} => {}", key, value);
    }

    assert_eq!(btree_set.len(), 2); // Only two unique ids
    assert_eq!(btree_map.len(), 2); // Only two unique ids
    assert_eq!(btree_map.get(&item1).unwrap(), &"Third Item"); // Value should be from item3
}