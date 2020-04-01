
fn main() {
    let s1 = String::from("hello");
    borrowing_str(&s1);

    // ownership was borrowed by borrowing_str, so s1 can still be used
    println!("{}", s1);
}

fn borrowing_str(s2: &String) {
    println!("borrowing_str borrowed {}", s2);
}
