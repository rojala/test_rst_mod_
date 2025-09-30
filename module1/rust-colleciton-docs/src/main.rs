mod lib_priorityqueue;
mod lib_wordcount;

fn test_word_count() {
    let text = "Hello world hello";
    let result = lib_wordcount::word_count(text);
    println!("{:?}", result);
    assert_eq!(result.len(), 2);
    assert_eq!(result.get("hello"), Some(&2));
    assert_eq!(result.get("world"), Some(&1));
}

fn test_priority_queue() {
    let mut pq = lib_priorityqueue::PriorityQueue::new();
    pq.push(3);
    pq.push(5);
    pq.push(1);

    println!("Priority Queue length: {}", pq.len());
    println!("Priority Queue contents: {:?}", pq);

    assert_eq!(pq.len(), 3);

    assert_eq!(pq.peek(), Some(&5));
    assert_eq!(pq.pop(), Some(5));
    assert_eq!(pq.pop(), Some(3));
    assert_eq!(pq.pop(), Some(1));
    assert!(pq.is_empty());
}

fn main() {
    test_word_count();
    test_priority_queue();
}
