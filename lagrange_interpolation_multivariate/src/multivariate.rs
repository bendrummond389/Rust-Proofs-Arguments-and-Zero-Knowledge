#[derive(Debug, Clone, PartialEq)]

/*  As a suitable Rust crate for implementing multivariate polynomials was not available,
   developed our own package in multivariate.rs to ensure accurate computation of multivariate
   Lagrange basis polynomials.

   These basis polynomials are essential in the construction of interactive proofs for multivariate
   functions, allowing for efficient verification and low communication costs. By implementing our own
   package, we can ensure the highest level of precision and efficiency in our interactive proof systems.
*/

pub struct Monomial {
    pub coefficient: f64,
    pub exponents: Vec<u32>,
}

impl Monomial {
    pub fn is_equal(&self, other: &Self) -> bool {
        // due to issuse pertaining to rounding errors and limitations in their
        // representation we use a tolerance (EPSILON) while determining
        // equivelence
        if (self.coefficient - other.coefficient).abs() < f64::EPSILON {
            for i in 0..self.exponents.len() {
                if self.exponents[i] != other.exponents[i] {
                    return false;
                }
            }
            return true;
        }
        return false;
    }
}

// Define a multivariate polynomial struct to store monomials
#[derive(Debug, Clone)]
pub struct MultivariatePolynomial {
    pub monomials: Vec<Monomial>,
}

impl MultivariatePolynomial {
    // Create a new multivariate polynomial from a list of monomials
    pub fn new(monomials: Vec<Monomial>) -> Self {
        MultivariatePolynomial { monomials }
    }

    // Evaluate the multivariate polynomial at a given point
    pub fn evaluate(&self, point: &[f64]) -> f64 {
        let mut result = 0.0;

        for monomial in &self.monomials {
            let mut monomial_value = monomial.coefficient;
            for (variable_exponent, variable_value) in monomial.exponents.iter().zip(point) {
                monomial_value *= variable_value.powi(*variable_exponent as i32);
            }
            result += monomial_value;
        }

        result
    }
}
