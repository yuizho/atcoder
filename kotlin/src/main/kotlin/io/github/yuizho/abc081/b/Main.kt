package io.github.yuizho.abc081.b

fun main(args: Array<String>) {
    val n = readLine()!!
    val inputs = readLine()!!.split(" ").map(String::toInt)
    var attemptedTimes = 0
    var devidedValues = inputs
    while (devidedValues.all { it -> it % 2 == 0 }) {
        devidedValues = devidedValues.map { it -> it / 2 }
        attemptedTimes += 1
    }
    println(attemptedTimes)
}