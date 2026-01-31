use std::{
    env, fs,
    path::Path,
    time::{Duration, SystemTime},
};

fn parse_duration(arg: &str) -> Option<Duration> {
    let (num, unit) = arg.split_at(arg.len() - 1);
    let value: u64 = num.parse().ok()?;

    match unit {
        "m" => Some(Duration::from_secs(value * 60)),
        "h" => Some(Duration::from_secs(value * 60 * 60)),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: ssh-enable-login <Nm|Nh>");
        return;
    }

    let duration = match parse_duration(&args[1]) {
        Some(d) => d,
        None => {
            eprintln!("invalid duration (use e.g. 15m or 1h)");
            return;
        }
    };

    let user = match env::var("SUDO_USER") {
        Ok(u) => u,
        Err(_) => {
            eprintln!("must be run via sudo");
            return;
        }
    };

    // TODO: Check if user exists on system, and is in specific group?

    let host = match hostname::get() {
        Ok(h) => h.to_string_lossy().into_owned(),
        Err(_) => return,
    };

    let principal = format!("{}@{}", user, host);
    let path = Path::new("/var/lib/ssh-principals/enabled").join(&principal);

    // Create or update approval file
    if fs::write(&path, b"enabled\n").is_err() {
        return;
    }

    // Encode expiry via mtime
    let expires = SystemTime::now() + duration;
    let _ = filetime::set_file_mtime(&path, filetime::FileTime::from_system_time(expires));

    println!("SSH login enabled for {} on {} for {}", user, host, args[1]);
}
