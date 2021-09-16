pub const BORDER: u8 = 9;
pub const EMPTY_SPACE: u8 = 0;

pub const FIELD_WIDTH: usize = 12;
pub const FIELD_HEIGHT: usize = 18;
pub const SCREEN_WIDTH: usize = 12;
pub const SCREEN_HEIGHT: usize = 12;

pub const tetromino: [[u8; 16]; 7] = [
    [0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0],
];

pub fn rotate(px: usize, py: usize, r: usize) -> usize {
    match r % 4 {
        0 => px + py * 4,
        1 => 12 - 4 * px + py,
        2 => 15 - px - 4 * py,
        3 => 3 - py + 4 * px,
        _ => panic!("Rotação não permitida"),
    }
}

pub fn show_field(field: &[u8]) {
    for (i, value) in field.iter().enumerate() {
        if i % FIELD_WIDTH == 0 {
            println!();
        }
        match *value {
            BORDER => print!("▒"),
            EMPTY_SPACE => print!(" "),
            _ => (),
        }
    }
    println!();
}

pub fn show_tetromino(tetromino_number: usize) {
    for (index, i) in tetromino[tetromino_number].iter().enumerate() {
        if index % 4 == 0 {
            println!();
        }
        match i {
            0 => print!(" "),
            1 => print!("▉"),
            _ => (),
        }
    }
    println!();
}

pub fn create_field() -> [u8; FIELD_HEIGHT * FIELD_WIDTH] {
    let mut field: [u8; FIELD_HEIGHT * FIELD_WIDTH] = [0; FIELD_HEIGHT * FIELD_WIDTH];
    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            field[y * FIELD_WIDTH + x] =
                if x == 0 || x == FIELD_WIDTH - 1 || y == FIELD_HEIGHT - 1 {
                    BORDER
                } else {
                    EMPTY_SPACE
                };
        }
    }
    field
}
