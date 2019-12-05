use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn solve(input: &str) {
    let regex = Regex::new(r#"([A-Z])(\d*)"#).unwrap();
    let mut vectors = Vec::new();

    input.lines().for_each(|l| {
        let mut points: HashMap<(i64, i64), i64> = HashMap::new();
        regex.captures_iter(l).fold(((0, 0), 0 as i64), |(current_coords, counter), path| {
            let mut c = counter;
            let direction = &path[1];
            let units = *&path[2].parse::<i64>().unwrap();
            let (x, y) = match direction {
                "L" => {
                    for i in 1..units + 1 {
                        c += 1;
                        if let Entry::Vacant(o) = points.entry((current_coords.0 - i, current_coords.1)) {
                            o.insert(c);
                        }
                    }
                    (current_coords.0 - units, current_coords.1)
                }
                "R" => {
                    for i in 1..units + 1 {
                        c += 1;
                        if let Entry::Vacant(o) = points.entry((current_coords.0 + i, current_coords.1)) {
                            o.insert(c);
                        }
                    }
                    (current_coords.0 + units, current_coords.1)
                }
                "U" => {
                    for i in 1..units + 1 {
                        c += 1;
                        if let Entry::Vacant(o) = points.entry((current_coords.0, current_coords.1 + i)) {
                            o.insert(c);
                        }
                    }
                    (current_coords.0, current_coords.1 + units)
                }
                "D" => {
                    for i in 1..units + 1 {
                        c += 1;
                        if let Entry::Vacant(o) = points.entry((current_coords.0, current_coords.1 - i)) {
                            o.insert(c);
                        }
                    }
                    (current_coords.0, current_coords.1 - units)
                }
                _ => (0, 0),
            };
            ((x, y), c)
        });
        vectors.push(points);
    });

    let line1 = vectors.get(1).unwrap();
    let line2 = vectors.get(0).unwrap();

    let intersections = line1
    .keys()
    .filter(|k| line2.contains_key(k));

    let result1 = intersections.clone()
        .map(|k| i64::abs(k.0) + i64::abs(k.1))
        .min();

    let result2 = intersections.clone()
    .map(|k| {
        i64::abs(*line1.get(k).unwrap()) + i64::abs(*line2.get(k).unwrap())
    }).min();

    println!("Day 3 - Part 1 {:?}", result1);
    println!("Day 3 - Part 2 {:?}", result2);
}
