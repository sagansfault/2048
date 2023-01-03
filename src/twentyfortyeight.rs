use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Board {
    pub cells: [[u8; 4]; 4]
}

impl Board {
    pub fn new() -> Board {
        let mut cells = [[0; 4]; 4];
        let mut rng = rand::thread_rng();
        for ele in rand::seq::index::sample(&mut rng, 16, 2).iter() {
            let column = ele % 4;
            let row = (ele - column) / 4;
            cells[row][column] = 1;
        }

        Board { cells }
    }

    pub fn shift(&mut self, dir: Direction) {
        match dir {
            Direction::Left => {
                for index in 0..self.cells.len() {
                    self.cells[index] = Board::shift_row(self.cells[index]);
                }
            },
            Direction::Right => {
                for index in 0..self.cells.len() {
                    let mut reversed = self.cells[index];
                    reversed.reverse();
                    let mut processed = Board::shift_row(reversed);
                    processed.reverse();
                    self.cells[index] = processed;
                }
            },
            Direction::Up => {
                for index in 0..self.cells.len() {
                    let mut arr: [u8; 4] = [0; 4];
                    for i in 0..4 {
                        arr[i] = self.cells[i][index];
                    }
                    let processed = Board::shift_row(arr);
                    for i in 0..4 {
                        self.cells[i][index] = processed[i];
                    }
                }
            },
            Direction::Down => {
                for index in 0..self.cells.len() {
                    let mut arr: [u8; 4] = [0; 4];
                    for i in 0..4 {
                        arr[i] = self.cells[i][index];
                    }
                    arr.reverse();
                    let mut processed = Board::shift_row(arr);
                    processed.reverse();
                    for i in 0..4 {
                        self.cells[i][index] = processed[i];
                    }
                }
            },
        }

        // spawn new cell
        let mut v: Vec<(usize, usize)> = vec![];
        for y in 0..4 {
            for x in 0..4 {
                if self.cells[y][x] == 0 {
                    v.push((x,y));
                }
            }
        }
        if !v.is_empty() {
            let (x, y) = v.remove(rand::thread_rng().gen_range(0..v.len()));
            self.cells[y][x] = 1;
        }
    }

    // Defaultly shifts left. Manipulate row before and after function to get other directions
    pub fn shift_row(row: [u8; 4]) -> [u8; 4] {
        let mut row = row; // ensure copy
        let len = row.len();
            for index in 0..len {
                let cell = row[index];
                if index < len - 1 {
                    let mut ind = index + 1;
                    let mut found = true;
                    let mut right_cell = row[ind];
                    'inner: while right_cell == 0 {
                        ind += 1;
                        if ind >= len {
                            found = false;
                            break 'inner;
                        }
                        right_cell = row[ind];
                    }
                    if !found {
                        continue;
                    }
                    if cell == right_cell {
                        row[index] = cell + 1;
                        row[ind] = 0;
                    }
                }
            }
            for index in 0..len {
                let cell = row[index];
                let mut left_ind = index as i32 - 1;
                while left_ind >= 0 && row[left_ind as usize] == 0 {
                    row[left_ind as usize] = cell;
                    row[index] = 0;
                    left_ind -= 1;
                }
            }
            return row;
    }
}

pub enum Direction {
    Left, Right, Up, Down
}