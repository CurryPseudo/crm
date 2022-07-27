use std::{path::Path, time::Instant};

use clap::Parser;
use indicatif::{HumanBytes, ProgressBar, ProgressStyle};

#[derive(Parser)]
struct Args {
    dir: String,
}

struct Context {
    pb: ProgressBar,
    size: u64,
    last_time: Instant,
    last_size: u64,
    rate: u64,
}
fn main() {
    let args = Args::parse();
    let spinner_style = ProgressStyle::default_bar()
        .template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let pb = ProgressBar::new(u64::MAX);
    pb.set_style(spinner_style);
    let dir = args.dir;
    let mut context = Context {
        pb,
        size: 0,
        last_time: Instant::now(),
        last_size: 0,
        rate: 0,
    };
    remove_dir(&dir, &mut context);
    context
        .pb
        .finish_with_message(format!("Deleted {}", HumanBytes(context.size)));
}
fn remove_dir<P: AsRef<Path>>(p: P, context: &mut Context) {
    for entry in p.as_ref().read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            context.pb.println(path.display().to_string());
            let metadata = std::fs::metadata(&path).unwrap();
            let size = metadata.len();
            std::fs::remove_file(&path).unwrap();
            context.size += size;
            let now = Instant::now();
            let duration = now - context.last_time;
            if duration.as_secs_f64() > 0.25 {
                context.rate =
                    ((context.size - context.last_size) as f64 / duration.as_secs_f64()) as u64;
            }
            if duration.as_secs_f64() > 2f64 {
                context.last_time = now;
                context.last_size = context.size;
            }
            context.pb.set_message(format!(
                "Deleted {}, rate: {}/s",
                HumanBytes(context.size),
                HumanBytes(context.rate)
            ));
        } else if path.is_dir() {
            remove_dir(path, context);
        } else {
            panic!("unknown path {}", path.display());
        }
    }
    std::fs::remove_dir(p.as_ref()).unwrap();
    context.pb.println(p.as_ref().display().to_string());
    context.pb.set_message(format!(
        "Deleted {}, rate: {}/s",
        HumanBytes(context.size),
        HumanBytes(context.rate)
    ));
}
