use std::cmp::max;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
struct Piece(u8, u8, u8);

const COL_VALUES: [u8; 4] = [0, 1, 5, 9];
const TL_VALUES: [u8; 4] = [0, 3, 4, 8];
const TR_VALUES: [u8; 4] = [0, 2, 6, 7];


/*
                ____
               /    \
          ____/  0   \____
         /    \      /    \
    ____/   3  \____/  1   \____
   /    \      /    \      /    \
  /  7   \____/  4   \____/  2   \
  \      /    \      /    \      /
   \____/  8   \____/  5   \____/
   /    \      /    \      /    \
  /  12  \____/  9   \____/  6   \
  \      /    \      /    \      /
   \____/  13  \____/  10  \____/
   /    \      /    \      /    \
  /  16  \____/  14  \____/  11  \
  \      /    \      /    \      /
   \____/  17  \____/  15  \____/
        \      /    \      /
         \____/  18  \____/
              \      /
               \____/

Cols :
    - 7, 12, 17
    - 3, 8, 13, 18
    - 0, 4, 9, 14, 19
    - 1, 5, 10, 15
    - 2, 6, 11
top left -> bottom right diags:
    - 0, 1, 2
    - 3, 4, 5, 6
    - 7, 8, 9, 10, 11
    - 12, 13, 14, 15
    - 17, 18, 19
top right -> bottom left diags:
    - 0, 3, 7
    - 1, 4, 8, 12
    - 2, 5, 9, 13, 17
    - 6, 10, 14, 18
    - 11, 15, 19
*/
k
// (cols, tl, tr)
#[derive(Clone, Debug)]
struct HexagonCoord(usize, usize, usize);

const HEXAGON_COORDS: [HexagonCoord; 19] = [
    HexagonCoord(2, 0, 0), // 0
    HexagonCoord(3, 0, 1), // 1
    HexagonCoord(4, 0, 2), // 2
    HexagonCoord(1, 1, 0), // 3
    HexagonCoord(2, 1, 1), // 4
    HexagonCoord(3, 1, 2), // 5
    HexagonCoord(4, 1, 3), // 6
    HexagonCoord(0, 2, 0), // 7
    HexagonCoord(1, 2, 1), // 8
    HexagonCoord(2, 2, 2), // 9
    HexagonCoord(3, 2, 3), // 10
    HexagonCoord(4, 2, 4), // 11
    HexagonCoord(0, 3, 1), // 12
    HexagonCoord(1, 3, 2), // 13
    HexagonCoord(2, 3, 3), // 14
    HexagonCoord(3, 3, 4), // 15
    HexagonCoord(0, 4, 2), // 16
    HexagonCoord(1, 4, 3), // 17
    HexagonCoord(2, 4, 4), // 18
];


#[derive(Clone, Debug)]
struct Problem {
    cols: [Option<u8>; 5],
    tl_diag: [Option<u8>; 5],
    tr_diag: [Option<u8>; 5],
    pieces: HashMap<Piece, Option<usize>>,
    rev_pieces: HashMap<usize, Piece>,
}

enum Updated {
    Col(usize, u8),
    Tl(usize, u8),
    Tr(usize, u8),
}

