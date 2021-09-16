use tetris::*;

fn main() {
    show_tetromino(1);

    let mut field = create_field();

    show_field(&field);
}
