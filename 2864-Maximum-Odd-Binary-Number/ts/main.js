/**
 * @param {string} s
 * @return {string}
 */
var maximumOddBinaryNumber = function(s) {
    let sth_str = s;
    let len = sth_str.length;
    let zeros = [];
    let ones = [];

    for (let i = 0; i < sth_str.length; i++) {
        if (sth_str[i] == '1') {
            ones.push('1');
        } else {
            zeros.push('0');
        }
    }

    let res;
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
};
