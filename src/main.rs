use crossterm::event::KeyEvent;
use crossterm::event::{poll, read, Event};
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::terminal::enable_raw_mode;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tetris::*;

enum Movement {
    Up,
    Down,
    Right,
    Left,
}

fn print_events(sender: mpsc::Sender<Movement>) -> crossterm::Result<()> {
    loop {
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(500))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('w') {
                        sender.send(Movement::Up).unwrap();
                    } else if event.code == KeyCode::Char('a') {
                        sender.send(Movement::Left).unwrap();
                    } else if event.code == KeyCode::Char('s') {
                        sender.send(Movement::Down).unwrap();
                    } else if event.code == KeyCode::Char('d') {
                        sender.send(Movement::Right).unwrap();
                    } else if event.code == KeyCode::Up {
                        sender.send(Movement::Up).unwrap();
                    } else if event.code == KeyCode::Left {
                        sender.send(Movement::Left).unwrap();
                    } else if event.code == KeyCode::Down {
                        sender.send(Movement::Down).unwrap();
                    } else if event.code == KeyCode::Right {
                        sender.send(Movement::Right).unwrap();
                    } else if event.code == KeyCode::Char('q') {
                        std::process::abort();
                    } else if event.code == KeyCode::Char('c')
                        && event.modifiers == KeyModifiers::CONTROL
                    {
                        std::process::abort();
                    }
                }
                Event::Mouse(event) => println!("{:?}", event),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
            }
        } else {
            // Timeout expired and no `Event` is available
        }
    }
    Ok(())
}

fn main() {
    let (sender, receiver) = mpsc::channel::<Movement>();
    let handle = thread::spawn(move || {
        enable_raw_mode().unwrap();
        print_events(sender).unwrap();
    });

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
    let mut current_rotation: usize = 0;
    let mut current_y: usize = 0;
    let mut current_x: usize = FIELD_WIDTH / 2 - 1;
    let mut speed = 20;
    let mut speed_counter = 0;
    let mut force_down = false;
    // Game loop
    loop {
        // Game timing
        thread::sleep(Duration::from_millis(50));
        force_down = speed_counter == speed;
        speed_counter += 1;
        // Game logic
        // --Input
        match receiver.try_recv() {
            Ok(movement) => match movement {
                Movement::Up => {
                    if does_piece_fix(
                        current_x as i32,
                        current_y as i32,
                        &field,
                        current_piece,
                        current_rotation + 1,
                    ) {
                        current_rotation += 1;
                    }
                }
                Movement::Down => {
                    if does_piece_fix(
                        current_x as i32,
                        current_y as i32 + 1,
                        &field,
                        current_piece,
                        current_rotation,
                    ) {
                        current_y += 1;
                    }
                }
                Movement::Right => {
                    if does_piece_fix(
                        current_x as i32 + 1,
                        current_y as i32,
                        &field,
                        current_piece,
                        current_rotation,
                    ) {
                        current_x += 1
                    }
                }
                Movement::Left => {
                    if does_piece_fix(
                        current_x as i32 - 1,
                        current_y as i32,
                        &field,
                        current_piece,
                        current_rotation,
                    ) {
                        current_x -= 1;
                    }
                }
                _ => (),
            },
            Err(_) => (),
        };

        if force_down {
            if does_piece_fix(current_x as i32, current_y as i32 + 1, &field, current_piece, current_rotation) {
                current_y += 1;
                force_down = false;
                speed_counter = 0;
            } else {

                // lock the current piece in the field
                for x in 0..4 {
                    for y in 0..4 {
                        let mut piece_idx = rotate(x, y, current_rotation);
                        if tetromino[current_piece][piece_idx] == PIECE {
                            field[(y + current_y) * FIELD_WIDTH + current_x + x] =
                                tetromino[current_piece][piece_idx];
                        }
                    }
                }

                //check if we got any lines
                for y in 0..4{
                    let mut line: bool = true;
                    for x in 1..FIELD_WIDTH-1 {
                        line &= field[(y+current_y)*FIELD_WIDTH + x] != EMPTY_SPACE;
                    }
                    
                    if line {
                        for i in 1..FIELD_WIDTH-1 {
                            field[(y+current_y)*FIELD_WIDTH+i] = BORDER;
                        }
                    }
                }

                current_piece = 2;
                current_rotation = 0;
                current_y = 0;
                current_x = FIELD_WIDTH / 2 - 1;
                speed_counter = 0;
                force_down = false;

                if !does_piece_fix(current_x as i32, current_y as i32, &field, current_piece, current_rotation) {
                    
                    std::process::abort();
                }
            }
        }

        // Render output
        draw_field_on_screen(&field, &mut screen);
        //Draw the current piece
        for y in 0..4 {
            for x in 0..4 {
                if tetromino[current_piece][rotate(x, y, current_rotation)] != EMPTY_SPACE {
                    screen[(y + current_y + 2) * SCREEN_WIDTH + x + current_x + 2] =
                        tetromino[current_piece][rotate(x, y, current_rotation)];
                }
            }
        }
        print!("\x1B[2J\x1B[1;1H");
        show_screen(&screen);
    }

    //draw_field_on_screen(&field, &mut screen);

    // Draw tetromino piece on the screen
    /*for y in 0..4 {
        for x in 0..4 {
            screen[(y + current_y + 2) * SCREEN_WIDTH + x + current_x + 2] =
                tetromino[current_piece][y * 4 + x];
        }
    }

    for y in 0..4 {
        for x in 0..4 {
            screen[(y + 2) * SCREEN_WIDTH + x + 18] =
                tetromino[current_piece][y * 4 + x];
        }
    }

    show_field(&field);
    show_screen(&screen);*/
    handle.join().unwrap();
}
