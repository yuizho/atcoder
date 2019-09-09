use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut str = buf;
    let key_words = ["dream", "dreamer", "erase", "eraser"];
    let result = loop {
        let attempted = key_words.iter().find(|&key_word| str.ends_with(key_word));
        if attempted == None {
            break "NO";
        }
        let end_index = str.len() - attempted.unwrap().len();
        str = str[0..end_index].to_string();
        if str.len() == 0 {
            break "YES";
        }
    };
    println!("{}", result);
}
