
fn main() {
    let s1 = String::from("hello");
    new_owner(s1);

    // ownership of s1 moves to new_owner, so s1 can no longer be used
    println!("{}", s1);
}

fn new_owner(s2: String) {
    println!("new owner owns {} now", s2);
}
