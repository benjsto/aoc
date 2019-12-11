
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let mut count = 0;
    let orbits = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| (l.split(")").nth(1).unwrap().to_string(), l.split(")").nth(0).unwrap().to_string()))
        .collect::<HashMap<_,_>>();

    println!("orbits: {:?}", orbits);

    // part 1
    for mut body in orbits.keys() {
        while let Some(parent) = orbits.get(body) {
            count += 1;
            body = parent;
        }
    }

    println!("{}", count);

    // part 2
    let mut you_parents = HashSet::new();
    let mut san_parents = HashSet::new();

    let mut body = "YOU";

    while let Some(parent) = orbits.get(body) {
        you_parents.insert(parent);
        body = parent;
    }

    body = "SAN";

    while let Some(parent) = orbits.get(body) {
        san_parents.insert(parent);
        body = parent;
    }

    println!("you_parents: {:?}", you_parents);
    println!("san_parents: {:?}", san_parents);

    let common_parents = you_parents.intersection(&san_parents);

    println!("common_parents: {:?}", common_parents);

    let mut lowest_count = 100000;

    for com in common_parents {
        let mut count = 0;

        body = "YOU";

        while let Some(parent) = orbits.get(body) {
            body = parent;

            if &parent == com { break; }

            count += 1;
        }

        body = "SAN";

        while let Some(parent) = orbits.get(body) {
            body = parent;

            if &parent == com { break; }

            count += 1;
        }

        if count < lowest_count { lowest_count = count; }
    }

    println!("{}", lowest_count);
}
