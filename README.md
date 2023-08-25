# Two Sum

## Intuition

Cycle through the list of numbers and adding them with their index as a pair to a collection if their complement with the target number is not already present in the collection. The iteration stops automatically when a number whose compliment exists is found, since the addition is solved

## Approach

Use HashMap for collection storing the number and their index together.

Iterate through each one and then break out from the for loop once the complement is found in the collection.

Return the indices

## Complexity

- Time complexity:
Since we are carrying out only one for loop for the entire array or for searching in the collection, the time complexity becomes $$0(n)$$

- Space complexity:
The space complexity is $$O(2n)$$ since we are need two bytes per number (one for the index additionally) in the collection

## Code

```Rust
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
```
