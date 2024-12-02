package main

import (
	"advent-2024/utils"
	"fmt"
	"sort"
	"strconv"
	"strings"
	"time"
)

func main() {
	// Grab file contents
	input := utils.ReadFile("in.dat")

	{
		// Part 1
		start := time.Now()
		fmt.Printf("Part 1: %d  | Time elapsed: %s\n", part1(input), time.Since(start))
	}

	{
		// Part 2
		start := time.Now()
		fmt.Printf("Part 2: %d | Time elapsed: %s\n", part2(input), time.Since(start))
	}

}

func part2(input string) int {
	// Iterate over each line
	lines := strings.Split(input, "\n")

	leftValues, rightValues := parseInput(lines)

	var counter = make(map[int]int)

	for _, number := range rightValues {
		counter[number] += 1
	}

	var product = []int{}
	for _, number := range leftValues {
		product = append(product, int(number)*counter[number])
	}

	var sum int
	for _, number := range product {
		sum += number
	}
	return sum
}

func parseInput(lines []string) ([]int, []int) {
	leftValues := []int{}
	rightValues := []int{}
	for _, line := range lines {
		values := strings.Fields(line)
		if len(values) > 0 {

			leftNumber, _ := strconv.ParseInt(values[0], 10, 64)
			leftValues = append(leftValues, int(leftNumber))

			rightNumber, _ := strconv.ParseInt(values[1], 10, 64)
			rightValues = append(rightValues, int(rightNumber))
		}
	}
	return leftValues, rightValues
}

func part1(input string) int {
	// Iterate over each line
	lines := strings.Split(input, "\n")

	leftValues, rightValues := parseInput(lines)

	// Sort both arrays
	sort.Slice(leftValues, func(i, j int) bool {
		return leftValues[i] < leftValues[j]
	})

	sort.Slice(rightValues, func(i, j int) bool {
		return rightValues[i] < rightValues[j]
	})

	// For each pair determine the distance between each number
	count := 0
	for i := range leftValues {
		// Add up all these numbers
		count += absDiff(int(leftValues[i]), int(rightValues[i]))
	}

	return count
}

func absDiff(x, y int) int {
	if x < y {
		return y - x
	}
	return x - y
}
