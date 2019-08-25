package io.github.yuizho.edpc.d

fun main(args: Array<String>) {
    val (N, W) = readLine()!!.split(" ").map(String::toInt)
    val napsack = Array<Array<Int>>(N, {Array<Int>(2, {0})})
    for (n in 0 until N) {
        napsack[n] = readLine()!!.split(" ").map(String::toInt).toTypedArray()
    }
    val dp = Array<Array<Long>>(N + 10, {Array<Long>(W + 10, {0L})})
    for (n in 0 until N) {
        for (w in 0 .. W) {
            val weight = napsack[n][0]
            val value = napsack[n][1]
            // n番目の商品を選ぶ場合
            if (w - weight >= 0) {
                if (dp[n + 1][w] < dp[n][w - weight] + value) {
                    dp[n + 1][w] = dp[n][w - weight] + value
                }
            }

            if (dp[n + 1][w] < dp[n][w]) {
                dp[n + 1][w] = dp[n][w]
            }
        }
    }
    println(dp[N][W])
}