pub fn insertionsort_dsa_rs(nums: &mut [i32]) {
    for i in 1..nums.len() {
        let current_cmp_val = nums[i];
        let mut j = i;

        while j > 0 && current_cmp_val < nums[j - 1] {
            nums[j] = nums[j - 1];
            j -= 1;
        }

        nums[j] = current_cmp_val;
    }
}
