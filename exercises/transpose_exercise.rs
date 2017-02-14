/* Add a transpose function using the reverse function as a template, which accepts a matrix as
 an argument, and returns a matrix in which two elements have been swapped.

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
} */

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("---");
    println!("{:?}", transpose(matrix));
}
