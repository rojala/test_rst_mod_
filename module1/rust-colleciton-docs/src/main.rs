mod lib_wordcount;

fn main() {
    let _the_message = "Hello, world wild Web world !";
    let _counts = lib_wordcount::word_count(_the_message);
    println!("{} {} {:?}", _the_message, _counts.len(), _counts);
}
