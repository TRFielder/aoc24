import { expect, test } from "vitest"
import { readFileSync } from "fs"
import { part1, part2 } from "./day4"

test("Day 4 Part 1", () => {
    const input = readFileSync("../testInputs/day4.txt", "utf-8")

    const result = part1(input)

    expect(result).toEqual(18)
})

test("Day 4 Part 2", () => {
    const input = readFileSync("../testInputs/day4.txt", "utf-8")

    const result = part2(input)

    expect(result).toEqual(9)
})
