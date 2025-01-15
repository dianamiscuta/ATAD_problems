// Crossword Puzzle problem

fn crosswordPuzzle(crossword: &[String], words: &str) -> Vec<String> {
    let mut crossword: Vec<Vec<char>> = crossword.iter().map(|row| row.chars().collect()).collect(); // Convert to mutable char grid
    let mut word_list: Vec<&str> = words.split(';').collect();

    word_list.sort_by_key(|&word| -(word.len() as isize)); // Sort words by length (longest first)

    fn can_place(crossword: &[Vec<char>], word: &str, row: usize, col: usize, horizontal: bool) -> bool {
        for (i, ch) in word.chars().enumerate() {
            let (r, c) = if horizontal { (row, col + i) } else { (row + i, col) };
            if r >= 10 || c >= 10 || (crossword[r][c] != '-' && crossword[r][c] != ch) {
                return false;
            }
        }
        true
    }

    fn place(crossword: &mut [Vec<char>], word: &str, row: usize, col: usize, horizontal: bool) -> Vec<(usize, usize)> {
        let mut positions = vec![];
        for (i, ch) in word.chars().enumerate() {
            let (r, c) = if horizontal { (row, col + i) } else { (row + i, col) };
            if crossword[r][c] == '-' {
                crossword[r][c] = ch;
                positions.push((r, c));
            }
        }
        positions
    }

    fn unplace(crossword: &mut [Vec<char>], positions: &[(usize, usize)]) {
        for &(r, c) in positions {
            crossword[r][c] = '-';
        }
    }

    fn solve(crossword: &mut [Vec<char>], words: &[&str]) -> bool {
        if words.is_empty() {
            return true;
        }

        let word = words[0];
        for row in 0..10 {
            for col in 0..10 {
                if can_place(crossword, word, row, col, true) {
                    let positions = place(crossword, word, row, col, true);
                    if solve(crossword, &words[1..]) {
                        return true;
                    }
                    unplace(crossword, &positions);
                }

                if can_place(crossword, word, row, col, false) {
                    let positions = place(crossword, word, row, col, false);
                    if solve(crossword, &words[1..]) {
                        return true;
                    }
                    unplace(crossword, &positions);
                }
            }
        }

        false
    }

    solve(&mut crossword, &word_list);
    crossword.into_iter().map(|row| row.into_iter().collect()).collect()
}
