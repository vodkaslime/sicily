use num::bigint::BigUint;
use sha2::{ Sha256, Digest };

pub fn hash(input: &String) -> BigUint {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let arr = hasher.finalize();
    BigUint::from_bytes_be(&arr)
}

pub fn is_in_range(
    n: &BigUint,
    left_border: (&BigUint, bool),
    right_border: (&BigUint, bool)
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
        if left_inclusive && right_inclusive && n == left {
            return true
        } else {
            return false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::bigint::BigUint;

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
                if *left_inclusive && *right_inclusive {
                    assert_eq!(res, true);
                } else {
                    assert_eq!(res, false);
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
                assert_eq!(res, false);
            }
        }
    }
}