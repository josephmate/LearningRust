
fn main() {
    let mut owner = String::from("hello");
    // I cannot borrow because there is an immutable borrower still.
    {
        let mut_borrower = &mut owner;
        mut_borrower.push_str(" there");
    }

    // mut_borrower is out of scope here
    println!("{}", owner);
}
