use std::io::Read;
use std::process::exit;

type Point = (usize, usize);

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let maze: Vec<Vec<char>> = (0..10)
        .map(|_| {
            let row: String = iter.next().unwrap().parse().unwrap();
            row.chars().collect()
        })
        .collect();

    for (y, row) in maze.iter().enumerate() {
        for (x, &column) in row.iter().enumerate() {
            if column != 'x' {
                continue;
            }
            // xをoに置き換えたマスを作成
            let mut copied: Vec<Vec<(char, bool)>> = maze
                .iter()
                .map(|row| row.iter().map(|&c| (c, false)).collect())
                .collect();
            copied[y][x] = ('o', false);
            // oのマスがすべて辿れるようになっているかチェック
            let mut stack = vec![(x, y)];
            while !stack.is_empty() {
                let current_point = stack.pop().unwrap();
                // rachedにする
                copied[current_point.1][current_point.0].1 = true;
                push_reachable_point(&current_point, &copied, &mut stack);
            }
            if copied
                .iter()
                .flat_map(|x| x)
                .filter(|x| x.0 == 'o')
                .all(|x| x.1 == true)
            {
                println!("YES");
                exit(0);
            }
        }
    }
    println!("NO");
}

fn push_reachable_point(
    current_point: &Point,
    maze: &Vec<Vec<(char, bool)>>,
    stack: &mut Vec<Point>,
) {
    let mut push = |x: i32, y: i32| {
        if x >= 0
            && 10 > x
            && y >= 0
            && 10 > y
            && maze[y as usize][x as usize].0 == 'o'
            && !maze[y as usize][x as usize].1
        {
            stack.push((x as usize, y as usize));
        }
    };
    push(current_point.0 as i32 + 1, current_point.1 as i32);
    push(current_point.0 as i32 - 1, current_point.1 as i32);
    push(current_point.0 as i32, current_point.1 as i32 + 1);
    push(current_point.0 as i32, current_point.1 as i32 - 1);
}
