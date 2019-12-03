use std::io;
use std::io::prelude::*;

fn apply_frequency_changes(starting:i32, elements:Vec<String>) -> i32 {
    let mut result_frequency:i32 = starting;

    for el in elements {
        result_frequency = apply_frequency_change(result_frequency, el.parse::<i32>().unwrap())
    }

    return result_frequency;
}

fn apply_frequency_change(current:i32, change:i32) -> i32 {
    return current + change;
}

fn main() -> io::Result<()> {
    let mut elements = vec![];

    let s = io::stdin();

    for line in s.lock().lines() {
        let l = line.unwrap().clone();

        if l == "" {
            break;
        }

        elements.push(l);
    }

    let result = apply_frequency_changes(0, elements);

    println!("result: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use apply_frequency_change;
    use apply_frequency_changes;

    #[test]
    fn test_against_given_values_part_1() {
        assert_eq!(apply_frequency_change(0, 1), 1);
        assert_eq!(apply_frequency_change(1, -2), -1);
        assert_eq!(apply_frequency_change(-1, 3), 2);
        assert_eq!(apply_frequency_change(2, 1), 3);

        assert_eq!(apply_frequency_changes(0, vec![String::from("+1"), String::from("+1"), String::from("+1")]), 3);
        assert_eq!(apply_frequency_changes(0, vec![String::from("+1"), String::from("+1"), String::from("-2")]), 0);
        assert_eq!(apply_frequency_changes(0, vec![String::from("-1"), String::from("-2"), String::from("-3")]), -6);
    }
}
