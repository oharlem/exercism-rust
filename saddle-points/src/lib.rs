pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {

    // tests use (row, column) coords
    let mut res: Vec<(usize, usize)> = Vec::new();

    let row_len = input.len();

    for r in 0..row_len {
        'inner: for c in 0..input[0].len() {
            // validate row max
            if &input[r][c] < input[r].iter().max().unwrap() {
                continue;
            }

            // validate column min
            for i in 0..row_len {
                if &input[i][c] < &input[r][c] {
                    break 'inner;
                }
            }

            res.push((r as usize, c as usize));
        }
    }

    res
}
