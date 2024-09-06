use crate::filter;

pub fn check(name: &str, value: &str) -> bool {
    let e = filter::get(name).unwrap();

    e.target_value == value
}
