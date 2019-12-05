use std::io;
use std::io::prelude::*;

fn apply_frequency_changes(starting:i32, elements:Vec<String>) -> i32 {
    let mut result_frequency:i32 = starting;

    for el in elements {
        result_frequency = apply_frequency_change(result_frequency, el.parse::<i32>().unwrap())
    }

    return result_frequency;
}

fn apply_frequency_changes_and_find_repeat(starting:i32, elements:Vec<String>) -> i32 {
    let mut result_frequency:i32 = starting;

    let mut result_freqs:Vec<i32> = vec![starting];

    let mut found_repeat = false;

    loop {
        for el in &elements {
            result_frequency = apply_frequency_change(result_frequency, el.parse::<i32>().unwrap());

            if (&result_freqs).into_iter().any(|&f| f == result_frequency) {
                found_repeat = true;
                break;
            }

            result_freqs.push(result_frequency);

            println!("current: {}", result_frequency);
        }

        if found_repeat {
            break;
        }
    }

    result_frequency
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

    // let result = apply_frequency_changes(0, elements);

    let result = apply_frequency_changes_and_find_repeat(0, elements);

    println!("result: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use *;

    #[test]
    fn test_apply_frequency_change() {
        assert_eq!(apply_frequency_change(0, 1), 1);
        assert_eq!(apply_frequency_change(1, -2), -1);
        assert_eq!(apply_frequency_change(-1, 3), 2);
        assert_eq!(apply_frequency_change(2, 1), 3);
    }

    #[test]
    fn test_apply_frequency_changes() {
        assert_eq!(apply_frequency_changes(0, vec![String::from("+1"), String::from("+1"), String::from("+1")]), 3);
        assert_eq!(apply_frequency_changes(0, vec![String::from("+1"), String::from("+1"), String::from("-2")]), 0);
        assert_eq!(apply_frequency_changes(0, vec![String::from("-1"), String::from("-2"), String::from("-3")]), -6);
    }

    #[test]
    fn test_apply_frequency_changes_and_find_repeat() {
        assert_eq!(apply_frequency_changes_and_find_repeat(0, vec![String::from("+1"), String::from("-1")]), 0);
        assert_eq!(apply_frequency_changes_and_find_repeat(0, vec![String::from("+3"), String::from("+3"), String::from("+4"), String::from("-2"), String::from("-4")]), 10);
        assert_eq!(apply_frequency_changes_and_find_repeat(0, vec![String::from("+7"), String::from("+7"), String::from("-2"), String::from("-7"), String::from("-4")]), 14);
    }
}
