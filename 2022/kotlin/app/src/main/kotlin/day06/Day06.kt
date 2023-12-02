package day06

fun part1(input: String): Int {
    val lastChars = ArrayList<Char>()
    var idx = 0

    repeat(4) { lastChars.add(input[idx++]) }

    println(lastChars)
    while(idx < input.length) {
        val set = HashSet(lastChars)
        if(set.size == 4) {
            return idx
        }
        lastChars.removeFirst()
        lastChars.add(input[idx++])

        println("$lastChars")
    }
    return -1
}

fun part2(input: String): Int {
    val lastChars = ArrayList<Char>()
    var idx = 0

    repeat(14) { lastChars.add(input[idx++]) }

    println(lastChars)
    while(idx < input.length) {
        val set = HashSet(lastChars)
        if(set.size == 14) {
            return idx
        }
        lastChars.removeFirst()
        lastChars.add(input[idx++])

        println("$lastChars")
    }
    return -1
}

fun main() {
    val realInput = util.readFileUsingGetResource("day-6-input.txt")
    val testInput = "mjqjpqmgbljsphdztnvjfqwrcgsmlb"

    val input = realInput

//    val result = part1(input)
//    println("Part 1 result: $result")

    val result2 = part2(input)
    println("Part 2 result: $result2")
}