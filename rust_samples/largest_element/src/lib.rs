pub fn largest<T>(list: &[T]) -> &T 
where T: std::cmp::PartialOrd,
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_int_works() {
        let test_data = vec![1, 2, 3];
        let result = largest(&test_data);
        assert_eq!(*result, 3);
    }

    #[test]
    fn largest_char_works() {
        let test_data = vec!['a', 'b', 'c'];
        let result = largest(&test_data);
        assert_eq!(*result, 'c');
    }
}
