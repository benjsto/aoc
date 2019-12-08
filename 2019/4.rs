use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let (mut input1, mut input2) = (String::new(), String::new());

    io::stdout().write_all(b"Enter lower bound: ")?;
    io::stdout().flush()?;

    match io::stdin().read_line(&mut input1) {
        Ok(_n) => (),
        Err(error) => println!("error: {}", error),
    }

    io::stdout().write_all(b"Enter upper bound: ")?;
    io::stdout().flush()?;

    match io::stdin().read_line(&mut input2) {
        Ok(_n) => (),
        Err(error) => println!("error: {}", error),
    }

    input1 = input1.trim_end().to_string();
    input2 = input2.trim_end().to_string();

    println!("lower bound: {}", input1);
    println!("upper bound: {}", input2);

    let low = input1.parse::<i32>().unwrap();
    let high = input2.parse::<i32>().unwrap();

    let mut keepers = vec![];

    for n in low..=high {
        let n_string = n.to_string();

        let (mut prev_int, mut num_digits) = (0, 0);

        for c in n_string.chars() {
            let c_int = c.to_digit(10).unwrap();

            if c_int < prev_int {
                break;
            }

            prev_int = c_int;
            num_digits += 1;
        }

        let mut has_dupes = false;
        prev_int = 0;
        let mut num_dupe_chars = 0;

        for c in n_string.chars() {
            let c_int = c.to_digit(10).unwrap();

            if c_int == prev_int {
                num_dupe_chars += 1;

                if num_dupe_chars == 1 {
                    has_dupes = true;
                } else {
                    has_dupes = false;
                }
            } else {
                if has_dupes {
                    break;
                }

                num_dupe_chars = 0;
            }

            prev_int = c_int;
        }

        if num_digits == 6 && has_dupes {
            keepers.push(n);
        }
    }

    println!("keepers: {:?}", keepers);
    println!("number of keepers: {}", keepers.len());

    Ok(())
}
