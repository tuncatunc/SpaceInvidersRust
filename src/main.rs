use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use inviders::{frame, render};

use std::{error::Error, io, sync::mpsc, time::Duration, thread};

use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    // let mut audio = Audio::new();
    // audio.add("explode", "explode.wav");
    // audio.add("lose", "lose.wav");
    // audio.add("move", "move.wav");
    // audio.add("pew", "pew.wav");
    // audio.add("startup", "startup.wav");
    // audio.add("win", "win.wav");
    // audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        'renderloop: loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break 'renderloop,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Player
    let mut player = Player::new();

    // Game loop
    'gameloop: loop {
        // Per frame init
        let curr_frame = frame::new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        // audio.play("lose")
                        println!("lose");
                        break 'gameloop;
                    }
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    _ => {}
                }
            }
        }

        // Draw & Render
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(16));
    }


    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    
    // audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
