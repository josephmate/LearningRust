
fn main() {
    let owner = String::from("hello");
    // I cannot borrow because there is an immutable borrower still.
    let mut_borrower = &mut owner;

    println!("{} {}", owner, mut_borrower);
}
