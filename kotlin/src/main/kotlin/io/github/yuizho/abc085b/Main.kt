package io.github.yuizho.abc085b

fun main(args: Array<String>) {
    val mochiSet = mutableSetOf<String>()
    for (n in 1..readLine()!!.toInt()) {
        mochiSet.add(readLine()!!)
    }
    println(mochiSet.size)
}