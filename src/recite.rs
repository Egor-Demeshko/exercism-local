use std::collections::HashMap;

const SONG: &str = "Ten green bottles hanging on the wall,
Ten green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be nine green bottles hanging on the wall.

Nine green bottles hanging on the wall,
Nine green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be eight green bottles hanging on the wall.

Eight green bottles hanging on the wall,
Eight green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be seven green bottles hanging on the wall.

Seven green bottles hanging on the wall,
Seven green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be six green bottles hanging on the wall.

Six green bottles hanging on the wall,
Six green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be five green bottles hanging on the wall.

Five green bottles hanging on the wall,
Five green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be four green bottles hanging on the wall.

Four green bottles hanging on the wall,
Four green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be three green bottles hanging on the wall.

Three green bottles hanging on the wall,
Three green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be two green bottles hanging on the wall.

Two green bottles hanging on the wall,
Two green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be one green bottle hanging on the wall.

One green bottle hanging on the wall,
One green bottle hanging on the wall,
And if one green bottle should accidentally fall,
There'll be no green bottles hanging on the wall.";

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut quantrains: Vec<&str> = SONG.split("\n\n").collect();

    let start_index = start_bottles.abs_diff(10);
    dbg!(start_index);
    let end_index = start_index + take_down;

    quantrains
        .drain(start_index as usize..end_index as usize)
        .collect::<Vec<&str>>()
        .join("\n\n")
}

mod tests {
    use super::*;

    fn first_generic_verse() {
        assert_eq!(
            recite(10, 1).trim(),
            concat!(
                "Ten green bottles hanging on the wall,\n",
                "Ten green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be nine green bottles hanging on the wall.",
            )
        );
    }

    fn last_generic_verse() {
        assert_eq!(
            recite(3, 1).trim(),
            concat!(
                "Three green bottles hanging on the wall,\n",
                "Three green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be two green bottles hanging on the wall.",
            )
        );
    }

    #[test]
    fn all_versus() {
        assert_eq!(
            recite(10, 10).trim(),
            concat!(
                "Ten green bottles hanging on the wall,\n",
                "Ten green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be nine green bottles hanging on the wall.\n",
                "\n",
                "Nine green bottles hanging on the wall,\n",
                "Nine green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be eight green bottles hanging on the wall.\n",
                "\n",
                "Eight green bottles hanging on the wall,\n",
                "Eight green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be seven green bottles hanging on the wall.\n",
                "\n",
                "Seven green bottles hanging on the wall,\n",
                "Seven green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be six green bottles hanging on the wall.\n",
                "\n",
                "Six green bottles hanging on the wall,\n",
                "Six green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be five green bottles hanging on the wall.\n",
                "\n",
                "Five green bottles hanging on the wall,\n",
                "Five green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be four green bottles hanging on the wall.\n",
                "\n",
                "Four green bottles hanging on the wall,\n",
                "Four green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be three green bottles hanging on the wall.\n",
                "\n",
                "Three green bottles hanging on the wall,\n",
                "Three green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be two green bottles hanging on the wall.\n",
                "\n",
                "Two green bottles hanging on the wall,\n",
                "Two green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be one green bottle hanging on the wall.\n",
                "\n",
                "One green bottle hanging on the wall,\n",
                "One green bottle hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be no green bottles hanging on the wall.",
            )
        );
    }
}
