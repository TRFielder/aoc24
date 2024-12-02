import { readFileSync } from "fs"

const input = readFileSync("../inputs/day2.txt", "utf-8")

const lines = input.split("\r\n")

const isSafe = (report: number[]): boolean => {
    const isDecreasing = report.every((number, index) => {
        // it's decreasing if number is > the number after it
        if (index < report.length - 1) return number > report[index + 1]
        // if we're on the last element there is no index+1 so we just return true here
        return true
    })

    const isIncreasing = report.every((number, index) => {
        // it's increasing if number is > the number after it
        if (index < report.length - 1) return number < report[index + 1]
        return true
    })

    const differencesInBounds = report.every((number, index) => {
        // differences need to be between 1 and 3, inclusive
        if (index < report.length - 1)
            return (
                Math.abs(number - report[index + 1]) >= 1 &&
                Math.abs(number - report[index + 1]) <= 3
            )
        return true
    })
    console.log(isDecreasing, isIncreasing, differencesInBounds)

    return (isDecreasing || isIncreasing) && differencesInBounds
}

export const part1 = (reports: string[]): number => {
    let safe: number = 0

    reports.forEach((report) => {
        const levels = report.split(/\s+/).map(Number)

        if (isSafe(levels)) safe++
    })

    return safe
}

const part1Result = part1(lines)

console.log("The result for day 2 part 1 is ", part1Result)
