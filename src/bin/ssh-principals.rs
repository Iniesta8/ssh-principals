use std::{env, fs, path::Path, time::SystemTime};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return;
    }

    let user = &args[1];

    let host = match hostname::get() {
        Ok(h) => h.into_string().unwrap(),
        Err(_) => return,
    };

    let principal = format!("{}@{}", user, host);

    let path = Path::new("/var/lib/ssh-principals/enabled").join(&principal);

    let meta = match fs::metadata(&path) {
        Ok(m) => m,
        Err(_) => return,
    };

    let expiry = match meta.modified() {
        Ok(t) => t,
        Err(_) => return,
    };

    if SystemTime::now() <= expiry {
        println!("{}", principal);
    }
}
