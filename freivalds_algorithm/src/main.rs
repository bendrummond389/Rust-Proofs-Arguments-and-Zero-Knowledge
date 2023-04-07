use ndarray::ArrayBase;
use ndarray::Dim;
use ndarray::{arr1, arr2, Array};
use rand::Rng;

fn main() {
    // we start with an n x n matrix, here we are using 3 x 3 with elements
    // in F_p aka the integers mod p where p = 11

    let a = arr2(&[[1, 4, 7], [10, 13, 16], [2, 5, 8]]);
    let b = arr2(&[[11, 14, 17], [3, 6, 9], [12, 15, 1]]);
    let c = arr2(&[[107, 143, 60], [341, 458, 303], [133, 178, 87]]);

    // next choose a random r in F_p
    let mut rng = rand::thread_rng();
    let r: i32 = rng.gen_range(1..11);

    // create an n x 1 vector x = [1, r, r^2, ..., r^n-1]
    let x = arr1(&[1, r, r.pow(2)]);

    let w = b.dot(&x); // Bx

    let result: bool = c.dot(&x) == a.dot(&w);

    println!("{:?}", c.dot(&x));
    println!("{:?}", w);

    println!("{}", result);
}
