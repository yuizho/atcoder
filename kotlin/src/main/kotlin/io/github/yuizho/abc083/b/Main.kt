package io.github.yuizho.abc083.b

fun main(args: Array<String>) {
    val (max, left, right) = readLine()!!.split(" ").map(String::toInt)
    var total = 0
    for (num in 1..max) {
        val subtotal = num.toString().sumBy { it.toString().toInt() }
        if (subtotal in left..right) total += num
    }
    print(total)
}