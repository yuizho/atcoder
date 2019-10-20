use std::io::Read;
use std::process::exit;

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
    let mut manipulaed_mazes = vec![];
    let mut stack = vec![];
    'outer: for (y, row) in maze.iter().enumerate() {
        for (x, &column) in row.iter().enumerate() {
            if ((x != 0 && x != 9 && maze[y][x - 1] == 'o' && maze[y][x + 1] == 'o')
                || (y != 0 && y != 9 && maze[y - 1][x] == 'o' && maze[y + 1][x] == 'o'))
                && column == 'x'
            {
                let mut copied: Vec<Vec<(char, bool)>> = maze
                    .iter()
                    .map(|row| row.iter().map(|&c| (c, false)).collect())
                    .collect();
                copied[y][x] = ('o', false);
                manipulaed_mazes.push(copied);
            }
            if column == 'o' && stack.is_empty() {
                stack.push((x, y));
            }
        }
    }

    for _maze in manipulaed_mazes {
        let mut _m = _maze.clone();
        while !stack.is_empty() {
            let current_point = stack.pop().unwrap();
            // rachedにする
            _m[current_point.1][current_point.0].1 = true;
            push_reachable_point(&current_point, &_m, &mut stack);
        }
        if _m
            .iter()
            .flat_map(|x| x)
            .filter(|x| x.0 == 'o')
            .all(|x| x.1 == true)
        {
            println!("YES");
            exit(0);
        }
    }
    println!("NO");
}

fn push_reachable_point(
    current_point: &(usize, usize),
    maze: &Vec<Vec<(char, bool)>>,
    stack: &mut Vec<(usize, usize)>,
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
