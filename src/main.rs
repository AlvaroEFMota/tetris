use tetris::*;
use std::time::Duration;
use crossterm::event::{poll, read, Event};
use crossterm::terminal::enable_raw_mode;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::event::KeyEvent;

fn print_events() -> crossterm::Result<()> {
    loop {
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(500))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('z') {
                        println!("Apertou z");
                    } else if event.code == KeyCode::Char('q') {
                        std::process::abort();
                    } else if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL {
                        std::process::abort();
                    }
                },
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
    enable_raw_mode().unwrap();
    print_events().unwrap();
    /*show_tetromino(0);
    show_tetromino(1);
    show_tetromino(2);
    show_tetromino(3);
    show_tetromino(4);
    show_tetromino(5);
    show_tetromino(6);*/

    //let mut field = create_field();

    //let mut screen = create_screen();

    // Game logic
    //let mut current_piece = 1;
    //let mut current_y = 0;
    //let mut current_x = FIELD_WIDTH/2;



    
    
    
    
    
    
    /*draw_field_on_screen(&field, &mut screen);


    // Draw tetromino piece on the screen
    for y in 0..4 {
        for x in 0..4 {
            screen[(y+current_y)*SCREEN_WIDTH + x + current_x] = tetromino[current_piece][y*4+x];
        }
    }


    //show_field(&field);
    show_screen(&screen);
    */
}
