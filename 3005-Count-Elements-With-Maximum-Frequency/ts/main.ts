function maxFrequencyElements(nums: number[]): number {
    // Create a map to store frequency of elements
    const freqMap: Map<number, number> = new Map();

    // Populate frequency map
    for (const val of nums) {
        freqMap.set(val, (freqMap.get(val) || 0) + 1);
    }

    // Find maximum frequency
    let max = -1;
    for (const freq of freqMap.values()) {
        max = Math.max(max, freq);
    }

    // Calculate count of elements with maximum frequency
    let count = 0;
    for (const freq of freqMap.values()) {
        if (freq === max) {
            count += max;
        }
    }

    return count;
}

// Example usage:
const nums: number[] = [1, 2, 2, 3, 3, 3];
console.log(maxFrequencyElements(nums));
