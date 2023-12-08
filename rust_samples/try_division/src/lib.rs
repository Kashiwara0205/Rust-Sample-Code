fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

pub fn try_division(dividend: i32, divisor: i32) -> i32{
    match checked_division(dividend, divisor) {
        None => 0,
        Some(quotient) => {
            quotient
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_division_works() {
        assert_eq!(try_division(2, 2), 1);
        assert_eq!(try_division(0, 2), 0);
    }

    #[test]
    fn checked_division_works() {
        let result = checked_division(2, 2);
        assert_eq!(result, Some(1i32));
        assert_eq!(result.unwrap(), 1);
    }
}
