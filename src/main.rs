mod process_image;
mod x11_display;

use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;
use process_image::process_image;

const INTERVAL: u64 = 2500;

fn main() -> Result<()> {
    print!("nigga im drunk everyday\n");

    println!(
        "Checking your screen for a 'Accept' window every {} second(s)...\n",
        INTERVAL / 1000
    );

    run_x11()
}

fn run_x11() -> Result<()> {
    let ctx = x11_display::X11Context::new()?;

    let mut i: u64 = 0;
    loop {
        print!("\r[{i}] Searching...");
        io::stdout().flush().ok();

        let img = ctx.take_screenshot()?;

        if let Some((match_x, match_y)) = process_image(&img) {
            println!("\r\x1b[32m[{i}] Button found! Accepting match...\x1b[0m");
            println!("\nPlease close this window if everyone accepted and you are in the loading screen.\nI will otherwise continue searching.\n");

            ctx.set_mouse_pos(match_x as i16, match_y as i16)?;
            thread::sleep(Duration::from_millis(100));
            ctx.mouse_click(true)?;
            thread::sleep(Duration::from_millis(100));
            ctx.mouse_click(false)?;
        }

        i += 1;
        sleep_remainder(INTERVAL);
    }
}

fn sleep_remainder(interval_ms: u64) {
    let deadline = Instant::now() + Duration::from_millis(interval_ms);
    let now = Instant::now();
    if deadline > now {
        thread::sleep(deadline - now);
    }
}
