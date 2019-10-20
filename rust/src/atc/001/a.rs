use std::io::Read;
use std::process::exit;

struct MazeSpec {
    height: usize,
    width: usize,
}

#[derive(Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let height: usize = iter.next().unwrap().parse().unwrap();
    let width: usize = iter.next().unwrap().parse().unwrap();
    let maze_spec = MazeSpec {
        height: height,
        width: width,
    };
    let mut maze: Vec<Vec<(char, bool)>> = (0..height)
        .map(|_| {
            let row: String = iter.next().unwrap().parse().unwrap();
            row.chars().map(|c| (c, false)).collect()
        })
        .collect();
    let mut start_point = (0, 0);
    'outer: for (y, row) in maze.iter().enumerate() {
        for (x, &column) in row.iter().enumerate() {
            if column.0 == 's' {
                start_point = (x, y);
                break 'outer;
            }
        }
    }
    let mut stack = vec![];
    stack.push(Point {
        x: start_point.0,
        y: start_point.1,
    });
    while !stack.is_empty() {
        let current_point = stack.pop().unwrap();
        maze[current_point.y][current_point.x].1 = true;
        if maze[current_point.y][current_point.x].0 == 'g' {
            println!("Yes");
            exit(0);
        }
        push_reachable_point(&current_point, &maze, &maze_spec, &mut stack);
    }
    println!("No");
}

fn push_reachable_point(
    current_point: &Point,
    maze: &Vec<Vec<(char, bool)>>,
    maze_spec: &MazeSpec,
    stack: &mut Vec<Point>,
) {
    let mut push = |x: i32, y: i32| {
        if x >= 0
            && maze_spec.width > x as usize
            && y >= 0
            && maze_spec.height > y as usize
            && maze[y as usize][x as usize].0 != '#'
            && maze[y as usize][x as usize].1 == false
        {
            stack.push(Point {
                x: x as usize,
                y: y as usize,
            });
        }
    };
    push(current_point.x as i32 + 1, current_point.y as i32);
    push(current_point.x as i32 - 1, current_point.y as i32);
    push(current_point.x as i32, current_point.y as i32 + 1);
    push(current_point.x as i32, current_point.y as i32 - 1);
}
