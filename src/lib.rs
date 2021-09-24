pub const BORDER: u8 = 9;
pub const EMPTY_SPACE: u8 = 0;
pub const PIECE: u8 = 1;

pub const FIELD_WIDTH: usize = 12;
pub const FIELD_HEIGHT: usize = 18;
pub const SCREEN_WIDTH: usize = 80;
pub const SCREEN_HEIGHT: usize = 20;

pub const tetromino: [[u8; 16]; 7] = [
    [0,0,1,0,
     0,0,1,0,
     0,0,1,0,
     0,0,1,0],

    [0,0,1,0,
     0,1,1,0,
     0,1,0,0,
     0,0,0,0],

    [0,1,0,0,
     0,1,1,0,
     0,0,1,0,
     0,0,0,0],

    [0,0,0,0,
     0,1,1,0,
     0,1,1,0,
     0,0,0,0],

    [0,0,1,0,
     0,1,1,0,
     0,0,1,0,
     0,0,0,0],

    [0,1,1,0,
     0,0,1,0,
     0,0,1,0,
     0,0,0,0],

    [0,1,1,0,
     0,1,0,0,
     0,1,0,0,
     0,0,0,0],
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
        match value {
            &EMPTY_SPACE => print!("░"),
            &BORDER => print!("▒"),
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
            &EMPTY_SPACE => print!("░"),
            &PIECE => print!("█"),
            //&PIECE => print!("■"),
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

pub fn create_screen() -> [u8; SCREEN_HEIGHT*SCREEN_WIDTH] {
    let screen: [u8; SCREEN_HEIGHT*SCREEN_WIDTH] = [0; SCREEN_HEIGHT*SCREEN_WIDTH];
    screen
}

pub fn show_screen(screen: &[u8]) {
    for (index, value) in screen.iter().enumerate() {
        if index % SCREEN_WIDTH == 0 {
            println!("\r");
        }
        match value {
            //&EMPTY_SPACE => print!("░"),
            &EMPTY_SPACE => print!(" "),
            &BORDER => print!("▒"),
            &PIECE => print!("█"),
            &i => print!("{}",i as char),
        }
    }
    println!("\r");
}

pub fn draw_field_on_screen(field: &[u8], screen: &mut [u8]) {
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            screen[SCREEN_WIDTH*(y+2)+x+2] = field[FIELD_WIDTH*y+x];
        }
    }
}

pub fn does_piece_fix(pos_x: i32, pos_y: i32, field: &[u8], current_piece: usize, current_rotation: usize) -> bool{
    for x in 0..4 {
        for y in 0..4 {
            let piece_idx = rotate(x, y, current_rotation);
            let field_idx = (pos_y+y as i32 )*FIELD_WIDTH as i32 +pos_x+x as i32;

            if pos_x < 0 || pos_y < 0 {
                return false
            }
            if tetromino[current_piece][piece_idx as usize] == PIECE && field[field_idx as usize] != EMPTY_SPACE {
                return false
            }
        }
    }
    true
}