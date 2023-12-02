package doy08

fun part1(input:String): Int {
    val inputSplit = input
        .split("\n")
        .map { it -> it.map { it.digitToInt() } }

    println("numRows = ${inputSplit.size}")
    println("numCols = ${inputSplit[0].size}")

    val z=  (1 until inputSplit[0].size - 1)
        .map{ i -> (1 until inputSplit.size - 1)
            .map { j ->
                Pair(i, j)
        }}
        .flatten()
        .map{ isVisible(inputSplit, it.first, it.second) }
        .count { it }

    val perimeterSize = (4 * inputSplit.size - 4 )

    println("perimeterSize = ${perimeterSize}")
    return z  +  perimeterSize //(4 * (inputSplit.size - 1 ))
}

private fun isVisible(
    inputSplit: List<List<Int>>,
    i: Int,
    j: Int
): Boolean {
    val treeHeight = inputSplit[i][j]
    val width = inputSplit[0].size

    val visibleFromRight = inputSplit[i].subList(j + 1, width).all { it < treeHeight }
    val visibleFromLeft = inputSplit[i].subList(0, j).all { it < treeHeight }

    val visibleFromDown = (i + 1 until inputSplit.size)
        .asSequence()
        .map { inputSplit[it][j] }
        .toList()
        .all { it < treeHeight }

    val visibleFromUp = (0 until i)
        .asSequence()
        .map { inputSplit[it][j] }
        .toList()
        .all { it < treeHeight }

    return visibleFromDown || visibleFromUp || visibleFromLeft || visibleFromRight
}

fun part2(input:String): Int {
    val inputSplit = input
        .split("\n")
        .map { it -> it.map { it.digitToInt() } }

    val z=  (1 until inputSplit[0].size - 1)
        .map{ i -> (1 until inputSplit.size - 1)
            .map { j ->
                Pair(i, j)
            }}
        .flatten()
        .map{ viewingScore(inputSplit, it.first, it.second) }
        .max()

    // Don't forget perimeter!
    return z
}

fun <T> List<T>.takeUntilInclusive(predicate: (T) -> Boolean): List<T> {
    val result = mutableListOf<T>()
    for (element in this) {
        result.add(element)
        if (predicate(element)) {
            break
        }
    }
    return result
}

fun viewingScore(inputSplit: List<List<Int>>, i: Int, j: Int): Int {
    val treeHeight = inputSplit[i][j]
    val width = inputSplit[0].size

    println("treeHeight = $treeHeight")

    val upScore = maxOf(1, (0 until i)
        .asSequence()
        .map { inputSplit[it][j] }
        .toList()
        .reversed()
        .takeUntilInclusive { it >= treeHeight }
        .size)

    val leftScore = maxOf(1, inputSplit[i]
        .subList(0, j)
        .reversed()
        .takeUntilInclusive { it >= treeHeight }
        .size)

    val rightScore = maxOf(1, inputSplit[i]
        .subList(j + 1, width)
        .takeUntilInclusive { it >= treeHeight }
        .size)

    //-------

    println("Hmmmmmmm: ${(i + 1 until inputSplit.size)
        .asSequence()
        .map { inputSplit[it][j] }
        .toList()
        .takeUntilInclusive { it >= treeHeight }
        }")

    val downScore = maxOf(1, (i + 1 until inputSplit.size)
        .asSequence()
        .map { inputSplit[it][j] }
        .toList()
        .takeUntilInclusive { it >= treeHeight }
        .size)

    val up = upScore
    val down = downScore
    val left = leftScore
    val right = rightScore

    println("up = $up (1), left = $left (1), right = $right (2), down = $down (2)")

    return up * down * left * right
}

fun main() {
    val realInput = util.readFileUsingGetResource("day-8-input.txt")

    val testInput = "30373\n" +
                             "25512\n" +
                             "65332\n" +
                             "33549\n" +
                             "35390"

    //val input = testInput
    val input = realInput

    //println("Part 1 result = ${part1(input)}" )
    // 21


    val inputSplit = input
        .split("\n")
        .map { it -> it.map { it.digitToInt() } }

    //val result = viewingScore(inputSplit, 1, 2)
    val result = part2(input)
    println("Result = ${result}")
}
