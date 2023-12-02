package day05

import java.util.ArrayDeque
import kotlin.math.ceil

typealias Stacks = Array<ArrayDeque<Char>>

data class MoveInstruction(val quantity: Int, val from: Int, val to: Int)

fun parseMove(s: String): MoveInstruction {
    val parts = s.split(" ")

    return MoveInstruction(parts[1].toInt(), parts[3].toInt(), parts[5].toInt())
}

fun initStacks(asStrings: List<String>): Stacks {
    val numStacks = ceil(asStrings[0].length / 4.0).toInt()
    val stacks: Stacks = Array(numStacks) { ArrayDeque<Char>() }

    for(column in 0 until numStacks) {
        for(line in asStrings.size - 2 downTo 0) {
            val crate = asStrings[line][1 + (4 * column)]
            if (crate.isWhitespace())
                break

            stacks[column].push(crate)
        }
    }

    return stacks
}

fun move(stacks: Stacks, move: MoveInstruction) {
    repeat(move.quantity) {
        val crate = stacks[move.from - 1].pop()
        stacks[move.to - 1].push(crate)
    }
}

fun movePart2(stacks: Stacks, move: MoveInstruction) {
    val holdingColumn = ArrayList<Char>()
    repeat(move.quantity) {
        val crate = stacks[move.from - 1].pop()
        holdingColumn.add(crate)

    }

    repeat(move.quantity) {
        stacks[move.to - 1].push(holdingColumn.removeAt(holdingColumn.size - 1))
    }
}

fun part1(input: String):String {
    val crates = input.split("\n\n")[0].split("\n")
    val movesAsStrings = input.split("\n\n")[1]

    val stacks = initStacks(crates)

    movesAsStrings
        .split("\n")
        .map { parseMove(it) }
        .forEach { move(stacks, it)}

        //.forEach { println(it) }

    val topCrates = stacks.map { it.peek() }
        .map { it.toString() }
        .reduce {acc, s -> s + acc}
        .reversed()

    return topCrates
}

fun part2(input: String):String {
    val crates = input.split("\n\n")[0].split("\n")
    val movesAsStrings = input.split("\n\n")[1]

    val stacks = initStacks(crates)

    movesAsStrings
        .split("\n")
        .map { parseMove(it) }
        .forEach { movePart2(stacks, it) }

    //.forEach { println(it) }

    val topCrates = stacks.map { it.peek() }
        .map { it.toString() }
        .reduce {acc, s -> s + acc}
        .reversed()

    return topCrates
}

fun main() {
    val realInput = util.readFileUsingGetResource("day-5-input.txt")
    val testInput =
                         "    [D]    \n" +
                         "[N] [C]    \n" +
                         "[Z] [M] [P]\n" +
                         " 1   2   3 \n" +
                         "\n" +
                         "move 1 from 2 to 1\n" +
                         "move 3 from 1 to 3\n" +
                         "move 2 from 2 to 1\n" +
                         "move 1 from 1 to 2"
    val input = realInput

    println("Part 1 Result = ${part1(input)}")
    // FWNSHLDNZ

    println("Part 2 Result = ${part2(input)}")
    // RNRGDNFQG
}