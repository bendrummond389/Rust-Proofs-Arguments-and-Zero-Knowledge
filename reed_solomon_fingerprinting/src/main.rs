use ascii_converter::string_to_decimals;
use rand::{thread_rng, Rng};

use polynomial::Polynomial;

// there are a total of 128 ASCII characters which means m = 128
// let's choose a string length of n = 12
// we then choose a p > max{m, n^2}, so p = 149

fn main() {
    // alices string is going to be "hello world" which is 11 characters
    // plus a terminating character
    let alices_string = "hello world";
    let bobs_string = "hell world";

    // Alice converts her string to a vector and then a polynomial
    let alices_vector: Vec<u8> = string_to_decimals(&alices_string).unwrap();
    let alices_poly: Polynomial<u64> =
        Polynomial::new(alices_vector.iter().map(|&x| x as u64).collect());

    // Alice then picks a random r <= n
    let mut rng = thread_rng();
    let r = rng.gen_range(0, 12);

    // alice then evaluates her hash function (polynomial) at r; v = h(r) = p_a(r)
    let v = alices_poly.eval(r);

    // Alice then sends v = p(r) and r to Bob and Bob evluates his polynomial at r p_b

    let bobs_vector: Vec<u8> = string_to_decimals(&bobs_string).unwrap();
    let bobs_poly: Polynomial<u64> =
        Polynomial::new(bobs_vector.iter().map(|&x| x as u64).collect());
    let result: bool = bobs_poly.eval(r) == v;

    // From "Proofs, Arguments, and Zero Knowledge," written by Justin Thaler:
    // Fact 2.1
    // Given that For any two distinct (i.e., unequal) polynomials p_a, p_b of degree at most n with
    // coefficients in F_p (the set integers mod p), p_a(x) = p_b(x) for at most n values of x in F_p

    // both p_a and p_b are polynomials in x of degree at most n − 1. The value v that Alice sends to
    // Bob in the communication protocol is precisely p_a(r), and Bob compares this value to p_b(r).
    // By Fact 2.1, if there is even one i such that ai ̸= bi, then there are at most n − 1 values of r
    // such that p_a(r) = p_b(r). Since r is chosen at random from Fp, the probability that Alice picks
    //  such an r is thus at most (n−1)/p. Hence, Bob outputs NOT-EQUAL with probability at least
    // 1−(n−1)/p (where the probability is over the random choice of r)

    // We know:

    // - If ai = bi for all i = 1,...,n. Then Bob outputs EQUAL for every possible choice of r.
    // - If there is even one i such that ai ̸= bi, then Bob outputs NOT-EQUAL with probability at
    //   least 1−(n−1)/p, which is at least 1−1/n

    // Bob outputs NOT-EQUAL with probability at least 1−(n−1)/p (where the probability is over the
    // random choice of r). 1 - (12 - 1) / 149 evaluates to approximately 0.9215. This is the probability 
    // that Bob outputs NOT-EQUAL, given that Alice's and Bob's strings differ in at least one character, 
    // and assuming that r is chosen uniformly at random from the set of integers modulo p = 149.

    if result {
        println!("there is a 92.15% probility that alice and bob have the same string");
    } else {
        println!("the strings are different")
    }
    let size_v = std::mem::size_of_val(&v);
    let size_r = std::mem::size_of_val(&r);
    let size_alices_string = std::mem::size_of_val(&alices_string) * (alices_string.len());


    println!("r: {} \nv: {} \n{:?}", r, v, alices_vector);
    println!("size of v: {} \nsize of r: {}\nsize of alices string: {}",size_v, size_r, size_alices_string );

}
