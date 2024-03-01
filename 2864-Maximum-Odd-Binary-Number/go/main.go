func maximumOddBinaryNumber(s string) string {
    sthStr := s
    lenS := len(sthStr)
    var zeros []string
    var ones []string

    for _, char := range sthStr {
        if char == '1' {
            ones = append(ones, "1")
        } else {
            zeros = append(zeros, "0")
        }
    }

    switch len(ones) {
    case 0:
        return strings.Repeat("0", lenS)
    case 1:
        return strings.Repeat("0", lenS-1) + "1"
    default:
        ones = ones[:len(ones)-1]
        return strings.Join(ones, "") + strings.Join(zeros, "") + "1"
    }
}
