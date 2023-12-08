pub fn add_two_numbers(list_node: Vec<i32>, list_node2: Vec<i32>) -> Vec<i32> {
    let val1 = string_to_i32(&vec_i32_to_string(reverse_vec_i32(list_node)));
    let val2 = string_to_i32(&vec_i32_to_string(reverse_vec_i32(list_node2)));
    let sum = val1 + val2;
    
    string_to_vec_i32(&reverse_string(&i32_to_string(sum)))
}

fn reverse_vec_i32(vec: Vec<i32>)  -> Vec<i32> {
    vec.iter().copied().rev().collect()
}

fn reverse_string(value:  &str)  -> String {
    value.chars().rev().collect::<String>()
}

fn vec_i32_to_string(vec: Vec<i32>) -> String{
    vec.into_iter().map(|i| i.to_string()).collect::<String>()
}

fn string_to_vec_i32(value: &str) -> Vec<i32>{
    value.chars().map(|i| i as i32 - 48).collect::<Vec<i32>>()
}

fn string_to_i32(value: &str) -> i32{
    value.parse().unwrap()
}

fn i32_to_string(value: i32) -> String{
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_numbers_works() {
        assert_eq!(
          add_two_numbers(vec![2, 4, 3], vec![5, 6, 4]), 
          vec![7, 0, 8]
        );

        assert_eq!(
          add_two_numbers(vec![0], vec![0]), 
          vec![0]
        );

        assert_eq!(
          add_two_numbers(vec![9, 9, 9, 9, 9, 9, 9], vec![9, 9, 9, 9]), 
          vec![8, 9, 9, 9, 0, 0, 0, 1]
        );
    }

    #[test]
    fn reverse_vec_i32_works() {
        let result = reverse_vec_i32(vec![1, 2, 3]);
        assert_eq!(result, [3, 2, 1]);
    }

    #[test]
    fn reverse_string_works() {
        let result = reverse_string("123");
        assert_eq!(result, "321");
    }


    #[test]
    fn vec_i32_to_string_works() {
        let result = vec_i32_to_string(vec![1, 2, 3]);
        assert_eq!(result, "123");
    }

    #[test]
    fn string_to_vec_i32_works() {
        let result = string_to_vec_i32("123");
        assert_eq!(result, [1, 2, 3]);
    }


    #[test]
    fn string_to_i32_works() {
        let result = string_to_i32("123");
        assert_eq!(result, 123);
    }

    #[test]
    fn i32_to_string_works() {
        let result = i32_to_string(100);
        assert_eq!(result, "100");
    }
}
