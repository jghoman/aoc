package day04

data class Range(val start: Int, val end: Int) {
    fun span() = end - start
}

data class Assignments(val first: Range, val second: Range)

fun parseLine(line: String): Assignments {
    val first = line.split(",")[0]
    val second = line.split(",")[1]

    fun parseRange(r: String): Range {
        val start = r.split("-")[0]
        val end = r.split("-")[1]
        return Range(Integer.valueOf(start), Integer.valueOf(end))
    }

    return Assignments(parseRange(first), parseRange(second))
}

fun entirelyContains(a: Assignments): Boolean {
    val (larger, smaller) = if (a.first.span() > a.second.span()) Pair(a.first, a.second) else Pair(a.second, a.first)

    return (larger.start <= smaller.start) && (larger.end >= smaller.end)
}

fun anyOverlap(a: Assignments): Boolean {
    val (first, second) = if (a.first.start < a.second.start) Pair(a.first, a.second) else Pair(a.second, a.first)

    return first.end >= second.start
}

fun part1(input: String): Int {
    return input
        .split("\n")
        .map { parseLine(it) }
        .count { entirelyContains(it) }
    //.forEach { println(it) }
    // 490
}

fun part2(input: String): Int {
    return input
        .split("\n")
        .map { parseLine(it) }
        .count { anyOverlap(it) }
    //.forEach{println("$it -> ${anyOverlap(it)}")};
    // 921
}

fun main() {
    val input = util.readFileUsingGetResource("day-4-input.txt")

    //println("Number of entirely contained assignments: ${part1(input)}")
    println("Number of any overlapping assignments: ${part2(input)}")
}