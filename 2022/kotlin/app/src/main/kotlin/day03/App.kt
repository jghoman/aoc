package day03

fun readFileUsingGetResource(fileName: String): String
        = ClassLoader.getSystemClassLoader().getResource(fileName).readText(Charsets.UTF_8)

fun day03Part1(lines: String): Int {
    return lines
            .split("\n")
            .map { l -> Triple(l, l.substring(0, l.length / 2), l.substring(l.length / 2)) }
            .map { Triple(it.first, it.second.toList(), it.third.toList()) }
            .map { Triple(it.first, HashSet(it.second), HashSet(it.third)) }
            .map { Pair(it.first, it.second.intersect(it.third)) }
            .map { Pair(it.first, it.second.first())}
            .map { Triple(it.first, it.second, if(it.second.isLowerCase()) it.second.code - 96 else it.second.code - 38)}
            .map { it.third }
            .sum()
            //.forEach { println(it) }
            // 7831
}

fun day03Part2(lines: String):Int {
    return lines.split("\n")
            .map { it -> it.toSet() }
            .chunked(3)
            .map { it.get(0).intersect(it.get(1)).intersect(it.get(2))}
            .map { it.first() }
            .map { if(it.isLowerCase()) it.code - 96 else it.code - 38}
            .sum()
            //.forEach { println(it) }
    // 2683
}

fun main() {
    val input = readFileUsingGetResource("day-3-input.txt");
    //val result = day03Part1(input)
    //println("Answer = $result")

    val result2 = day03Part2(input)
    println("Answer = $result2")
}
