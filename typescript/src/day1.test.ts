import { expect, test } from "vitest"
import { readFileSync } from "fs"
import { parseInputToArrays, part1 } from "./day1"

test("Day 1 Part 1", () => {
    const input = readFileSync("../testInputs/day1.txt", "utf-8")

    const [first, second] = parseInputToArrays(input)

    const result = part1(first, second)

    expect(result).toEqual(11)
})

test("Day 1 Part 2", () => {})
