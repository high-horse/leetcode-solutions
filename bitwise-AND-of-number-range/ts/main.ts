function rangeBitwiseAnd(left: number, right: number): number {
    while (left < right) {
        right &= right-1
    }
    return right
};

console.log(rangeBitwiseAnd(5,7))

console.log(rangeBitwiseAnd(0, 0))

console.log(rangeBitwiseAnd(1, 2147483647))