import { expect, test } from "vitest"
import { readFileSync } from "fs"
import { parseInputToArrays, part1, part2 } from "./day1"

test("Day 1 Part 1", () => {
    const input = readFileSync("../testInputs/day1.txt", "utf-8")

    const [first, second] = parseInputToArrays(input)

    const result = part1(first, second)

    expect(result).toEqual(11)
})

test("Day 1 Part 2", () => {
    const input = readFileSync("../testInputs/day1.txt", "utf-8")

    const [first, second] = parseInputToArrays(input)

    const result = part2(first, second)

    expect(result).toEqual(31)
})
