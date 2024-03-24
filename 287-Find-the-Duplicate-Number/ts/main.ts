function findDuplicate(nums: number[]): number {
    let hasSeenNumber: number[] = new Array(nums.length).fill(0);
    for (let num of nums) {
        if (hasSeenNumber[num] === -1) {
            return num;
        }
        hasSeenNumber[num] = -1;
    }
    return -1;
}


