fn main() {
    let lines = include_str!("../test.txt").lines();

    type LineSegment = Vec<Vec<usize>>;
    let x1 = |l: &LineSegment| l[0][0];
    let y1 = |l: &LineSegment| l[0][1];
    let x2 = |l: &LineSegment| l[1][0];
    let y2 = |l: &LineSegment| l[1][1];
    let low_x = |l: &LineSegment| usize::min(x1(l), x2(l));
    let high_x = |l: &LineSegment| usize::max(x1(l), x2(l));
    let low_y = |l: &LineSegment| usize::min(y1(l), y2(l));
    let high_y = |l: &LineSegment| usize::max(y1(l), y2(l));
    let vertical = |l: &LineSegment| x1(l) == x2(l);
    let horizontal = |l: &LineSegment| y1(l) == y2(l);

    let line_segments: Vec<LineSegment> = lines.map(|line| {
        line.split(" -> ")
            .map(|p| p.split(",").map(|v| v.parse::<usize>().unwrap()).collect()).collect()
    }).collect();

    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;
    line_segments.iter().for_each(|l| {
        max_x = usize::max(max_x, high_x(l));
        max_y = usize::max(max_y, high_y(l));
    });

    let mut matrix = vec![vec![0;max_x+1];max_y+1];
    line_segments.iter().for_each(|l| {
        if horizontal(l) {
            (low_x(l)..high_x(l)+1).for_each(|x| {
                matrix[y1(l)][x] += 1;
            })
        } else if vertical(l) {
            (low_y(l)..high_y(l)+1).for_each(|y| {
                matrix[y][x1(l)] += 1;
            })
        }
    });
    for row in matrix.iter() {
        for e in row.iter() {
            if *e == 0 {
                print!(".")
            } else {
                print!("{}", e);
            }
        }
        print!("\n");
    }
}
