def minimumLength(self, s: str) -> int:
    left = 0
    right = len(s) - 1

    while left < right :
        if s[left] == s[right] :
            same = s[left]
            while (left<=right and s[left] == same) :
                left += 1
            while left<=right and s[right] == same :
                right -= 1
        else :
            break
            
    return (right-left +1)  

def main() {
    print(minimumLength("aabccabba"))
    print(minimumLength("cabaabac"))
}

main()
