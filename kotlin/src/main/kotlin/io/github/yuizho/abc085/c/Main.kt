package io.github.yuizho.abc085.c

 fun main(args: Array<String>) {
    val (N, value) = readLine()!!.split(" ").map(String::toInt)
    for (n10000 in 0..N) {
        for (n5000 in 0..(N - n10000)) {
            val n1000 = N - n10000 - n5000
            if (n10000 * 10000 + n5000 * 5000 + n1000 * 1000 == value) {
                print(listOf(n10000, n5000, n1000).joinToString(" "))
                return
            }
        }
    }
    print(listOf(-1, -1, -1).joinToString(" "))
}