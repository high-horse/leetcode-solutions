package main
import "fmt"

func main() {
	var result_1 int = rangeBitwiseAnd(5, 7);
	fmt.Println(result_1);

	var result_2 int = rangeBitwiseAnd(0, 0);
	fmt.Println(result_2);

	var result_3 int = rangeBitwiseAnd(1, 2147483647);
	fmt.Println(result_3);

}

func rangeBitwiseAnd(left int, right int) int {
    for (left < right) {
        right &= right-1;
    }
    return right;
}