function firstMissingPositive(nums: number[]): number {
    let n = nums.length;
    
    for (let i = 0; i < n; i++) {
        if (nums[i] > 0 && nums[i] <= n && nums[i] !== nums[nums[i] - 1]) {
            nums[i], nums[nums[i] - 1] = nums[nums[i] - 1], nums[i];
            i--;
        }
    }
    
    for (let i = 0; i < n; i++) {
        if (nums[i] !== i + 1) {
            return i + 1;
        }
    }
    
    return n + 1;
}
