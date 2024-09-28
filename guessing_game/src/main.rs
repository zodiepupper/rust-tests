use std::time;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    println!("Please input guess");

    let mut guess = String::new();


    let now = time::SystemTime::now();
    let nanonow = now.duration_since(time::UNIX_EPOCH).unwrap().as_millis();

    //let secret_number = (nanonow%100).to_string();
    let secret_number = (nanonow%100).to_string();

    println!("secret is {secret_number}");

    loop {
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        match guess.trim().cmp(&secret_number) {
            Ordering::Equal => println!("got it!!"),
            Ordering::Less => println!("too smol >~<"),
            Ordering::Greater => println!("too lorge @-@"),
        }

        guess = String::new();

    }
    println!("secret was {secret_number}");
}
