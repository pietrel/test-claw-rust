use console::{Term, style};
use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

const FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub async fn spin_with<F, T>(message: &str, done_message: &str, future: F) -> T
where
    F: Future<Output = T>,
{
    let stop = Arc::new(AtomicBool::new(false));
    let stop_clone = stop.clone();
    let message_clone = message.to_string();

    let anim = thread::spawn(move || {
        let term = Term::stderr();
        let mut i = 0;

        while !stop_clone.load(Ordering::Relaxed) {
            let frame = FRAMES[i % FRAMES.len()];
            let _ = term.write_str(&format!(
                "\r{} {}",
                style(frame).cyan().bold(),
                style(&message_clone).dim(),
            ));
            i += 1;
            thread::sleep(Duration::from_millis(80));
        }

        let _ = term.clear_line();
        let _ = term.write_str("\r");
    });

    let start = Instant::now();
    let result = future.await;
    let elapsed = start.elapsed();

    stop.store(true, Ordering::Relaxed);
    let _ = anim.join();

    let time_str = if elapsed.as_secs() >= 60 {
        format!(
            "{} m {:.1} s",
            elapsed.as_secs() / 60,
            elapsed.as_secs_f64() % 60.0
        )
    } else if elapsed.as_millis() >= 1000 {
        format!("{:.1} s", elapsed.as_secs_f64())
    } else {
        format!("{} ms", elapsed.as_millis())
    };

    let term = Term::stderr();
    let _ = term.write_line(&format!(
        "{} {} {}",
        style("✔").green().bold(),
        done_message,
        style(format!("{time_str}")).dim(),
    ));

    result
}
