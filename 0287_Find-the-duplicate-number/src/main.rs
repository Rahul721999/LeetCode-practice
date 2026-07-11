mod tests;
fn main() {
    let nums = vec![1,3,4,2,2];
    let output = Solution::find_duplicate(nums);
    println!("{}", output);
}
pub struct Solution;
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        let len = nums.len();
        for i in 0..len{
            let value = nums[i].abs();
            let target_idx = value as usize - 1;
            if nums[target_idx] < 0{
                return value
            }else{
                nums[target_idx] *= -1;
            }
        }
        0
    }
}