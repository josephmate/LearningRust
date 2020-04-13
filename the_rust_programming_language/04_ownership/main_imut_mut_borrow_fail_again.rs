
fn main() {
    let owner = String::from("hello");
    // I cannot borrow because there is an immutable borrower still.
    let mut_borrower = &mut owner;
    
    // this time it's okay
    println!("{}", mut_borrower);
}
