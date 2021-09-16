use tetris::*;

fn main() {
    show_tetromino(0);
    show_tetromino(1);
    show_tetromino(2);
    show_tetromino(3);
    show_tetromino(4);
    show_tetromino(5);
    show_tetromino(6);

    let mut field = create_field();

    show_field(&field);
}
