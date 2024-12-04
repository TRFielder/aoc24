import { readFileSync } from "fs"

const input = readFileSync("../inputs/day4.txt", "utf-8")

// directional steps are horizontal, vertical and diagonal
const DIRECTIONS: [number, number][] = [
    // Horizontal
    [1, 0],
    [-1, 0],
    // Vertical
    [0, 1],
    [0, -1],
    // Diagonal
    [1, 1],
    [1, -1],
    [-1, -1],
    [-1, 1],
]

export const isNotExpectedLetter = (
    grid: string[][],
    expectedLetter: string,
    col: number,
    row: number
) => {
    // This is lazy. I don't want to deal with it if we go outside the bounds of the grid...
    try {
        return grid[row][col] !== expectedLetter
    } catch {
        return true
    }
}

export const part1 = (input: string) => {
    const grid: string[][] = input.split("\r\n").map((line) => line.split(""))

    const wordToFind: string = "XMAS"

    let result: number = 0

    // Iterate through every item in the grid
    for (let row = 0; row < grid.length; row++) {
        for (let col = 0; col < grid[row].length; col++) {
            DIRECTIONS.forEach((direction) => {
                let foundXmas: boolean = true

                for (let index = 0; index < wordToFind.length; index++) {
                    if (
                        isNotExpectedLetter(
                            grid,
                            wordToFind[index],
                            // if X is at index 1, and it's horizontal forwards, we expect
                            // That M (index 1 in the word) will be at (0, 1) from where we are
                            // A (index 2) at (0, 2) from where we are or 2 x the direction value
                            // and S (index 3) at (0, 3) or 3x direction values
                            // This applies diagonally, backwards, wherever we like
                            col + index * direction[0],
                            row + index * direction[1]
                        )
                    ) {
                        // Stop checking letters in the word if we don't find the letter we expect
                        foundXmas = false
                        break
                    }
                }

                if (foundXmas) result++
            })
        }
    }

    return result
}

export const part2 = (input: string) => {
    const grid: string[][] = input.split("\r\n").map((line) => line.split(""))

    let result: number = 0

    // Iterate through the grid again, but the directions are different (only diagonals) and we just want "MAS"
    // We need to start by finding A, and then M and S are either side of it
    for (let row = 0; row < grid.length; row++) {
        for (let col = 0; col < grid[row].length; col++) {
            if (grid[row][col] === "A") {
                // Lazy try block again in case we go out of bounds
                try {
                    // Form a set of the letters before and after the "A" in both diagonal direcitons
                    // We use a set because a set is made up of unique values, so if S and M are present
                    // we know that it's a legit MAS in that direction
                    const forward = new Set([
                        grid[row - 1][col - 1],
                        grid[row + 1][col + 1],
                    ])
                    const backward = new Set([
                        grid[row - 1][col + 1],
                        grid[row + 1][col - 1],
                    ])
                    if (
                        // We care if each set has an S and an M. 
                        // Because there are two elements in each set, we just need to check for each letter
                        forward.has("S") &&
                        forward.has("M") &&
                        backward.has("S") &&
                        backward.has("M")
                    ) {
                        result++
                    }
                } catch {
                    break
                }
            }
        }
    }
    return result
}

const part2Result = part2(input)

const part1Result = part1(input)

console.log("The result for Day 4 part 1 is: ", part1Result)
console.log("The result for Day 4 part 2 is: ", part2Result)
