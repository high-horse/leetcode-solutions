function firstMissingPositive(nums: number[]): number {
    let n = nums.length;
    
    for (let i = 0; i < n; i++) {
        while (nums[i] > 0 && nums[i] <= n && nums[nums[i] - 1] !== nums[i]) {
            let temp = nums[i];
            nums[i] = nums[nums[i] - 1];
            nums[temp - 1] = temp;
        }
    }
    
    for (let i = 0; i < n; i++) {
        if (nums[i] !== i + 1) {
            return i + 1;
        }
    }
    
    return n + 1;
}

