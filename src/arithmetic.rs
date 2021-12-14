use num::bigint::BigUint;
use sha2::{ Sha256, Digest };

/*
 * Given a string, hash it with SHA256 into a big unit.
 */
fn hash(input: &String) -> BigUint {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let arr = hasher.finalize();
    BigUint::from_bytes_be(&arr)
}

/* 
 * Given a target n,
 * a left_border: tuple(left, left_inclusive),
 * a right_border: tuple(right, right_inclusive),
 * return whether the target n in in the range defined by left_border and right_border.
 */
pub fn is_in_range(
    n: &BigUint,
    left_border: (&BigUint, bool),
    right_border: (&BigUint, bool),
) -> bool {
    let (left, left_inclusive) = left_border;
    let (right, right_inclusive) = right_border;

    if left != right {
        let left_condition = match left_inclusive {
            true => { n >= left },
            false => { n > left },
        };

        let right_condition = match right_inclusive {
            true => { n <= right },
            false => { n < right },
        };

        if left < right {
            left_condition && right_condition
        } else {
            left_condition || right_condition
        }
    } else {
        /* If left equals right and neither left nor right is inclusive,
         * then we just validate that the n is not equal to the border.
         * Otherwise just return true, since the borders encompass the entire ring. */
        match !left_inclusive && !right_inclusive {
            true => { n != left },
            false => { true },
        }
    }
}

pub fn compute_identifier(bits: u32, input: &String) -> BigUint {
    let base = BigUint::from_bytes_be(&[2]);
    let divisor = base.pow(bits);
    let hash = hash(input);
    let identifier = hash % divisor;
    identifier
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let src = "207.216.57.167:8820:6".to_string();
        let expected_hash = "73983030965240321521725464828347026369133146436118419434250862939976471883122";
        assert_eq!(format!("{}", hash(&src)), expected_hash);
    }

    #[test]
    fn test_is_in_range_left_smaller_than_right() {
        let left = BigUint::from_bytes_be(&[12, 17, 23]);
        let right = BigUint::from_bytes_be(&[12, 17, 56]);

        let n = BigUint::from_bytes_be(&[12, 17, 42]);
        for left_inclusive in [true, false].iter() {
            for right_inclusive in [true, false].iter() {
                let res = is_in_range(
                    &n,
                    (&left, *left_inclusive),
                    (&right, *right_inclusive),
                );
                assert_eq!(res, true);
            }
        }

        let n = BigUint::from_bytes_be(&[12, 17, 96]);
        for left_inclusive in [true, false].iter() {
            for right_inclusive in [true, false].iter() {
                let res = is_in_range(
                    &n,
                    (&left, *left_inclusive),
                    (&right, *right_inclusive),
                );
                assert_eq!(res, false);
            }
        }

        let n = BigUint::from_bytes_be(&[12, 17, 23]);
        for left_inclusive in [true, false].iter() {
            for right_inclusive in [true, false].iter() {
                let res = is_in_range(
                    &n,
                    (&left, *left_inclusive),
                    (&right, *right_inclusive),
                );
                if *left_inclusive {
                    assert_eq!(res, true);
                } else {
                    assert_eq!(res, false);
                }
            }
        }

        let n = BigUint::from_bytes_be(&[12, 17, 56]);
        for left_inclusive in [true, false].iter() {
            for right_inclusive in [true, false].iter() {
                let res = is_in_range(
                    &n,
                    (&left, *left_inclusive),
                    (&right, *right_inclusive),
                );
                if *right_inclusive {
                    assert_eq!(res, true);
                } else {
                    assert_eq!(res, false);
                }
            }
        }
    }

    #[test]
    fn test_is_in_range_left_larger_than_right() {
        let left = BigUint::from_bytes_be(&[12, 17, 56]);
        let right = BigUint::from_bytes_be(&[12, 17, 23]);

        let n = BigUint::from_bytes_be(&[12, 17, 42]);
        for left_inclusive in [true, false].iter() {
            for right_inclusive in [true, false].iter() {
                let res = is_in_range(
                    &n,
                    (&left, *left_inclusive),
                    (&right, *right_inclusive),
                );
                assert_eq!(res, false);
            }
        }

        let n = BigUint::from_bytes_be(&[12, 17, 96]);
        for left_inclusive in [true, false].iter() {
            for right_inclusive in [true, false].iter() {
                let res = is_in_range(
                    &n,
                    (&left, *left_inclusive),
                    (&right, *right_inclusive),
                );
                assert_eq!(res, true);
            }
        }

        let n = BigUint::from_bytes_be(&[12, 17, 56]);
        for left_inclusive in [true, false].iter() {
            for right_inclusive in [true, false].iter() {
                let res = is_in_range(
                    &n,
                    (&left, *left_inclusive),
                    (&right, *right_inclusive),
                );
                if *left_inclusive {
                    assert_eq!(res, true);
                } else {
                    assert_eq!(res, false);
                }
            }
        }

        let n = BigUint::from_bytes_be(&[12, 17, 23]);
        for left_inclusive in [true, false].iter() {
            for right_inclusive in [true, false].iter() {
                let res = is_in_range(
                    &n,
                    (&left, *left_inclusive),
                    (&right, *right_inclusive),
                );
                if *right_inclusive {
                    assert_eq!(res, true);
                } else {
                    assert_eq!(res, false);
                }
            }
        }
    }

    #[test]
    fn test_is_in_range_left_equal_to_right() {
        let left = BigUint::from_bytes_be(&[12, 17, 42]);
        let right = BigUint::from_bytes_be(&[12, 17, 42]);

        let n = BigUint::from_bytes_be(&[12, 17, 42]);
        for left_inclusive in [true, false].iter() {
            for right_inclusive in [true, false].iter() {
                let res = is_in_range(
                    &n,
                    (&left, *left_inclusive),
                    (&right, *right_inclusive),
                );
                if !left_inclusive && !right_inclusive {
                    assert_eq!(res, false);
                } else {
                    assert_eq!(res, true);
                }
            }
        }

        let n = BigUint::from_bytes_be(&[12, 17, 96]);
        for left_inclusive in [true, false].iter() {
            for right_inclusive in [true, false].iter() {
                let res = is_in_range(
                    &n,
                    (&left, *left_inclusive),
                    (&right, *right_inclusive),
                );
                assert_eq!(res, true);
            }
        }

        let n = BigUint::from_bytes_be(&[12, 17, 23]);
        for left_inclusive in [true, false].iter() {
            for right_inclusive in [true, false].iter() {
                let res = is_in_range(
                    &n,
                    (&left, *left_inclusive),
                    (&right, *right_inclusive),
                );
                assert_eq!(res, true);
            }
        }
    }
}