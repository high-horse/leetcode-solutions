package main() 


func main() {
	println(removeKdigits("12333", 3))
}


func removeKdigits(num string, k int) string {
    var result []byte
	var removed int

	for i := 0; i < len(num); i++ {
		value := num[i]

		for removed < k && len(result) > 0 && result[len(result)-1] > value {
			result = result[:len(result)-1]
			removed++
		}
		result = append(result, value)
	}

	// If there are still remaining digits to be removed
	for removed < k {
		result = result[:len(result)-1]
		removed++
	}

	// Skip leading zeros
	leadingZeros := 0
	for _, c := range result {
		if c == '0' {
			leadingZeros++
		} else {
			break
		}
	}

	finalResult := string(result[leadingZeros:])
    if finalResult == "" {
        return "0"
    }
	return finalResult
}
