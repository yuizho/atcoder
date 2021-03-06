package io.github.yuizho.abc086.c

import java.util.*

fun main(args: Array<String>) {
    val N = readLine()?.toInt() ?: 0
    val eachTern: ArrayList<List<Int>> = arrayListOf()
    for (n in 0 until N) {
        eachTern.add(readLine()!!.split(" ").map(String::toInt))
    }
    var prevTern = listOf(0, 0, 0)
    var result = "Yes"
    for (e in eachTern) {
        val timeDiff = e[0] - prevTern[0]
        val diff
                = Math.abs(e[1] - prevTern[1]) + Math.abs(e[2] - prevTern[2])
        if (timeDiff < diff) {
            result = "No"
            break
        }
        if ((timeDiff % 2 != 0 && diff % 2 == 0) ||
            (timeDiff % 2 == 0 && diff % 2 != 0)) {
            result = "No"
            break
        }
        prevTern = e
    }
    print(result)
}