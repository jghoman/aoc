package day07

import java.lang.Exception
import java.util.Stack

val testInput = "\$ cd /\n" +
        "\$ ls\n" +
        "dir a\n" +
        "14848514 b.txt\n" +
        "8504156 c.dat\n" +
        "dir d\n" +
        "\$ cd a\n" +
        "\$ ls\n" +
        "dir e\n" +
        "29116 f\n" +
        "2557 g\n" +
        "62596 h.lst\n" +
        "\$ cd e\n" +
        "\$ ls\n" +
        "584 i\n" +
        "\$ cd ..\n" +
        "\$ cd ..\n" +
        "\$ cd d\n" +
        "\$ ls\n" +
        "4060174 j\n" +
        "8033020 d.log\n" +
        "5626152 d.ext\n" +
        "7214296 k"

sealed class Node() {
    data class Directory(val name: String):Node() {
        val nodes = ArrayList<Node>()
        override fun size() = nodes.sumOf { it.size() }

        fun addNode(node: Node) = nodes.add(node)

        override fun toString(): String {
            return "File(name='$name', totalSize=${size()})"
        }
    }

    data class File(val name: String, val size: Long): Node() {
        override fun size() = size
        override fun toString(): String {
            return "File(name='$name', size=$size)"
        }
    }

    abstract fun size():Long
}

fun printNode(node: Node, offset:Int = 0) {
    val sb = StringBuilder("")

    for(i in 0..offset) sb.append("  ")
    when(node) {
        is Node.Directory -> {
            println("${sb}${node.name}")
            node.nodes.forEach { printNode(it, offset + 1)}
        }
        is Node.File -> println("${sb}${node.name} ${node.size}")
    }
}

fun allDirs(node: Node): List<Node.Directory> {
    when(node) {
        is Node.Directory -> return (node.nodes.map{ allDirs(it) }.flatten()) + node
        is Node.File -> return ArrayList()
    }
}

fun part1(input: String): Pair<Long, Node.Directory> {
    val lines = input
        .split("\n")

    val root = Node.Directory("/")
    val dirStack = Stack<Node.Directory>()
    dirStack.add(root)

    var currentDir = root;

    for(line in lines) {
        if(line.equals("$ ls")) {
            continue
            //println("LS-ing")
        } else if (line.startsWith("dir")) {
            val dirName = line.split(" ")[1];
            currentDir.addNode(Node.Directory(dirName))
        } else if (line[0].isDigit()) {
            val split = line.split(" ")
            val fileName = split[1]
            val size = split[0].toLong()
            currentDir.addNode(Node.File(fileName, size))
        } else if (line.equals("$ cd ..")) {
            if (dirStack.size == 1) {
                currentDir = root
            } else {
                currentDir = dirStack.pop();
            }
        } else if (line.equals("$ cd /")) {
            dirStack.clear()
            dirStack.add(root)
        } else if (line.startsWith("$ cd ")) {
            val targetDir = line.split(" ")[2]

            val targetNode = currentDir.nodes.find {it is Node.Directory && it.name.equals(targetDir)} as Node.Directory

            if(targetNode == null) {
                throw Exception("No such dir.")
            }

            if(!(dirStack.size == 1 && currentDir == root)) {
                dirStack.push(currentDir)
            }
            currentDir = targetNode
        } else {
            println("Huh? $line")
        }
    }

    return Pair(allDirs(root)
        .filter { it.size() <= 100000 }
        .sumOf { it.size() }, root)

}

fun part2(root:Node): Long {
    val totalSize = 70000000
    val needed = 30000000

    val freeSpace = totalSize - root.size()
    val needToFree = needed - freeSpace

    return allDirs(root)
        .map { it.size() }
        .filter { it >= needToFree }
        .minBy { it }
}

fun main() {
    val realInput = util.readFileUsingGetResource("day-7-input.txt")

    val input = realInput

    val (partOneSize, root) = part1(input)
    println("Part 1 result: $partOneSize")

    println("Part 2 result: ${part2(root)}")

}