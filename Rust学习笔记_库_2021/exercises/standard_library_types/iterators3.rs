#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZero)
    } else if a % b == 0 {
        Ok(a/b)
    } else {
        Err(DivisionError::NotDivisible(NotDivisibleError{dividend: a,divisor: b}))
    }
}

fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    let x: Result<Vec<i32>, DivisionError> = Ok(division_results.collect::<Vec<_>>().into_iter().map(|x| x.unwrap_or(-1)).collect::<Vec<_>>());
    x
}

fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27)).collect::<Vec<Result<i32, DivisionError>>>();
    division_results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}