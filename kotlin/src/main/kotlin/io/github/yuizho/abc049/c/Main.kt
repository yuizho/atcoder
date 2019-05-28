package io.github.yuizho.abc049.c

import java.lang.StringBuilder

fun main(args: Array<String>) {
    val text = readLine()!!
    val keyWords = listOf("dream", "dreamer", "erase", "eraser")
    var result = "NO"
    var sb = StringBuilder(text)
    do {
        var sholdContine = false
        for (keyWord in keyWords) {
            if (sb.endsWith(keyWord)) {
                sb.delete(sb.length - keyWord.length, sb.length)
                sholdContine = true
                break
            }
        }
        if (sb.isEmpty()) {
            result = "YES"
            break
        }
    } while (sholdContine)
    print(result)
}