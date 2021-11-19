pub mod countdown {
    use std::{thread, time};
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

    const CLEAR_LINE: &str = "\x1B[1A\x1B[2K\r";

    pub fn exercise() {
        let mut elapsed = -1;
        let mut color = StandardStream::stdout(ColorChoice::Always);

        for i in 1..=4 {
            if i % 2 != 0 {
                // Resetting color
                if let Err(e) = color.reset() {
                    eprintln!("Error: {}", e);
                }
                println!("\nKEGEL");
            } else {
                // Resetting color
                if let Err(e) = color.reset() {
                    eprintln!("Error: {}", e);
                }
                println!("\nREVERSE KEGEL");
            }

            // Setting green color
            if let Err(e) = color.set_color(ColorSpec::new().set_fg(Some(Color::Green))) {
                eprintln!("Error: {}", e);
            }
            println!("TIME LEFT FOR EXERCISE:");

            print_and_count(&mut elapsed, 30);

            // Setting green color
            if let Err(e) = color.set_color(ColorSpec::new().set_fg(Some(Color::Green))) {
                eprintln!("Error: {}", e);
            }
            println!("TIME TO RELAX:");

            print_and_count(&mut elapsed, 15);
        }
    }

    fn print_and_count(elapsed: &mut i32, start_time: i32) {
        let mut color = StandardStream::stdout(ColorChoice::Always);
        // Setting red color
        if let Err(e) = color.set_color(ColorSpec::new().set_fg(Some(Color::Red))) {
            eprintln!("Error: {}", e);
        }

        for i in (1..=start_time).rev() {
            println!("{} seconds | {}", i, super::elapsed_time(elapsed));
            thread::sleep(time::Duration::from_secs(1));
            if i != 1 {
                print!("{}", CLEAR_LINE)
            };
        }
    }
}

struct Time {
    seconds: i32,
    minutes: i32,
}

fn elapsed_time(elapsed: &mut i32) -> String {
    *elapsed += 1;
    // 2700 = 45 minutes in seconds
    // n/27 = n/2700 * 100 (percentage)
    let percentage: i32 = *elapsed / 27;
    let time = Time {
        seconds: *elapsed % 60,
        minutes: (*elapsed / 60) % 60,
    };

    format!(
        "{:0>2}:{:0>2} - 45:00 ({}%)",
        time.minutes, time.seconds, percentage
    )
}
