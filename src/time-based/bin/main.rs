use std::{
    env, fs,
    path::Path,
    time::{Duration, SystemTime},
};

const MAX_AGE_SECS: u64 = 2 * 60 * 60; // 2 hours

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return;
    }

    let user = &args[1];
    let host = &args[2];
    let principal = format!("{}@{}", user, host);

    let path = Path::new("/var/lib/ssh-principals/approved").join(&principal);

    let meta = match fs::metadata(&path) {
        Ok(m) => m,
        Err(_) => return,
    };

    let modified = match meta.modified() {
        Ok(t) => t,
        Err(_) => return,
    };

    let age_ok = SystemTime::now()
        .duration_since(modified)
        .map(|age| age <= Duration::from_secs(MAX_AGE_SECS))
        .unwrap_or(false);

    if age_ok {
        println!("{}", principal);
    }
}
