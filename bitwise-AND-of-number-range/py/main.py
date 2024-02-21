def rangeBitwiseAnd(self, left: int, right: int) -> int:
    while left < right :
        right &= right-1
        
    return right


print(rangeBitwiseAnd(5,7))
print(rangeBitwiseAnd(0, 0))
print(rangeBitwiseAnd(1, 2147483647))

