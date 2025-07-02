package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Direction int

const (
	North Direction = iota
	East
	South
	West
)

func (d Direction) String() string {
	directions := []string{"north", "east", "south", "west"}
	return directions[d]
}

type Position struct {
	X, Y int
}

func (p Position) ManhattanDistance() int {
	return abs(p.X) + abs(p.Y)
}

func abs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}

func main() {
	fileBytes, err := os.ReadFile("puzzle.txt")
	if err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		return
	}

	instructions := strings.Split(strings.TrimSpace(string(fileBytes)), ", ")

	pos := Position{0, 0}
	facing := North

	fmt.Printf("Starting at position: %d %d\n", pos.X, pos.Y)

	for _, instruction := range instructions {
		if len(instruction) < 2 {
			fmt.Printf("Invalid instruction: %s\n", instruction)
			continue
		}

		turn := instruction[0]
		distanceStr := instruction[1:]

		distance, err := strconv.Atoi(distanceStr)
		if err != nil {
			fmt.Printf("Error parsing distance '%s': %v\n", distanceStr, err)
			continue
		}

		// Handle turning
		switch turn {
		case 'R':
			facing = (facing + 1) % 4
		case 'L':
			facing = (facing + 3) % 4 // Same as (facing - 1 + 4) % 4
		default:
			fmt.Printf("Unknown turn direction: %c\n", turn)
			continue
		}

		// Move in the current direction
		switch facing {
		case North:
			pos.Y += distance
		case East:
			pos.X += distance
		case South:
			pos.Y -= distance
		case West:
			pos.X -= distance
		}

		fmt.Printf("%c\n", turn)
		fmt.Printf("Now facing %s moved %d\n", facing, distance)
		fmt.Printf("Current position: %d %d\n", pos.X, pos.Y)
	}

	fmt.Printf("Final position: %d %d\n", pos.X, pos.Y)
	fmt.Printf("Final distance: %d\n", pos.ManhattanDistance())
}
