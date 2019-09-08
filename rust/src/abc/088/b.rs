use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let N: u8 = iter.next().unwrap().parse().unwrap();
    let mut numbers: Vec<u8> = (0..N)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    let mut alice = 0;
    let mut bob = 0;
    let mut tern = 0;
    while numbers.len() > 0 {
        // ココで参照取ると、numbers.removeでmutable参照が取れない
        /*
        19 |         let max = numbers.iter().max().unwrap();
              |                   ------- immutable borrow occurs here
        20 |         let max_index: usize = numbers.iter().position(|s| s == max).unwrap();
        21 |         numbers.remove(max_index);
              |         ^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
        ...
        25 |             bob += max;
            |                    --- immutable borrow later used here
        */
        let max = *numbers.iter().max().unwrap();
        let max_index: usize = numbers.iter().position(|&s| s == max).unwrap();
        numbers.remove(max_index);
        if tern % 2 == 0 {
            alice += max;
        } else {
            bob += max;
        }
        tern += 1;
    }
    println!("{}", alice - bob);
}
