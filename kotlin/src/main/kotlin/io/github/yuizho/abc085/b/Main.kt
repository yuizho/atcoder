package io.github.yuizho.abc085.b

fun main(args: Array<String>) {
    val n = readLine()?.toInt() ?: 0
    val mochiSet = List(n) { readLine()!! }.toSet()
    println(mochiSet.size)
}