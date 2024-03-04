function bagOfTokensScore(tokens: number[], power: number): number {
    tokens.sort((a, b) => a - b);

    let left = 0;
    let right = tokens.length - 1;

    let score = 0;
    let max_score = 0;

    while (left <= right) {
        if (tokens[left] <= power) {
            power -= tokens[left];
            left++;
            score++;
            max_score = Math.max(max_score, score);
        } else if (score > 0) {
            power += tokens[right];
            right--;
            score--;
        } else {
            break;
        }
    }

    return max_score;
}
