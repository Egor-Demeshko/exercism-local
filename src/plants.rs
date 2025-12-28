use std::collections::HashMap;

const students: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];
const GRASS: &str = "grass";
const CLOVER: &str = "clover";
const RADISH: &str = "radishes";
const VIOLET: &str = "violets";

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut plants: Vec<&'static str> = vec![];

    if diagram.is_empty() {
        return plants;
    }

    let flowers = get_flowers();
    let student_index = match get_student_index(student) {
        Some(index) => index * 2,
        None => return plants,
    };
    let rows: Vec<&str> = get_rows(diagram);
    if rows.len() != 2 {
        return plants;
    }

    for row in rows {
        let slice = &row[student_index..=student_index + 1];
        for char in slice.chars() {
            let plant = flowers.get(&char).unwrap();
            plants.push(plant);
        }
    }

    plants
}

fn get_flowers() -> HashMap<char, &'static str> {
    HashMap::from([('G', GRASS), ('C', CLOVER), ('R', RADISH), ('V', VIOLET)])
}

fn get_student_index(student: &str) -> Option<usize> {
    students.iter().position(|st: &&str| st == &student)
}

fn get_rows(diagram: &str) -> Vec<&str> {
    diagram.split("\n").map(|row: &str| row.trim()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn garden_with_single_student() {
        let diagram = "RC
GG";
        let student = "Alice";
        let expected = vec!["radishes", "clover", "grass", "grass"];
        assert_eq!(plants(diagram, student), expected);
    }

    #[test]
    fn different_garden_with_single_student() {
        let diagram = "VC
RC";
        let student = "Alice";
        let expected = vec!["violets", "clover", "radishes", "clover"];
        assert_eq!(plants(diagram, student), expected);
    }

    #[test]
    fn for_eve() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Eve";
        let expected = vec!["clover", "grass", "radishes", "grass"];
        assert_eq!(plants(diagram, student), expected);
    }
}
