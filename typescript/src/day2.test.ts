import { expect, test } from "vitest"
import { readFileSync } from "fs"
import { part1, part2 } from "./day2"

test("Day 2 Part 1", () => {
    const input = readFileSync("../testInputs/day2.txt", "utf-8")

    const lines = input.split("\r\n")

    const result = part1(lines)

    expect(result).toEqual(2)
})

test("Day 2 Part 2", () => {
    const input = readFileSync("../testInputs/day2.txt", "utf-8")

    const lines = input.split("\r\n")

    const result = part2(lines)

    expect(result).toEqual(4)
})
