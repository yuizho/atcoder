package io.github.yuizho.edpc.b

fun main(args: Array<String>) {
    val (n, k) = readLine()!!.split(" ").map(String::toInt)
    val steps
            = readLine()!!.split(" ").map(String::toInt)
    val dp = listOf(0).toMutableList()
    for ((i, v) in steps.withIndex()) {
        if (i < 1) {
            continue
        }
        val costs = listOf<Int>().toMutableList()
        for (j in 1..k) {
            costs.add(
                if (i < j) Int.MAX_VALUE else (Math.abs(v - steps[i-j]) + dp[i-j])
            )
        }
        val bestMove = costs.min() ?: Int.MAX_VALUE
        dp.add(bestMove)
    }
    println(dp.last())
}