impl Problem {
    fn display(&self) {
        println!("
                  _____
                 /  {}  \\
           _____/{}     {}\\_____
          /  {}  \\       /  {}  \\
    _____/{}     {}\\_____/{}     {}\\_____
   /  {}  \\       /  {}  \\       /  {}  \\
  /{}     {}\\_____/{}     {}\\_____/{}     {}\\
  \\       /  {}  \\       /  {}  \\       /
   \\_____/{}     {}\\_____/{}     {}\\_____/
   /  {}  \\       /  {}  \\       /  {}  \\
  /{}     {}\\_____/{}     {}\\_____/{}     {}\\
  \\       /  {}  \\       /  {}  \\       /
   \\_____/{}     {}\\_____/{}     {}\\_____/
   /  {}  \\       /  {}  \\       /  {}  \\
  /{}     {}\\_____/{}     {}\\_____/{}     {}\\
  \\       /  {}  \\       /  {}  \\       /
   \\_____/{}     {}\\_____/{}     {}\\_____/
         \\       /  {}  \\       /
          \\_____/{}     {}\\_____/
                \\       /
                 \\_____/
        ",
                 self.rev_pieces.get(&0usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&0usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&0usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&3usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&1usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&3usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&3usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&1usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&1usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&7usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&4usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&2usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&7usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&7usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&4usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&4usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&2usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&2usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&8usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&5usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&8usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&8usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&5usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&5usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&12usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&9usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&6usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&12usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&12usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&9usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&9usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&6usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&6usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&13usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&10usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&13usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&13usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&10usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&10usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&16usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&14usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&11usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&16usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&16usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&14usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&14usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&11usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&11usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&17usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&15usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&17usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&17usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&15usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&15usize).unwrap_or(&Piece(0, 0, 0)).2,
                 self.rev_pieces.get(&18usize).unwrap_or(&Piece(0, 0, 0)).0,
                 self.rev_pieces.get(&18usize).unwrap_or(&Piece(0, 0, 0)).1,
                 self.rev_pieces.get(&18usize).unwrap_or(&Piece(0, 0, 0)).2,
        )
    }

    fn max_potential(&self) -> u32 {
        let mut potential: u32 = 0;

        let importance_order: [usize; 5] = [2, 1, 3, 4, 0];
        let n_hexagon_in_row: [u8; 5] = [3, 4, 5, 4, 3];

        let mut n_9_available = max(0, 2 - self.cols.iter().filter(|&x| *x == Some(9)).count());
        let mut n_5_available = max(0, 2 - self.cols.iter().filter(|&x| *x == Some(5)).count());
        let mut n_1_available = max(0, 2 - self.cols.iter().filter(|&x| *x == Some(1)).count());

        let mut n_8_available = max(0, 2 - self.tl_diag.iter().filter(|&x| *x == Some(8)).count());
        let mut n_4_available = max(0, 2 - self.tl_diag.iter().filter(|&x| *x == Some(4)).count());
        let mut n_3_available = max(0, 2 - self.tl_diag.iter().filter(|&x| *x == Some(3)).count());

        let mut n_7_available = max(0, 2 - self.tr_diag.iter().filter(|&x| *x == Some(7)).count());
        let mut n_6_available = max(0, 2 - self.tr_diag.iter().filter(|&x| *x == Some(6)).count());
        let mut n_2_available = max(0, 2 - self.tr_diag.iter().filter(|&x| *x == Some(2)).count());

        for i in importance_order.iter() {
            if let Some(x) = self.cols.get(*i).unwrap() {
                potential = potential + (x * n_hexagon_in_row.get(*i).unwrap()) as u32;
            } else {
                if n_9_available > 0 {
                    potential += 9u32 * *n_hexagon_in_row.get(*i).unwrap() as u32;
                    n_9_available -= 1;
                    continue;
                }
                if n_5_available > 0 {
                    potential += 5u32 * *n_hexagon_in_row.get(*i).unwrap() as u32;
                    n_5_available -= 1;
                    continue;
                }
                if n_1_available > 0 {
                    potential += 1u32 * *n_hexagon_in_row.get(*i).unwrap() as u32;
                    n_1_available -= 1;
                    continue;
                }
                panic!("Should have hit condition")
            }
        }


        for i in importance_order.iter() {
            if let Some(x) = self.tl_diag.get(*i).unwrap() {
                potential = potential + (x * n_hexagon_in_row.get(*i).unwrap()) as u32;
            } else {
                if n_8_available > 0 {
                    potential += 8u32 * *n_hexagon_in_row.get(*i).unwrap() as u32;
                    n_8_available -= 1;
                    continue;
                }
                if n_4_available > 0 {
                    potential += 4u32 * *n_hexagon_in_row.get(*i).unwrap() as u32;
                    n_4_available -= 1;
                    continue;
                }
                if n_3_available > 0 {
                    potential += 3u32 * *n_hexagon_in_row.get(*i).unwrap() as u32;
                    n_3_available -= 1;
                    continue;
                }
                panic!("Should have hit condition")
            }
        }


        for i in importance_order.iter() {
            if let Some(x) = self.tr_diag.get(*i).unwrap() {
                potential = potential + (x * n_hexagon_in_row.get(*i).unwrap()) as u32;
            } else {
                if n_7_available > 0 {
                    potential += 7u32 * *n_hexagon_in_row.get(*i).unwrap() as u32;
                    n_7_available -= 1;
                    continue;
                }
                if n_6_available > 0 {
                    potential += 6u32 * *n_hexagon_in_row.get(*i).unwrap() as u32;
                    n_6_available -= 1;
                    continue;
                }
                if n_2_available > 0 {
                    potential += 2u32 * *n_hexagon_in_row.get(*i).unwrap() as u32;
                    n_2_available -= 1;
                    continue;
                }
                panic!("Should have hit condition")
            }
        }

        potential
    }

