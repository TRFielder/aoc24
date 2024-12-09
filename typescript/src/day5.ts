import { readFileSync } from "fs"

const input = readFileSync("../inputs/day5.txt", "utf-8")

export const part1 = (input: string) => {
    const sections: string[] = input.split("\r\n\r\n")

    // collect rules into vector of tuples
    const rules: [number, number][] = sections[0].split("\n").map((line) => {
        const parts = line.split("|").map((part) => parseInt(part))

        return [parts[0], parts[1]]
    })

    // and the updates into a vector of vectors of numbers
    const updates: number[][] = sections[1].split("\r\n").map((line) => {
        return line.split(",").map((num) => parseInt(num))
    })

    // Iterate over each update, see if the rules have been obeyed by iterating over the vector of rules for each
    // For the ones that obey the ordering rules, add their middle value to result
    let result: number = 0

    updates.forEach((update) => {
        let isInOrder: boolean = true

        for (const [before, after] of rules) {
            const beforeIndex = update.findIndex((num) => num === before)
            const afterIndex = update.findIndex((num) => num === after)

            // Only check if we found indices for both numbers
            if (beforeIndex !== -1 && afterIndex !== -1) {
                if (beforeIndex > afterIndex) {
                    isInOrder = false
                    break
                }
            }
        }

        if (isInOrder) {
            const middleIndex = Math.floor(update.length / 2)
            result += update[middleIndex]
        }
    })
    return result
}

const part1Result = part1(input)

console.log("The result for Day 4 part 1 is: ", part1Result)
