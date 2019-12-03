use std::env;
use std::io;
use std::io::prelude::*;

fn calculate_fuel_total(elements:Vec<String>, include_fuel_mass:bool) -> i32 {
    let mut sum : i32 = 0;

    for el in elements {
        sum = sum + calculate_fuel_for_element(el.parse::<i32>().unwrap(), include_fuel_mass);
    }

    return sum;
}

fn calculate_fuel_for_element(mass:i32, include_fuel_mass:bool) -> i32 {
    let fuel = mass/3 - 2;

    if fuel < 0 {
        return 0;
    }

    if include_fuel_mass {
        return fuel + calculate_fuel_for_element(fuel, include_fuel_mass);
    }

    return fuel;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut part: String = "".to_string();

    if args.len() > 1 {
        part = args[1].clone();
    }

    let mut elements = vec![];

    let s = io::stdin();

    for line in s.lock().lines() {
        let l = line.unwrap().clone();

        if l == "" {
            break;
        }

        elements.push(l);
    }

    let mut include_fuel_mass: bool = false;

    if part == "2" {
        include_fuel_mass = true;
    }

    let sum = calculate_fuel_total(elements, include_fuel_mass);

    println!("part: {}", part);
    println!("sum: {}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use calculate_fuel_for_element;

    #[test]
    fn test_against_given_values_part_1() {
        assert_eq!(calculate_fuel_for_element(14, false), 2);
        assert_eq!(calculate_fuel_for_element(12, false), 2);
        assert_eq!(calculate_fuel_for_element(1969, false), 654);
        assert_eq!(calculate_fuel_for_element(100756, false), 33583);
    }

    #[test]
    fn test_against_given_values_part_2() {
        assert_eq!(calculate_fuel_for_element(14, true), 2);
        assert_eq!(calculate_fuel_for_element(1969, true), 966);
        assert_eq!(calculate_fuel_for_element(100756, true), 50346);
    }
}
