function removeKDigits(num: string, k: number): string {
    const result: string[] = [];
    let removed: number = 0;

    for (let i = 0; i < num.length; i++) {
        const value = num[i];

        while (removed < k && result.length > 0 && result[result.length - 1] > value) {
            result.pop();
            removed++;
        }
        result.push(value);
    }

    // If there are still remaining digits to be removed
    while (removed < k) {
        result.pop();
        removed++;
    }

    // Skip leading zeros
    let leadingZeros = 0;
    for (const c of result) {
        if (c === '0') {
            leadingZeros++;
        } else {
            break;
        }
    }

    const finalResult = result.slice(leadingZeros).join('');
    if(finalResult == ""){
	return "0";
    }
    return finalResult;
}

function main() {
    console.log("Sss");
    const number = "1234";
    const result = removeKDigits(number, 2);
    console.log(result);
}

main();
