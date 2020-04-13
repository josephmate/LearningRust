use std::io;

enum Colour {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Black,
    White
}

fn main() {
    println!("Enter a color and I will analyze it.");

    let mut input_colour_str = String::new();
    io::stdin().read_line(&mut input_colour_str)
        .expect("Failed to read line.");
    // shadowing
    let input_colour_str = input_colour_str.trim();

    // shadowing
    let input_colour = match input_colour_str {
        "Red" => Some(Colour::Red),
        "red" => Some(Colour::Red),
        "Orange" => Some(Colour::Orange),
        "orange" => Some(Colour::Orange),
        "Yellow" => Some(Colour::Yellow),
        "yellow" => Some(Colour::Yellow),
        "Green" => Some(Colour::Green),
        "green" => Some(Colour::Green),
        "Blue" => Some(Colour::Blue),
        "blue" => Some(Colour::Blue),
        "Violet" => Some(Colour::Violet),
        "violet" => Some(Colour::Violet),
        "Black" => Some(Colour::Black),
        "black" => Some(Colour::Black),
        "White" => Some(Colour::White),
        "white" => Some(Colour::White),
        _ => None
    };
    
    match input_colour {
        Some(colour) => {
            match colour {
                Colour::Black => {
                    println!("{} is not part of the rainbow!", input_colour_str);
                },
                Colour::White => {
                    println!("{} is not part of the rainbow!", input_colour_str);
                },
                _ => {
                    println!("{} is part of the rainbow!", input_colour_str);
                }
            }
        },
        None => {
            println!("I'm not familiar with the colour {}!", input_colour_str);
        }
    };
}

