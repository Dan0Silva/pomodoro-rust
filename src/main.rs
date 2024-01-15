use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

fn pomodoro(duration_seconds: u64, message: &str) {
    let pb = ProgressBar::new(duration_seconds);
    pb.set_style(ProgressStyle::default_bar()
        .template(" {elapsed} {msg} | {bar:26.cyan} ")
        .unwrap()
    );
    pb.set_message(message.to_string());

    for _ in 0..duration_seconds {
        pb.inc(1);
        thread::sleep(Duration::from_secs(1));
    }

    pb.finish_with_message("Pomodoro Complete!");
}

fn main() {
    let matches = App::new("Pomodoro CLI")
        .version("1.0")
        .author("Dan0Silva")
        .about("Rust Pomodoro CLI")
        .arg(
            Arg::with_name("work")
                .short("w")
                .long("work")
                .help("Inicia um ciclo de trabalho de 45 minutos"),
        )
        .arg(
            Arg::with_name("break")
                .short("b")
                .long("break")
                .help("Inicia um ciclo de pausa de 15 minutos"),
        )
        .get_matches();

    if matches.is_present("work") {
        pomodoro(45 * 60, "working"); // 45 minutos em segundos
    } else if matches.is_present("break") {
        pomodoro(15 * 60, "breaking"); // 15 minutos em segundos
    } else {
        println!("Uso: pomodoro --work | --break");
    }
}

