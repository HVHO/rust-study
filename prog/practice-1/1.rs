use std::io::{stdin,stdout,Write};

fn main () {
    let mut s = String::new();

        print!("Please enter number: ");
            stdout().flush();

            stdin()
                .read_line(&mut s)
                .expect("Fail to read from stdin");

            let trimmed = s.trim();
            match trimmed.parse::<u32>() {
                Ok(i) => println!("{}",2*i),
                Err(..) => println!("error"),
            };
}
