package io.github.yuizho.abc085.b

fun main(args: Array<String>) {
    val N = readLine()?.toInt() ?: 0
    val mochiSet = hashSetOf<String>()
    for(n in 0 until N) {
        mochiSet.add(readLine()!!)
    }
    println(mochiSet.size)
}