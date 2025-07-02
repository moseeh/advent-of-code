package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	filebytes, _ := os.ReadFile("puzzle.txt")
	filearr := strings.Split(strings.Trim(string(filebytes), "\n"), ", ")

	horizontal, vertical := 0, 0

	directions := []int{1, 1, -1, -1} // north, east, south , west
	facing := []string{"north", "east", "south", "west"}

	index := 0

	for i, v := range filearr {
		
		direction := string(v[0])
		num, err := strconv.Atoi(v[1:])
		if err != nil {
			fmt.Println(err)
		}
		switch direction {
		case "R":
			index++
			if index == 4 {
				index = 0
			}
		case "L":
			index--
			if index == -1 {
				index = 3
			}
		}

		if i%2 == 0 {
			horizontal += directions[index] * num
		} else {
			vertical += directions[index] * num
		}
		fmt.Println(direction)
		fmt.Println(facing[index], directions[index], num)
		fmt.Println("current position:", horizontal, vertical)
	}
	fmt.Println(horizontal, vertical)
	fmt.Println(horizontal + vertical)
}
