use std::collections::HashMap;

fn compute_multivariate_lagrange_basis_polynomials(x: &[u64]) -> Vec<f64> {
  // Definition of unique multilinear extension (MLE):
  // Any function f : {0,1}^v → F has a unique multilinear extension over F, and the 
  // notation ƒ is used to represent this special extension of f.

  // Lemma 3.6 (Lagrange interpolation of multilinear polynomials):
  // Let f : {0,1}^v → F be any function. Then the following multilinear polynomial ƒ
  //  extends f (see Figure 1).

  // Given f(w) for all w ∈ {0,1}^v and a vector r = (r1,...,rv) ∈ F^(log(n)), V can 
  // compute ƒ(r) in O(n) time and O(n) space.

  // The right-hand side of Equation 3.1 expresses ƒ(r) as the inner product of two 
  // n-dimensional vectors, which can be computed in O(n) time using memoization.

  // The memoization procedure consists of v = log(n) stages, where each stage j 
  // constructs a table A^(j) of size 2^j. Notice that A^(j)[(w1,...,wj)] = 
  // A^(j−1)[(w1,...,wj−1)]·(wjrj + (1−wj)(1−rj)), and thus, the jth stage of the 
  // memoization procedure requires time O(2^j). The total time across all log(n) 
  // stages is therefore O(n).

  // Here, we use a hashmap to represent our table.
  let mut memo = HashMap::new();
  let mut basis_polynomials = vec![0.0; x.len()];
  for i in 0..x.len() {
      basis_polynomials[i] = compute_lagrange_basis_polynomial(x, i, &mut memo);
  }
  return basis_polynomials;
}

fn compute_lagrange_basis_polynomial(x: &[u64], i: usize, memo: &mut HashMap<usize, f64>) -> f64 {
    if let Some(&basis) = memo.get(&i) {
        return basis;
    } else {
        let si: Vec<usize> = (0..x.len()).filter(|&j| j != i).collect();
        let mut basis = 1.0;
        for &j in &si {
            basis *= (x[i] as f64 - x[j] as f64) / (x[i] as f64 - x[j] as f64);
        }
        memo.insert(i, basis);
        return basis;
    }
}

fn main() {
    let x = vec![1, 2, 3];
    let basis_polynomials = compute_multivariate_lagrange_basis_polynomials(&x);
    println!("{:?}", basis_polynomials);
}
