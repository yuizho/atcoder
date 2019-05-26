package io.github.yuizho.abc087.b

fun main(args: Array<String>) {
    val fiveHandledYenCnt = readLine()!!.toInt()
    val handledYenCnt = readLine()!!.toInt()
    val fiftyYenCnt = readLine()!!.toInt()
    val amount = readLine()!!.toInt()

    var answer = 0
    for (fh in 0..fiveHandledYenCnt) {
        var fhAmount = fh * 500
        for(h in 0..handledYenCnt) {
            var hAmount = h * 100
            for (f in 0..fiftyYenCnt) {
                var fAmount = f * 50
                if (fhAmount + hAmount + fAmount == amount) {
                    answer += 1
                }
            }
        }
    }
    print(answer)
}