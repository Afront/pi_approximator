 mod leibniz {
        use num_bigint::{BigInt, Sign};
        use num_rational::BigRational;
        use num_traits::{One, Zero};
        use pi_approximator::leibniz::get_pi;

        #[test]
        fn returns_one_when_n_is_zero() {
            assert_eq!(
                get_pi(BigInt::new(Sign::Plus, vec![0])),
                BigRational::new_raw(Zero::zero(), One::one())
            );
        }

        #[test]
        fn returns_the_correct_big_rational_for_other_big_ints() {
            assert_eq!(
                get_pi(BigInt::new(Sign::Plus, vec![1])),
                BigRational::new_raw(BigInt::new(Sign::Plus, vec![4]), One::one())
            );
        }

        #[test]
        fn can_use_regular_ints() {
            assert_eq!(get_pi(0), BigRational::new_raw(Zero::zero(), One::one()));
            assert_eq!(
                get_pi(1),
                BigRational::new_raw(BigInt::new(Sign::Plus, vec![4]), One::one())
            );
        }

        #[test]
        fn can_use_uints() {
            assert_eq!(
                get_pi(0 as u32),
                BigRational::new_raw(Zero::zero(), One::one())
            );
            assert_eq!(
                get_pi(1 as u32),
                BigRational::new_raw(BigInt::new(Sign::Plus, vec![4]), One::one())
            );
        }
    }