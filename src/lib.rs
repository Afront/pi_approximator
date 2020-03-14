pub mod leibniz {
    use num_bigint::{BigInt, Sign};
    use num_rational::BigRational;
    use num_traits::{One, Zero};

    pub trait CanBeBigInt {
        fn as_big_int(self) -> BigInt;
    }

    impl CanBeBigInt for i32 {
        fn as_big_int(self) -> BigInt {
            BigInt::new(Sign::Plus, vec![0]) + self
        }
    }

    impl CanBeBigInt for u32 {
        fn as_big_int(self) -> BigInt {
            BigInt::new(Sign::Plus, vec![0]) + self
        }
    }

    impl CanBeBigInt for BigInt {
        fn as_big_int(self) -> BigInt {
            self
        }
    }

    pub fn get_pi<T: CanBeBigInt>(n: T) -> BigRational {
        let zero = BigInt::new(Sign::Plus, vec![0]);
        let one = BigInt::new(Sign::Plus, vec![1]);
        let two = BigInt::new(Sign::Plus, vec![2]);
        let four = BigInt::new(Sign::Plus, vec![4]);

        let mut total = BigRational::new_raw(Zero::zero(), One::one());
        let mut i = &zero * &zero;
        let big_int = &n.as_big_int();

        while &i < big_int {
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