    fn score(&self) -> u32 {
        let mut potential = 0;
        let n_hexagon_in_row: [u32; 5] = [3, 4, 5, 4, 3];
        let mut correct_in_cols = [0, 0, 0, 0, 0];
        let mut correct_in_tls = [0, 0, 0, 0, 0];
        let mut correct_in_trs = [0, 0, 0, 0, 0];

        for (&pos, &Piece(p_col, p_tl, p_tr)) in self.rev_pieces.iter() {
            let HexagonCoord(col, tl, tr) = HEXAGON_COORDS.get(pos).unwrap();

            if p_col == self.cols[*col].unwrap_or(0) {
                correct_in_cols[*col] += 1
            }
            if p_tl == self.tl_diag[*tl].unwrap_or(0) {
                correct_in_tls[*tl] += 1
            }
            if p_tr == self.tr_diag[*tr].unwrap_or(0) {
                correct_in_trs[*tr] += 1
            }
        }

        for (i, &n) in n_hexagon_in_row.iter().enumerate() {
            if correct_in_cols[i] == n {
                potential += n * self.cols[i].unwrap_or(0) as u32
            }
            if correct_in_tls[i] == n {
                potential += n * self.tl_diag[i].unwrap_or(0) as u32
            }
            if correct_in_trs[i] == n {
                potential += n * self.tr_diag[i].unwrap_or(0) as u32
            }
        }

        potential
    }

    fn remaining_pieces(&self) -> Vec<Piece> {
        self.pieces.iter().filter_map(|(p, pos)| {
            if pos.is_none() {
                Some(p.clone())
            } else {
                None
            }
        }).collect()
    }

