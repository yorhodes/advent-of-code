fn main() {
    let board_size = 5;

    let input = include_str!("../input.txt");
    let mut chunks = input.split("\n\n");
    let number_draw = chunks.next().expect("number draw string").split(",").map(|n| n.parse::<usize>().unwrap());

    let mut boards: Vec<Vec<Vec<(usize, bool)>>> = chunks
        .map(|chunk| {
            chunk
                .split("\n")
                .map(|row| row.split_whitespace().map(|val| (val.parse::<usize>().unwrap(), false)).collect())
                .collect() // all initially unmarked
        })
        .collect();

    let num_boards = boards.len();
    
    println!("num_boards {}", num_boards);

    let mut num_board_wins = 0;
    let mut board_wins = vec![false; num_boards];

    for n in number_draw {
        let mut board_idx = 0;
        for board in boards.iter_mut() {
            if board_wins[board_idx] {
                board_idx += 1;
                continue;
            }

            let mut col = 0;
            let mut col_counts = vec![0; board_size];

            for row in board.iter_mut() {
                if board_wins[board_idx] {
                    break;
                }

                let mut row_count = 0;
                for elem in row {
                    if elem.0 == n {
                        elem.1 = true;
                    }
                    if elem.1 {
                        row_count += 1;
                        col_counts[col] += 1;
                    }
                    if row_count == board_size || col_counts[col] == board_size {
                        if !board_wins[board_idx] {
                            board_wins[board_idx] = true;
                            num_board_wins += 1;
                        }
                    }
                    col = (col + 1) % board_size;
                }
            }

            // calculate (last) winning score
            if num_board_wins == num_boards {
                let mut sum = 0;
                for row in board {
                    for elem in row {
                        if !elem.1 {
                            sum += elem.0;
                        }
                    }
                }
                println!("board[{}] sum * drawn {}", board_idx, sum * n);
                std::process::exit(0);
            }

            board_idx += 1;
        }
    }
}
