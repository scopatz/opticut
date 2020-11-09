// Entrypoint for optimizing cut plan solver.
use ndarray::Array;
use ndarray::Array2;
use ndarray::s;

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
    let mut A = Array2::<f32>::zeros((I, J));
    let mut M = Array2::<bool>::from_elem((I, J), false);
    let mut pieces_known = 0;
    let w = 0.125;

    // main loop
    for i in 0..I {
        for j in 0..J {
            if M[[i, j]] {
                continue;
            }
            A[[i, j]] = 1.0;
            let usable = A.slice(s![i, ..]).dot(&x) <= b[i];
            A[[i, j]] = if usable { 1.0 } else { 0.0 };
            M[[i, j]] = true;
            if usable {
                pieces_known += 1;
                M.slice_mut(s![i+1.., j]).fill(true);
            }
            if pieces_known == J {
                break;
            }
        }
        if pieces_known == J {
            break;
        }
    }

    println!("Boards = {}", b);
    println!("Pieces = {}", x);
    println!("Solution =\n{}", A);
    println!("Known =\n{}", M);
    println!("piece_known = {}", pieces_known);
}


fn main() {
    solve();
}

