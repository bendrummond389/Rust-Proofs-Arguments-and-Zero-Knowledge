use ndarray::prelude::*;
use ndarray::{Array1, Array2};
use std::collections::HashMap;
mod multivariate;
use multivariate::{Monomial, MultivariatePolynomial};

fn main() {}

/*
    Definition of unique multilinear extension (MLE):
    Any function f : {0,1}^v → F has a unique multilinear extension over F, and the
    notation ƒ is used to represent this special extension of f.

    Lemma 3.6 (Lagrange interpolation of multilinear polynomials):
    Let f : {0,1}^v → F be any function. Then the following multilinear polynomial ƒ
    extends f (see Figure 1).

    Given f(w) for all w ∈ {0,1}^v and a vector r = (r1,...,rv) ∈ F^(log(n)), V can
    compute ƒ(r) in O(n) time and O(n) space.

    The right-hand side of Equation 3.1 expresses ƒ(r) as the inner product of two
    n-dimensional vectors, which can be computed in O(n) time using memoization.

    The memoization procedure consists of v = log(n) stages, where each stage j
    constructs a table A^(j) of size 2^j. Notice that A^(j)[(w1,...,wj)] =
    A^(j−1)[(w1,...,wj−1)]·(wjrj + (1−wj)(1−rj)), and thus, the jth stage of the
    memoization procedure requires time O(2^j). The total time across all log(n)
    stages is therefore O(n).

    Our implementation is based on the Lagrange interpolation of multilinear polynomials,
    and leverages the memoization technique to optimize the computation of the basis
    polynomials. The package includes functions for computing the basis polynomials for
    any function defined over the domain {0,1}^v, where v is a positive integer.

*/
