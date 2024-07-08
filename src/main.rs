use std::fmt;

#[derive(Debug)]
enum SudokuCellValue{
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9
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


fn main() {
    let sudoku1:SudokuCellValue = SudokuCellValue::V1;
    println!("Sudoku1: {}",sudoku1);
    println!("Sudoku1 Debug: {:?}",sudoku1);
}
