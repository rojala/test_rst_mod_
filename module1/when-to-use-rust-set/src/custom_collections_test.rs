
#[cfg(test)]
use crate::custom_collection::MyCollection;
mod tests {
    use super::*;

    #[test]
    fn test_add_and_len() {
        let mut col = MyCollection::new();
        col.add(1);
        col.add(2);
        assert_eq!(col.len(), 2);
    }

    #[test]
    fn test_iter_sum() {
        let mut col = MyCollection::new();
        col.add(10);
        col.add(20);
        let sum: i32 = col.iter().sum();
        assert_eq!(sum, 30);
    }

    #[test]
    fn test_into_iter_owned() {
        let mut col = MyCollection::new();
        col.add("a");
        col.add("b");
        let collected: Vec<_> = col.into_iter().collect();
        assert_eq!(collected, vec!["a", "b"]);
    }

    #[test]
    fn test_into_iter_borrowed() {
        let mut col = MyCollection::new();
        col.add(1);
        col.add(2);
        let doubled: Vec<_> = (&col).into_iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4]);
    }
}
