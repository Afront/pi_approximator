mod leibniz {
    use num_bigint::{BigInt, Sign};
    use num_rational::BigRational;
    use num_traits::{One, Zero};

    pub fn get_pi(n: BigInt) -> BigRational {
        let zero = BigInt::new(Sign::Plus, vec![0]);
        let one = BigInt::new(Sign::Plus, vec![1]);
        let two = BigInt::new(Sign::Plus, vec![2]);
        let four = BigInt::new(Sign::Plus, vec![4]);

        let mut total = BigRational::new_raw(Zero::zero(), One::one());
        let mut i = &zero * &zero;

        while &i < &n {
            total += BigRational::new_raw(
                &four * &one,
                (&i * &two + &one)
                    * if &i % &two == &zero * &zero {
                        &one + &zero // Probably better than &one * &one
                    } else {
                        &zero - &one
                    },
            );

            i += &one * &one;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    mod leibniz {
        use crate::leibniz::get_pi;
        use num_bigint::{BigInt, Sign};
        use num_rational::BigRational;
        use num_traits::{One, Zero};

        #[test]
        fn returns_one_when_n_is_zero() {
            assert_eq!(
                get_pi(BigInt::new(Sign::Plus, vec![0])),
                BigRational::new_raw(Zero::zero(), One::one())
            );
        }

        #[test]
        fn returns_the_correctt_big_rational_for_other_big_ints() {
            assert_eq!(
                get_pi(BigInt::new(Sign::Plus, vec![1])),
                BigRational::new_raw(BigInt::new(Sign::Plus, vec![4]), One::one())
            );
        }
    }
}