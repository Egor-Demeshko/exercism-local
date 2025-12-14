const PLING: &str = "Pling";
const PLANG: &str = "Plang";
const PLONG: &str = "Plong";

pub fn raindrops(n: u32) -> String {
    let mut result = String::new();
    if n % 3 == 0 {
        result.push_str(PLING);
    }

    if n % 5 == 0 {
        result.push_str(PLANG);
    }

    if n % 7 == 0 {
        result.push_str(PLONG);
    }

    if result.is_empty() {
        format!("{n}")
    } else {
        result
    }
}
