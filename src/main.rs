use tetris::*;

fn main() {
    /*show_tetromino(0);
    show_tetromino(1);
    show_tetromino(2);
    show_tetromino(3);
    show_tetromino(4);
    show_tetromino(5);
    show_tetromino(6);*/

    let mut field = create_field();

    let mut screen = create_screen();

    // Game logic
    let mut current_piece = 1;
    let mut current_y = 0;
    let mut current_x = FIELD_WIDTH/2;

    
    
    
    
    
    
    draw_field_on_screen(&field, &mut screen);


    // Draw tetromino piece on the screen
    for y in 0..4 {
        for x in 0..4 {
            screen[(y+current_y)*SCREEN_WIDTH + x + current_x] = tetromino[current_piece][y*4+x];
        }
    }


    //show_field(&field);
    show_screen(&screen);
}
