def remove_k_digits(num: str, k: int) -> str:
    result = []
    removed = 0

    for value in num:
        while removed < k and result and result[-1] > value:
            result.pop()
            removed += 1
        result.append(value)

    # If there are still remaining digits to be removed
    while removed < k:
        result.pop()
        removed += 1

    # Skip leading zeros
    leading_zeros = 0
    for c in result:
        if c == '0':
            leading_zeros += 1
        else:
            break

    final_result = ''.join(result[leading_zeros:])
    if final_result == "" :
	return "0"
    return final_result

def main():
    print("Sss")
    number = "1234"
    result = remove_k_digits(number, 2)
    print(result)

if __name__ == "__main__":
    main()
