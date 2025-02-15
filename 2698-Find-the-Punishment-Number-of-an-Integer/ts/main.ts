function punishmentNumber(n: number): number {
    let answer = 0;

    // Loop through all numbers from 1 to n
    for (let i = 1; i <= n; i++) {
        answer += calculatePunishment(i);
    }
    return answer;
}

// Helper function to calculate the punishment for each number
function calculatePunishment(n: number): number {
    const sq = n * n;
    const sqStr = sq.toString();

    const partitions = generatePartitions(sqStr);

    // Check each partition to see if its sum matches n
    for (const partition of partitions) {
        let sum = 0;
        for (const part of partition) {
            const num = parseInt(part, 10);
            if (!isNaN(num)) {
                sum += num;
            }
        }

        // If the sum matches n, return the square
        if (sum === n) {
            return sq;
        }
    }

    return 0;
}

// Helper function to generate all partitions of a string
function generatePartitions(s: string): string[][] {
    const result: string[][] = [];
    const partition: string[] = [];
    helper(s, 0, partition, result);
    return result;
}

// Recursive helper function to backtrack and generate partitions
function helper(s: string, start: number, partition: string[], result: string[][]): void {
    if (start === s.length) {
        result.push([...partition]);
        return;
    }

    // Loop to generate all partitions
    for (let i = start + 1; i <= s.length; i++) {
        const substr = s.substring(start, i);
        partition.push(substr);
        helper(s, i, partition, result);
        partition.pop(); // Backtrack
    }
}
