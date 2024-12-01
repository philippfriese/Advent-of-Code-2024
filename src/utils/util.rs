use std::fs;
use std::path::Path;
pub fn load_data(fname: &str) -> String {
    fs::read_to_string(fname).expect("Oops")
}

pub fn load<T>(test: bool, this: &str, parser: impl Fn(&str) -> T) -> Vec<T> {
    let basepath = Path::new(this).parent().unwrap();
    split_data::<T>(
        load_data(basepath.join(if test {"test.dat"} else {"data.dat"})
            .to_str().unwrap()),
        parser)
}

pub fn load_default<T>(test: bool, this: &str) -> Vec<T>
where <T as std::str::FromStr>::Err: std::fmt::Debug, T: std::str::FromStr
{
    load(test, this, |s: &str| -> T {
        s.parse::<T>().unwrap()
    })
}


pub fn split_data<T>(data: String, parser: impl Fn(&str) -> T) -> Vec<T> {
    data
        .split("\n")
        .map(|x| parser(x))
        .collect::<Vec<T>>()
}