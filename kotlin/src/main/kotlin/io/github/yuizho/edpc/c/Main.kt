package io.github.yuizho.edpc.c

fun main(args: Array<String>) {
    val N = readLine()?.toInt() ?: 0
    val activitiesEachDay: MutableList<List<Int>> = mutableListOf()
    val dp: MutableList<MutableList<Int>> = mutableListOf(mutableListOf(0, 0, 0))
    for (n in 0 until N) {
        activitiesEachDay.add(readLine()!!.split(" ").map(String::toInt))
        dp.add(mutableListOf(0, 0, 0))
    }
    // https://photos.app.goo.gl/Mbn1e7TctyhozFEb6
    for ((i, activities) in activitiesEachDay.withIndex()) {
        for((j, activity) in activities.withIndex()) {
            for (k in 0 until 3) {
                if (j == k)
                    continue
                if (dp[i + 1][k] < (dp[i][j] + activity)) {
                    dp[i + 1][k] = (dp[i][j] + activity)
                }
            }
        }
    }
    println(dp[N].max())
}