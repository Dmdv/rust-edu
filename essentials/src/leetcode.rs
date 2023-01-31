impl Solution1 {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut pre_num = 0;
        nums.into_iter().map(|num| {
            pre_num += num;
            pre_num
        }).collect::<Vec<i32>>()
    }
}

impl Solution2 {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut res:i32 = 0;
        let mut sums = Vec::new();
        for i in nums.iter() {
            res += i;
            sums.push(res);
        }
        return sums;
    }
}