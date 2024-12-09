use rayon::prelude::*;

advent_of_code::solution!(4);

pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub struct Puzzle {
    pub height: usize,
    pub width: usize,
    pub chars: Vec<char>,
}

impl Puzzle {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().filter(|c| c.is_alphabetic()).collect();
        let height = input.lines().count();
        let width = chars.len() / height;

        Self {
            height,
            width,
            chars,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &char> {
        self.chars.iter()
    }

    pub fn get_char(&self, row: usize, col: usize) -> Option<&char> {
        let index = row * self.width + col;
        self.chars.get(index)
    }

    #[inline(always)]
    pub fn find_word(&self, direction: Direction, row: usize, col: usize, word: &[char]) -> u8 {
        let mut x = col;
        let mut y = row;

        let word_len = word.len() - 1;

        if !match direction {
            Direction::North => y >= word_len,
            Direction::NorthEast => y >= word_len && x + word_len <= self.width,
            Direction::East => x + word_len <= self.width,
            Direction::SouthEast => x + word_len <= self.width && y + word_len <= self.height,
            Direction::South => y + word_len <= self.height,
            Direction::SouthWest => x >= word_len && y + word_len <= self.height,
            Direction::West => x >= word_len,
            Direction::NorthWest => x >= word_len && y >= word_len,
        } {
            return 0;
        }

        for letter in word.iter().take(word_len + 1) {
            let this_char = self.get_char(y, x);

            if this_char != Some(letter) {
                return 0;
            }

            match direction {
                Direction::North => {
                    y = y.saturating_sub(1);
                }
                Direction::NorthEast => {
                    x = x.saturating_add(1);
                    y = y.saturating_sub(1);
                }
                Direction::East => {
                    x = x.saturating_add(1);
                }
                Direction::SouthEast => {
                    x = x.saturating_add(1);
                    y = y.saturating_add(1);
                }
                Direction::South => {
                    y = y.saturating_add(1);
                }
                Direction::SouthWest => {
                    x = x.saturating_sub(1);
                    y = y.saturating_add(1);
                }
                Direction::West => {
                    x = x.saturating_sub(1);
                }
                Direction::NorthWest => {
                    x = x.saturating_sub(1);
                    y = y.saturating_sub(1);
                }
            };
        }

        1
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let word = ['X', 'M', 'A', 'S'];

    let puzzle = Puzzle::new(input);

    Some(
        (0..puzzle.height)
            .into_par_iter()
            .map(|row| -> u32 {
                let mut count = 0u32;

                for col in 0..puzzle.width {
                    let index = row * puzzle.width + col;
                    let c = puzzle.chars[index];

                    if c != 'X' {
                        continue;
                    }

                    count += (puzzle.find_word(Direction::North, row, col, &word)
                        + puzzle.find_word(Direction::NorthEast, row, col, &word)
                        + puzzle.find_word(Direction::East, row, col, &word)
                        + puzzle.find_word(Direction::SouthEast, row, col, &word)
                        + puzzle.find_word(Direction::South, row, col, &word)
                        + puzzle.find_word(Direction::SouthWest, row, col, &word)
                        + puzzle.find_word(Direction::West, row, col, &word)
                        + puzzle.find_word(Direction::NorthWest, row, col, &word))
                        as u32;
                }

                count
            })
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
