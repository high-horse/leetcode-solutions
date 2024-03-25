function findDuplicates(nums: number[]): number[] {
    const duplicates: number[] = [];
    nums.forEach((num) => {
        const index = Math.abs(num) - 1;
        if (nums[index] < 0) {
            duplicates.push(Math.abs(num));
        } else {
            nums[index] = -nums[index];
        }
    });
    return duplicates;
}
