package io.github.yuizho.abc049.c

fun main(args: Array<String>) {
    val text = readLine()!!
    var result = "NO"

    var tempText = text
    while(true) {
        val replaced = replace(tempText)
        if (tempText == replaced) {
            break
        }
        if (replaced == "") {
            result = "YES"
            break
        }
        tempText = replaced
    }
    print(result)
}

fun replace(text: String): String {
    return if (text.endsWith("dream")) {
        text.substring(0, text.length - "dream".length)
    } else if (text.endsWith("dreamer")) {
        text.substring(0, text.length - "dreamer".length)
    } else if (text.endsWith("erase")) {
        text.substring(0, text.length - "erase".length)
    } else if (text.endsWith("eraser")) {
        text.substring(0, text.length - "eraser".length)
    } else {
        text
    }
}