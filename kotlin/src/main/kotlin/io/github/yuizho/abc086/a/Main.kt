package io.github.yuizho.abc086.a

fun main(args: Array<String>) {
    val (l, r) = readLine()!!.split(" ").map(String::toInt)
    val result = if ((l * r % 2) == 0) "Even" else "Odd"
    print(result)
}