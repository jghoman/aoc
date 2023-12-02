package day01

import scala.collection.mutable
import scala.io.Source;
import scala.collection.mutable.ListBuffer
object App {
  def main(args: Array[String]): Unit = {
    part2
  }

  def part2(): Unit = {
    val input = Source.fromResource("day-01.txt")
    var currentSum = 0;
    val sums = mutable.ListBuffer[Int]()
    input
      .getLines()
      .foreach(line =>
        line match {
          case "" =>
            sums += currentSum
            currentSum = 0
          case num =>
            val asNum = Integer.parseInt(num);
            currentSum += asNum;
        })

    // Add in the last sum
    sums += currentSum
    val result  = sums
      .sortWith(_ > _)
      .take(3)
      .sum

    println("result = " + result)
    // 202585
  }

  def part1(): Unit = {
    val input = Source.fromResource("day-01.txt")

    var currentTotal = 0
    var largestTotal = -1
    var currentElf = 1
    var carryingElf = 1

    input
      .getLines()
      .foreach(line =>
        line match {
          case "" =>
            if (currentTotal > largestTotal) {
              largestTotal = currentTotal;
              carryingElf = currentElf;
            }
            currentTotal = 0;
            currentElf += 1;
          case num =>
            val asNum = Integer.parseInt(num);
            currentTotal += asNum;
        })

    println("Elf = " + carryingElf)
    println("Largest calories = " + largestTotal)
    // 68775
  }




}
