
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // ownership of s1 moves to s2, so s1 can no longer be used
    println!("{}", s1);
}
