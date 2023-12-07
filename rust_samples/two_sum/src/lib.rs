pub fn two_sum(nums: &[i32], target: i32) -> [usize; 2] {
    let length = nums.len();
    for i in 0..length {
        for j in (i+1)..length {
            if nums[i] + nums[j] == target{
                return [i, j]
            }
        }
    }
    return [0, 0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = two_sum(&[1, 2, 3], 5);
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 2);

        result = two_sum(&[3,2,4], 6);
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 2);

        result = two_sum(&[3, 3], 6);
        assert_eq!(result[0], 0);
        assert_eq!(result[1], 1);
    }
}
