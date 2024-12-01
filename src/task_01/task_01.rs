use crate::utils;

use regex::Regex;

fn parse(pattern: Regex, s: &str) -> (u32, u32) {
    let res = pattern.captures(s).unwrap();
    (res.get(1).unwrap().as_str().parse().unwrap(),
     res.get(2).unwrap().as_str().parse().unwrap())
}
pub fn first(test: bool) {
    let pattern = Regex::new(r"(\d+)\s+(\d+)$").unwrap();
    let parser = |s: &str| {
      parse(pattern.clone(), s)
    };

    let lines = utils::util::load::<(u32,u32)>(test, file!(), parser);
    let mut left = lines.iter().map(|t| {
        return t.0
    }).collect::<Vec<u32>>();
    left.sort();

    let mut right = lines.iter().map(|t| {
        return t.1
    }).collect::<Vec<u32>>();
    right.sort();

    let mut acc = 0;
    left.iter().zip(right.iter()).for_each(|(a, b)| {
        acc += a.abs_diff(*b);
    });
    println!("First: {:?}", acc);
}

pub fn second(test: bool) {
    let pattern = Regex::new(r"(\d+)\s+(\d+)$").unwrap();
    let parser = |s: &str| {
        parse(pattern.clone(), s)
    };

    let lines = utils::util::load::<(u32,u32)>(test, file!(), parser);
    let mut left = lines.iter().map(|t| {
        return t.0
    }).collect::<Vec<u32>>();
    left.sort();

    let mut right = lines.iter().map(|t| {
        return t.1
    }).collect::<Vec<u32>>();
    right.sort();

    let mut acc = 0;
    left.iter().for_each(|x| {
    let occurrences: Vec<_> = right.iter().filter(|y| *x == **y).collect();
        acc += *x * occurrences.len() as u32;
    });
    println!("Second: {:?}", acc);
}