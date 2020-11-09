// Entrypoint for optimizing cut plan solver.
use ndarray::Array;
use ndarray::Array2;

#[allow(non_snake_case)]
fn solve() {
    // Initialize data
    let mut b = [96.0, 96.0, 80.0, 104.0, 104.0];
    let mut x = vec![27.0, 27.0, 24.0, 21.0, 51.0, 51.0];
    b.sort_by(|a, b| a.partial_cmp(b).unwrap());
    x.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut x = x.to_vec();
    x.reverse();

    let b = Array::from(b.to_vec());
    let x = Array::from(x);
    let I = b.len();
    let J = x.len();
    let mut A = Array2::<bool>::from_elem((I, J), false);
    let mut M = Array2::<bool>::from_elem((I, J), false);
    let mut pieces_known = 0;
    println!("Boards = {}", b);
    println!("Pieces = {}", x);
}


fn main() {
    solve();
}

