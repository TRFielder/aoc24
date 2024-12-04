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

const part1Result = part1(input)

console.log("The result for Day 4 part 1 is: ", part1Result)
