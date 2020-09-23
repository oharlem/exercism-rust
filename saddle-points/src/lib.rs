pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    // Algorithm:
    // iterate through every number
    // for every number:
    // -- get max of a row
    // -- get min of a column
    // if true for both, add to output

    // tests consider (row, column) location

    let mut res: Vec<(usize, usize)> = Vec::new();

    let cols = &input[0];
    let rows = &input;

    for r in 0..rows.len() {
        for c in 0..cols.len() {

            // get row max
            let mut row_max = u64::MIN;
            for i in 0..cols.len() {
                if input[r][i] > row_max {
                    row_max = input[r][i];
                }
            }

            // get column min
            let mut col_min = u64::MAX;
            for i in 0..rows.len() {
                if input[i][c] < col_min {
                    col_min = input[i][c];
                }
            }

            if &input[r][c] >= &row_max && &input[r][c] <= &col_min {
                res.push((r as usize, c as usize));
            }
        }
    }

    res
}
