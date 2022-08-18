use std::io::stdout;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use termion::screen::AlternateScreen;

use crate::matrix::matrix::Matrix;

pub mod matrix;

fn get_matrix_codes() -> Vec<char> {
    let mut codes = (0x21..0x7E)
        .map(|c| char::from_u32(c).unwrap())
        .collect::<Vec<char>>();
    codes.extend((0xFF66..0xFF9D).map(|c| char::from_u32(c).unwrap()));
    codes
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    {
        let mut screen = AlternateScreen::from(stdout());
        let codes = get_matrix_codes();

        let mut matrix = Matrix::new(&mut screen, &codes);
        matrix.hide_cursor();

        while running.load(Ordering::SeqCst) {
            matrix.draw();
            matrix.update();
            matrix.flush();
            matrix.sleep();
        }

        matrix.unhide_cursor();
    }
    println!("Done");
}
