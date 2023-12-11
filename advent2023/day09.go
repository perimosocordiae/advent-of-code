// Usage: go run day09.go
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	filePath := "inputs/09.full"
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("Failed to open file: %v\n", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	part1 := int64(0)
	part2 := int64(0)
	for scanner.Scan() {
		array := parseArray(scanner.Text())
		part1 += rec1(array)
		part2 += rec2(array)
	}

	if err := scanner.Err(); err != nil {
		fmt.Printf("Error reading file: %v\n", err)
	}

	fmt.Printf("Part 1: %v\n", part1)
	fmt.Printf("Part 2: %v\n", part2)
}

func rec1(array []int64) int64 {
	if isAllZero(array) {
		return 0
	}
	return rec1(difference(array)) + array[len(array)-1]
}

func rec2(array []int64) int64 {
	if isAllZero(array) {
		return 0
	}
	return array[0] - rec2(difference(array))
}

func isAllZero(array []int64) bool {
	for _, n := range array {
		if n != 0 {
			return false
		}
	}
	return true
}

func difference(array []int64) []int64 {
	diffs := make([]int64, len(array)-1)
	for i := 0; i < len(array)-1; i++ {
		diffs[i] = array[i+1] - array[i]
	}
	return diffs
}

func parseArray(line string) []int64 {
	numbers := strings.Split(line, " ")
	var nums []int64
	for _, n := range numbers {
		num, err := strconv.ParseInt(n, 10, 64)
		if err != nil {
			fmt.Printf("Failed to convert string to int64: %v\n", err)
			continue
		}
		nums = append(nums, num)
	}
	return nums
}
