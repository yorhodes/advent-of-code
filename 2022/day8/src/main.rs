
fn probe_left(matrix: &Vec<Vec<u32>>, visible_matrix: &mut Vec<Vec<bool>>, row: usize) {
    let mut max = matrix[row][0];
    visible_matrix[row][0] = true;
    for i in 1..matrix.len()-1 {
        if matrix[row][i] > max {
            max = matrix[row][i];
            visible_matrix[row][i] = true;
        }
    }
}


fn probe_right(matrix: &Vec<Vec<u32>>, visible_matrix: &mut Vec<Vec<bool>>, row: usize) {
    let mut max = matrix[row][matrix.len()-1];
    visible_matrix[row][matrix.len()-1] = true;
    for i in (1..matrix.len()-1).rev() {
        if matrix[row][i] > max {
            max = matrix[row][i];
            visible_matrix[row][i] = true;
        }
    }
}

fn probe_top(matrix: &Vec<Vec<u32>>, visible_matrix: &mut Vec<Vec<bool>>, column: usize) {
    let mut max = matrix[0][column];
    visible_matrix[0][column] = true;
    for i in  1..matrix.len()-1 {
        if matrix[i][column] > max {
            max = matrix[i][column];
            visible_matrix[i][column] = true;
        }
    }
}

fn probe_bottom(matrix: &Vec<Vec<u32>>, visible_matrix: &mut Vec<Vec<bool>>, column: usize) {
    let mut max = matrix[matrix.len()-1][column];
    visible_matrix[matrix.len()-1][column] = true;
    for i in (1..matrix.len()-1).rev() {
        if matrix[i][column] > max {
            max = matrix[i][column];
            visible_matrix[i][column] = true;
        }
    }
}

fn part_1(input: &str) -> u32 {
    let lines = input.lines(); // convert input to lines
    // convert lines to u32 matrix
    let matrix: Vec<Vec<u32>> = lines.map(|line| line.chars().map(|x| x as u32 - 48).collect()).collect();

    let mut visible_matrix: Vec<Vec<bool>> = matrix.iter().map(|row| row.iter().map(|_| false).collect()).collect();
    
    let rows = matrix.len();
    for i in 1..rows-1 {
        probe_left(&matrix, &mut visible_matrix,  i);
        probe_right(&matrix, &mut visible_matrix, i);
    }

    let columns = matrix[0].len();
    for i in 1..columns-1 {
        probe_top(&matrix, &mut visible_matrix,  i);
        probe_bottom(&matrix, &mut visible_matrix, i);
    }

    let mut visible = 4;
    for i in 0..rows {
        for j in 0..columns {
            if visible_matrix[i][j] {
                visible += 1;
            }
        }
    }

    return visible;
}

fn scenic_distance(matrix: &Vec<Vec<u32>>, row: usize, column: usize) -> u32 {
    let tree = matrix[row][column];
    let mut left_distance = 0;
    for i in (0..column).rev() {
        left_distance += 1;
        if matrix[row][i] >= tree {
            break;
        }
    }

    let mut right_distance = 0;
    for i in column+1..matrix.len() {
        right_distance += 1;
        if matrix[row][i] >= tree {
            break;
        }
    }

    let mut top_distance = 0;
    for i in (0..row).rev() {
        top_distance += 1;
        if matrix[i][column] >= tree {
            break;
        }
    }

    let mut bottom_distance = 0;
    for i in row+1..matrix.len() {
        bottom_distance += 1;
        if matrix[i][column] >= tree {
            break;
        }
    }

    return left_distance * right_distance * top_distance * bottom_distance;
}

fn part_2(input: &str) -> u32 {
    let lines = input.lines();
    let matrix: Vec<Vec<u32>> = lines.map(|line| line.chars().map(|x| x as u32 - 48).collect()).collect();
    let rows = matrix.len();
    let columns = matrix[0].len();

    let mut max = 0;
    for i in 1..rows-1 {
        for j in 1..columns-1 {
            let distance = scenic_distance(&matrix, i, j);
            if distance > max {
                max = distance;
            }
        }
    }

    return max;
}

const DATA: &str = include_str!("../input.txt");

fn main() {
    println!("part 1: {}", part_1(DATA));
    println!("part 2: {}", part_2(DATA));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = include_str!("../example.txt");

    #[test]
    fn test_1() {
        assert_eq!(part_1(TEST_DATA), 21);
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(TEST_DATA), 8);
    }
}
