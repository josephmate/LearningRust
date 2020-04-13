
fn main() {
    let mut owner = String::from("hello");
    let imut_borrower = &owner;
    // I cannot borrow because there is an immutable borrower still.
    let mut_borrower = &mut owner;

    println!("{} {} {}", owner, imut_borrower, mut_borrower);
}
