use ndarray::Array1;
use std::f64;

// The Lagrange interpolation formula is:
// L(x) = f(x0) * l0(x) + f(x1) * l1(x) + ... + f(xn) * ln(x)
// where:
// L(x) is the Lagrange polynomial.
// f(xi) is the value of the function at the point xi.
// li(x) is the Lagrange basis polynomial, defined as li(x) = Π((x-xj)/(xi-xj)), for j=0, 1, ..., n and j≠i.

// This function calculates the Lagrange polynomial given a set of x_values, y_values, and a point x.
fn lagrange_polynomial(x_values: &Array1<f64>, y_values: &Array1<f64>, x: f64) -> f64 {
    let mut result = 0.0;

    for i in 0..x_values.len() {
        let mut li = 1.0;
        for j in 0..x_values.len() {
            // Skip the case when i == j to avoid division by zero.
            if i != j {
                li *= (x - x_values[j]) / (x_values[i] - x_values[j]);
            }
        }
        result += y_values[i] * li;
    }

    result
}

fn main() {
    // Define the x_values and y_values for the known data points.
    let x_values = ndarray::arr1(&[0.0, 1.0, 2.0, 3.0]);
    let y_values = ndarray::arr1(&[1.0, 3.0, 9.0, 27.0]);

    // Define the x-coordinate where the interpolation is required.
    let x_interp = 1.5;
    // Calculate the interpolated y-coordinate using the Lagrange polynomial.
    let y_interp = lagrange_polynomial(&x_values, &y_values, x_interp);

    println!("Interpolated value at x = {} is y = {}", x_interp, y_interp);
}
