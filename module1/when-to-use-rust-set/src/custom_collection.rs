use std::slice::Iter;

#[derive(Debug)]
pub struct MyCollection<T> {
    items: Vec<T>,
}

impl<T> MyCollection<T> {
    pub fn new() -> Self {
        MyCollection { items: Vec::new() }
    }

    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.items.iter()
    }
}

impl<T> IntoIterator for MyCollection<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a MyCollection<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

pub fn run() {
    let mut col = MyCollection::new();
    col.add(10);
    col.add(20);
    col.add(30);

    println!("Collection length: {}", col.len());
    println!("Items in collection:");
    for item in col.iter() {
        println!("{}", item);
    }

    let sum: i32 = col.iter().sum();
    println!("Sum of items: {}", sum);

    let doubled: Vec<_> = (&col).into_iter().map(|x| x * 2).collect();
    println!("Doubled items: {:?}", doubled);

    let collected: Vec<_> = col.into_iter().collect();
    println!("Collected items into a vector: {:?}", collected);
}