    fn update(&mut self, update: Updated) -> Result<(), ()> {
        match update {
            Updated::Col(i, v) => { self.cols[i] = Some(v); }
            Updated::Tl(i, v) => { self.tl_diag[i] = Some(v) }
            Updated::Tr(i, v) => { self.tr_diag[i] = Some(v) }
        }

        match update {
            Updated::Col(_, 0) => { return Ok(()); }
            Updated::Tl(_, 0) => { return Ok(()); }
            Updated::Tr(_, 0) => { return Ok(()); }
            _ => {}
        }

        // Check that we have enough pieces to fill all the rows with that value
        let n_pieces_of_updated_value = self.pieces.iter().filter(|&(&Piece(col, tl, tr), _)| {
            match update {
                Updated::Col(_, v) => { col == v }
                Updated::Tl(_, v) => { tl == v }
                Updated::Tr(_, v) => { tr == v }
            }
        }).count();

        let n_hexagons_of_updated_value = HEXAGON_COORDS.iter().filter(|HexagonCoord(col, tl, tr)| {
            match update {
                Updated::Col(_, v) => { self.cols[*col].unwrap_or(0) == v }
                Updated::Tl(_, v) => { self.tl_diag[*tl].unwrap_or(0) == v }
                Updated::Tr(_, v) => { self.tr_diag[*tr].unwrap_or(0) == v }
            }
        }).count();

        if n_pieces_of_updated_value < n_hexagons_of_updated_value {
            return Err(());
        }

        // Iterate over all the hexagons of that row
        for (i, _) in HEXAGON_COORDS.iter().enumerate().filter(|(_, HexagonCoord(col, tl, tr))| {
            match update {
                Updated::Col(v, _) => { *col == v }
                Updated::Tl(v, _) => { *tl == v }
                Updated::Tr(v, _) => { *tr == v }
            }
        }) {
            // Check that it does not conflict with an already present piece
            let piece_at_hexagon = self.rev_pieces.get(&i);
            if let Some(&Piece(p_col, p_tl, p_tr)) = piece_at_hexagon {
                match update {
                    Updated::Col(_, v) if p_col != v => return Err(()),
                    Updated::Tl(_, v) if p_tl != v => return Err(()),
                    Updated::Tr(_, v) if p_tr != v => return Err(()),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn place_forced_pieces(&mut self) -> Result<(), ()> {
        loop {
            let mut placed_piece = false;

            for (i, HexagonCoord(col, tl, tr)) in HEXAGON_COORDS.iter().enumerate() {
                // Check that it does not conflict with an already present piece
                if self.rev_pieces.get(&i).is_some() {
                    continue;
                }

                let remaining_pieces: Vec<_> = self.remaining_pieces();

                let col_value = self.cols[*col].unwrap_or(0);
                let tl_value = self.tl_diag[*tl].unwrap_or(0);
                let tr_value = self.tr_diag[*tr].unwrap_or(0);

                let possible_pieces: Vec<_> = remaining_pieces.iter().filter(|&Piece(p_col, p_tl, p_tr)| {
                    (col_value == 0 || col_value == *p_col)
                        && (tl_value == 0 || tl_value == *p_tl)
                        && (tr_value == 0 || tr_value == *p_tr)
                }).collect();

                match possible_pieces.len() {
                    0 => {
                        return Err(());
                    }
                    1 => {
                        let pos = self.pieces.get_mut(possible_pieces[0]).unwrap();
                        *pos = Some(i);
                        self.rev_pieces.insert(i, possible_pieces[0].clone());
                        placed_piece = true;
                    }
                    _ => {}
                }
            }

            if !placed_piece {
                break;
            }
        }

        Ok(())
    }

    fn sub_problems(&self) -> Vec<Problem> {
        let mut sub_problems = vec![];

        if let Some(i) = self.cols.iter().position(|&x| x.is_none()) {
            for v in COL_VALUES {
                if v != 0 && self.cols.iter().filter(|&&x| x == Some(v)).count() >= 2 {
                    // We cannot have more than 2 columns filled with the same value
                    continue;
                }
                let mut sub_problem = self.clone();
                if let Err(_) = sub_problem.update(Updated::Col(i, v)) {
                    continue;
                }
                if let Err(_) = sub_problem.place_forced_pieces() {
                    continue;
                }

                sub_problems.push(sub_problem);
            }

            return sub_problems;
        }

        if let Some(i) = self.tl_diag.iter().position(|&x| x.is_none()) {
            for v in TL_VALUES {
                if v != 0 && self.tl_diag.iter().filter(|&&x| x == Some(v)).count() >= 2 {
                    // Let's consider that we cannot have more than 2 columns filled with the same value
                    continue;
                }
                let mut sub_problem = self.clone();
                if let Err(_) = sub_problem.update(Updated::Tl(i, v)) {
                    continue;
                }
                if sub_problem.place_forced_pieces().is_ok() {
                    sub_problems.push(sub_problem);
                }
            }

            return sub_problems;
        }

        if let Some(i) = self.tr_diag.iter().position(|&x| x.is_none()) {
            for v in TR_VALUES {
                if v != 0 && self.tr_diag.iter().filter(|&&x| x == Some(v)).count() >= 2 {
                    // Let's consider that we cannot have more than 2 columns filled with the same value
                    continue;
                }
                let mut sub_problem = self.clone();
                if let Err(_) = sub_problem.update(Updated::Tr(i, v)) {
                    continue;
                }
                if sub_problem.place_forced_pieces().is_ok() {
                    sub_problems.push(sub_problem);
                }
            }

            return sub_problems;
        }

        sub_problems
    }
}

fn main() {
    let mut pieces: HashMap<Piece, Option<usize>> = HashMap::new();

    //pieces.insert(Piece(1, 3, 2), None);
    pieces.insert(Piece(5, 3, 2), None);
    pieces.insert(Piece(9, 3, 2), None);
    // pieces.insert(Piece(1, 4, 2), None);
    pieces.insert(Piece(5, 4, 2), None);
    // pieces.insert(Piece(9, 4, 2), None);
    // pieces.insert(Piece(1, 8, 2), None);
    pieces.insert(Piece(5, 8, 2), None);
    pieces.insert(Piece(9, 8, 2), None);
    // pieces.insert(Piece(1, 3, 6), None);
    pieces.insert(Piece(5, 3, 6), None);
    pieces.insert(Piece(9, 3, 6), None);
    // pieces.insert(Piece(1, 4, 6), None);
    pieces.insert(Piece(5, 4, 6), None);
    // pieces.insert(Piece(9, 4, 6), None);
    pieces.insert(Piece(1, 8, 6), None);
    pieces.insert(Piece(5, 8, 6), None);
    pieces.insert(Piece(9, 8, 6), None);
    pieces.insert(Piece(1, 3, 7), None);
    pieces.insert(Piece(5, 3, 7), None);
    // pieces.insert(Piece(9, 3, 7), None);
    pieces.insert(Piece(1, 4, 7), None);
    pieces.insert(Piece(5, 4, 7), None);
    pieces.insert(Piece(9, 4, 7), None);
    pieces.insert(Piece(1, 8, 7), None);
    pieces.insert(Piece(5, 8, 7), None);
    pieces.insert(Piece(9, 8, 7), None);

    let base_problem = Problem {
        cols: [None; 5],
        tl_diag: [None; 5],
        tr_diag: [None; 5],
        pieces,
        rev_pieces: HashMap::new(),
    };


    base_problem.display();

    let mut best_score = 0;
    let mut best_problem = base_problem.clone();

    let mut problem_stack: Vec<Problem> = vec![base_problem];
    let mut n_problems = 0;
    let mut skipped = 0;

    while let Some(problem) = problem_stack.pop() {
        n_problems += 1;
        if problem.max_potential() < best_score {
            skipped += 1;
            continue;
        }
        if problem.score() > best_score {
            best_score = max(best_score, problem.score());
            best_problem = problem.clone();
            println!("---");
            println!("skipped so far {}", skipped);
            println!("best score so far: {}", best_score);
            println!("best problem so far: {:?}", best_problem);
            best_problem.display();
        }
        let sub_problems = problem.sub_problems();
        for sub_problem in sub_problems {
            if sub_problem.max_potential() > best_score {
                skipped += 1;
                problem_stack.push(sub_problem)
            }
        }

        // Sorting the vector in descending order based on max_potential() values
        problem_stack.sort_by(|a, b| b.max_potential().cmp(&a.max_potential()));
    }

    println!("{}", n_problems);
    println!("{}", best_score);
    println!("{:?}", best_problem);
}