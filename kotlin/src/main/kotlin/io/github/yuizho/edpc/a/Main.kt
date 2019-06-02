package io.github.yuizho.edpc.a

fun main(args: Array<String>) {
    val N = readLine()?.toInt() ?: 0
    val steps
            = readLine()!!.split(" ").map(String::toInt)
    val dp = listOf(0).toMutableList()
    for ((i, v) in steps.withIndex()) {
        if (i < 1) {
            continue
        }
        val oneStepCost = (Math.abs(v - steps[i-1]) + dp[i - 1])
        val twoStepCost =
            if (i < 2) Int.MAX_VALUE else (Math.abs(v - steps[i-2]) + dp[i - 2])

        val bestMove =
            if (oneStepCost < twoStepCost) oneStepCost else twoStepCost
        dp.add(bestMove)
    }
    println(dp.last())
}