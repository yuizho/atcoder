package io.github.yuizho.abc083.b

fun main(args: Array<String>) {
    val (max, left, right) = readLine()!!.split(" ").map(String::toInt)
    val total = (1..max)
        // a in left...right is left <= a && a <= right
        .filter { it.toString().sumBy(Character::getNumericValue) in left..right }
        .sum()
    print(total)
}