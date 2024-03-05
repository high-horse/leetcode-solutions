function minimumLength(s: string): number {
    let left = 0;
    let right = s.length -1 ;

    while (left < right) {
        if( s[left] == s[right]) {
            let same = s[left];
            while (left<=right && s[left] == same ) {
                left ++;
            }
            while (left<=right && s[right] == same ) {
                right --;
            }
        } else {
            break;
        }
    }

    return (right-left +1)
};



function main() {
    console.log(minimumLength("cabaabac"))
}

main()
