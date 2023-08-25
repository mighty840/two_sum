#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::<i32, i32>::new();
        for (index, number) in nums.iter().enumerate() {
            if let Some(diff_index) = map.get(&(target - *number)) {
                return vec![*diff_index, index as i32];
            }
            map.insert(nums[index], index as i32);
        }
        vec![]
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_len_2() {
        // Arrange
        let nums = vec![-1, -2];
        let target = -3;
        // Act
        let two_sum = Solution::two_sum(nums, target);
        // Assert
        assert_eq!(two_sum, [0, 1])
    }
    #[test]
    fn test_len_3() {
        // Arrange
        let nums = vec![3, 4, 5];
        let target = 8;
        // Act
        let two_sum = Solution::two_sum(nums, target);
        // Assert
        assert_eq!(two_sum, [0, 2])
    }
    #[test]
    fn test_len_4() {
        // Arrange
        let nums = vec![-10, 7, 19, 15];
        let target = 9;
        // Act
        let two_sum = Solution::two_sum(nums, target);
        // Assert
        assert_eq!(two_sum, [0, 2])
    }
    #[test]
    fn test_len_5() {
        // Arrange
        let nums = vec![3, 4, 5, 9, 11];
        let target = 9;
        // Act
        let two_sum = Solution::two_sum(nums, target);
        // Assert
        assert_eq!(two_sum, [1, 2])
    }
}
