mod operations;
mod modules;

use operations::*;

fn main() {

    println!("
Welcome to matrizal, a simple matrix calculator, now in your terminal!

Please choose one of the following operations to perform

    1 - Sum 2 matrixes.
    2 - Subtract 2 matrixes.
    3 - Linearly combine 2 matrixes
        
    4 - Multiply a matrix by an escalar.
    5 - Multiply 2 matrixes together.
    6 - 'Square' a matrix (square matrixes only)

    7 - Calculate the determinant of a matrix. (Order 2 or 3)
        
    8 - Transpose a matrix.
    9 - Generate a random matrix.

Please input what you want to do");

    let mut i = String::new();

    //Query
    
    std::io::stdin()
        .read_line(&mut i)
        .expect("Failed to read line");

    //  Conversion

    let i = match i.trim().parse() {
        Ok(n) => n,
        Err(_) => 0,
    };

    match i {
        1 => {summatrix()},
        2 => {submatrix()},
        3 => {lc-matrix()},
        4 => {escmatrix()},
        5 => {multmatrix()},
        6 => {sqmatrix()},
        7 => {determatrix()},
        8 => {transmatrix()},
        9 => {randomatrix()},
        _ => {println!("Not a valid choice")},
    };
}