use num_bigint::{BigInt, Sign};
use num_rational::BigRational;
use num_traits::{One, Zero};

fn leibniz(n: BigInt) -> BigRational {
    let zero = BigInt::new(Sign::Plus, vec![0]);
    let one = BigInt::new(Sign::Plus, vec![1]);
    let two = BigInt::new(Sign::Plus, vec![2]);

    let mut total = BigRational::new_raw(Zero::zero(), One::one());
    let mut i = BigInt::new(Sign::Plus, vec![0]);

    while &i < &n {
        let left = BigInt::new(Sign::Plus, vec![4]);
        let right = (&i * &two + &one)
            * if &i % &two == &zero * &zero {
                &one * &one
            } else {
                &one * -&one
            };

        let new = BigRational::new_raw(left, right);
        total += new;
        i += &one * &one;
    }
    total
}

fn main() {
    let a = leibniz(BigInt::new(Sign::Plus, vec![100]));
    println!("{}", a);
}
