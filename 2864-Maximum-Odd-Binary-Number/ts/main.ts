function maximumOddBinaryNumber(s: string): string {
    const sth_str = s;
    const len = sth_str.length;
    const zeros: string[] = [];
    const ones: string[] = [];

    for (let i = 0; i < sth_str.length; i++) {
        if (sth_str[i] === '1') {
            ones.push('1');
        } else {
            zeros.push('0');
        }
    }

    let res: string;
    switch (ones.length) {
        case 0:
            res = new Array(len).fill("0").join('');
            return res;
        case 1:
            res = new Array(len - 1).fill("0").join('') + "1";
            return res;
        default:
            ones.pop();
            res = ones.join('') + zeros.join('') + "1";
            return res;
    }
}
