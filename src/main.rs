use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
enum SudokuCell {
    V0 = 0,
    V1 = 1,
    V2 = 2,
    V3 = 3,
    V4 = 4,
    V5 = 5,
    V6 = 6,
    V7 = 7,
    V8 = 8,
    V9 = 9,
}

impl fmt::Display for SudokuCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SudokuCell::V0 => write!(f, "0"),
            SudokuCell::V1 => write!(f, "1"),
            SudokuCell::V2 => write!(f, "2"),
            SudokuCell::V3 => write!(f, "3"),
            SudokuCell::V4 => write!(f, "4"),
            SudokuCell::V5 => write!(f, "5"),
            SudokuCell::V6 => write!(f, "6"),
            SudokuCell::V7 => write!(f, "7"),
            SudokuCell::V8 => write!(f, "8"),
            SudokuCell::V9 => write!(f, "9"),
        }
    }
}

impl SudokuCell {
    /// Converts to number
    fn value(&self) -> u8 {
        match self {
            SudokuCell::V0 => 0,
            SudokuCell::V1 => 1,
            SudokuCell::V2 => 2,
            SudokuCell::V3 => 3,
            SudokuCell::V4 => 4,
            SudokuCell::V5 => 5,
            SudokuCell::V6 => 6,
            SudokuCell::V7 => 7,
            SudokuCell::V8 => 8,
            SudokuCell::V9 => 9,
        }
    }
    /// Creates enum from value, None if not in valid range
    fn from_value(value: u8) -> Option<SudokuCell> {
        match value {
            0 => Some(SudokuCell::V0),
            1 => Some(SudokuCell::V1),
            2 => Some(SudokuCell::V2),
            3 => Some(SudokuCell::V3),
            4 => Some(SudokuCell::V4),
            5 => Some(SudokuCell::V5),
            6 => Some(SudokuCell::V6),
            7 => Some(SudokuCell::V7),
            8 => Some(SudokuCell::V8),
            9 => Some(SudokuCell::V9),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
enum SudokuRow {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
enum SudokuColumn {
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
enum FilledSudokuError {
    Length(usize),
    InvalidCharacter(char, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
struct FilledSudoku {
    values: [SudokuCell; 81],
}
impl FilledSudoku {
    // Converts from string without formatting, does not check for validity
    fn from_string(r: &str) -> Result<FilledSudoku, FilledSudokuError> {
        let mut values = [SudokuCell::V0; 81];
        let string_chars: Vec<char> = r.chars().collect();
        if string_chars.len() != 81 {
            return Err(FilledSudokuError::Length(string_chars.len()));
        }
        for (i, c) in string_chars.iter().enumerate() {
            let n = c.to_digit(10);
            if n.is_none() {
                return Err(FilledSudokuError::InvalidCharacter(*c, i));
            } else {
                values[i] = SudokuCell::from_value(n.unwrap() as u8).unwrap();
            }
        }
        Ok(FilledSudoku { values })
    }
}

fn main() {
    let sudoku1: SudokuCell = SudokuCell::V1;
    sudoku1.value();
    let my_value: SudokuCell = SudokuCell::from_value(3).unwrap();
    println!("Sudoku1: {}", sudoku1);
    println!("Sudoku1 Debug: {:?}", sudoku1);
    println!("Sudoku3: {}", my_value);
}

#[cfg(test)]
mod test {
    use super::FilledSudoku;
    use super::FilledSudokuError;
    use super::SudokuCell;
    #[test]
    fn cell_values() {
        // Enum to value
        let sudoku1: SudokuCell = SudokuCell::V1;
        assert_eq!(sudoku1.value(), 1_u8);

        let sudoku3: SudokuCell = SudokuCell::from_value(3).unwrap();
        assert_eq!(sudoku3, SudokuCell::V3);
        assert_eq!(sudoku3.value(), 3_u8);
    }
    #[test]
    fn filled_sudoku() {
        let broken_sudoku1 = FilledSudoku::from_string("r");
        assert_eq!(broken_sudoku1, Err(FilledSudokuError::Length(1)));
    }
}
