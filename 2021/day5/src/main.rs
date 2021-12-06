fn main() {
    let lines = include_str!("../test.txt").lines();

    type LineSegment = Vec<Vec<usize>>;
    let x1 = |l: &LineSegment| l[0][0];
    let y1 = |l: &LineSegment| l[0][1];
    let x2 = |l: &LineSegment| l[1][0];
    let y2 = |l: &LineSegment| l[1][1];
    let low_x = |l: &LineSegment| x1(l).min(x2(l));
    let high_x = |l: &LineSegment| x1(l).max(x2(l));
    let low_y = |l: &LineSegment| y1(l).min(y2(l));
    let high_y = |l: &LineSegment| y1(l).max(y2(l));
    let vertical = |l: &LineSegment| x1(l) == x2(l);
    let horizontal = |l: &LineSegment| y1(l) == y2(l);
    let diagonal = |l: &LineSegment| high_y(l) - low_y(l) == high_x(l) - low_x(l);

    let line_segments: Vec<LineSegment> = lines.map(|line| {
        line.split(" -> ")
            .map(|p| p.split(",").map(|v| v.parse::<usize>().unwrap()).collect()).collect()
    }).collect();

    let mut max_x = 0;
    let mut max_y = 0;
    line_segments.iter().for_each(|l| {
        max_x = max_x.max(high_x(l));
        max_y = max_y.max(high_y(l));
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
        } else if diagonal(l) {
            let (x_inc, y_inc) = (x1(l) == low_x(l), y1(l) == low_y(l));
            let (mut x, mut y) = (x1(l), y1(l));
            while (x,y) != (x2(l),y2(l)) {
                matrix[y][x] += 1;
                if x_inc {
                    x += 1;
                } else {
                    x -= 1;
                }
                if y_inc {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
        }
    });
    let mut num_overlap = 0;
    for row in matrix.iter() {
        for e in row.iter() {
            if *e == 0 {
                print!(".")
            } else {
                if *e >= 2 {
                    num_overlap += 1;
                }
                print!("{}", e);
            }
        }
        print!("\n");
    }
    println!("{}", num_overlap);
}
