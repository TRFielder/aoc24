import { expect, test } from "vitest"
import { readFileSync } from "fs"
import { part1 } from "./day5"

test("Day 5 Part 1", () => {
    const input = readFileSync("../testInputs/day5.txt", "utf-8")

    const result = part1(input)

    expect(result).toEqual(143)
})

// test("Day 5 Part 2", () => {
//     const input = readFileSync("../testInputs/day4.txt", "utf-8")

//     const result = part2(input)

//     expect(result).toEqual(9)
// })
