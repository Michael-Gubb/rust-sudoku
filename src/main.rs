use std::fmt;

#[derive(Debug, Clone, Copy,PartialEq, PartialOrd)]
enum SudokuCellValue {
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

impl fmt::Display for SudokuCellValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SudokuCellValue::V0 => write!(f, "0"),
            SudokuCellValue::V1 => write!(f, "1"),
            SudokuCellValue::V2 => write!(f, "2"),
            SudokuCellValue::V3 => write!(f, "3"),
            SudokuCellValue::V4 => write!(f, "4"),
            SudokuCellValue::V5 => write!(f, "5"),
            SudokuCellValue::V6 => write!(f, "6"),
            SudokuCellValue::V7 => write!(f, "7"),
            SudokuCellValue::V8 => write!(f, "8"),
            SudokuCellValue::V9 => write!(f, "9"),
        }
    }
}

impl SudokuCellValue {
    /// Converts to number
    fn value(&self) -> u8 {
        match self {
            SudokuCellValue::V0 => 0,
            SudokuCellValue::V1 => 1,
            SudokuCellValue::V2 => 2,
            SudokuCellValue::V3 => 3,
            SudokuCellValue::V4 => 4,
            SudokuCellValue::V5 => 5,
            SudokuCellValue::V6 => 6,
            SudokuCellValue::V7 => 7,
            SudokuCellValue::V8 => 8,
            SudokuCellValue::V9 => 9,
        }
    }
    /// Creates enum from value, None if not in valid range
    fn from_value(value: u8) -> Option<SudokuCellValue> {
        match value {
            0 => Some(SudokuCellValue::V0),
            1 => Some(SudokuCellValue::V1),
            2 => Some(SudokuCellValue::V2),
            3 => Some(SudokuCellValue::V3),
            4 => Some(SudokuCellValue::V4),
            5 => Some(SudokuCellValue::V5),
            6 => Some(SudokuCellValue::V6),
            7 => Some(SudokuCellValue::V7),
            8 => Some(SudokuCellValue::V8),
            9 => Some(SudokuCellValue::V9),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

fn main() {
    let sudoku1: SudokuCellValue = SudokuCellValue::V1;
    sudoku1.value();
    let my_value: SudokuCellValue = SudokuCellValue::from_value(3).unwrap();
    println!("Sudoku1: {}", sudoku1);
    println!("Sudoku1 Debug: {:?}", sudoku1);
    println!("Sudoku3: {}", my_value);
}

#[cfg(test)]
mod test {
    use super::SudokuCellValue;
    #[test]
    fn cell_values() {

        // Enum to value 
        let sudoku1: SudokuCellValue = SudokuCellValue::V1;
        assert_eq!(sudoku1.value(), 1_u8);

        let sudoku3: SudokuCellValue = SudokuCellValue::from_value(3).unwrap();
        assert_eq!(sudoku3, SudokuCellValue::V3);        
        assert_eq!(sudoku3.value(), 3_u8);
    }
}
