function findJudge(n: number, trust: number[][]): number {
    let trustCounts = new Array(n + 1).fill(0);

    trust.forEach(([truster, trustee]) => {
        trustCounts[truster]--;
        trustCounts[trustee]++;
    });

    for (let i = 1; i <= n; i++) {
        if (trustCounts[i] === n - 1) {
            return i;
        }
    }

    return -1;
}

function main() {
    let trust  = [[1,2]];
    let n = 2;

    let answer = findJudge(n, trust);
    console.log(answer);
}

main();