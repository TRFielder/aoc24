import { readFileSync } from "fs"

const input = readFileSync("../inputs/day1.txt", "utf-8")

export const parseInputToArrays = (input: string): number[][] => {
    const lines = input.split("\r\n")

    const firstArr: number[] = []
    const secondArr: number[] = []

    lines.forEach((line) => {
        // Split the line at whitespace to get the two values
        const [first, second] = line.split(/\s+/)
        // Push them to the output arrays
        firstArr.push(Number(first))
        secondArr.push(Number(second))
    })

    return [firstArr, secondArr]
}

export const part1 = (first: number[], second: number[]): number => {
    // Sort both arrays
    first.sort((a, b) => a - b)
    second.sort((a, b) => a - b)

    const output = first.reduce((acc, val, index) => {
        return (acc += Math.abs(val - second[index]))
    }, 0)

    return output
}

const [first, second] = parseInputToArrays(input)

const part1Result = part1(first, second)

console.log("The result for day 1 part 1 is ", part1Result)
