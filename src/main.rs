use anyhow::{bail, Result};
use fs4::FileExt;
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

fn ensure_datafiles(whom: u64, name: &str) -> Result<PathBuf> {
    let p = PathBuf::from(format!("/tmp/locker-{name}"));

    /*
     * Try to create the directory first.  If we are racing with another
     * process, it may exist already.
     */
    if let Err(e) = std::fs::create_dir(&p) {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            bail!("creating {p:?}: {e}");
        }
    }

    let lf = {
        let mut lf = p.clone();
        lf.push("lock");
        lf
    };

    /*
     * Try to create the lock file.  If we are racing, it might exist already at
     * which point we'll just open it.
     */
    let f = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(&lf)?;

    /*
     * Now that we are sure the file exists, try to lock it.
     */
    println!("[{whom}] locking file {lf:?}...");
    let start = Instant::now();
    f.lock_exclusive()?;
    let latency = Instant::now().saturating_duration_since(start);

    println!("[{whom}] now has the lock (after {} ms)", latency.as_millis());
    std::thread::yield_now();

    let dbf = {
        let mut dbf = p.clone();
        dbf.push("datafile.txt");
        dbf
    };

    if dbf.exists() {
        let contents = std::fs::read_to_string(&dbf)?;
        println!("[{whom}] got pre-existing contents: {contents}");
        return Ok(p);
    }

    /*
     * Gosh, it's a lot of work creating a database!
     */
    std::thread::sleep(Duration::from_secs(5));

    let contents =
        format!("database created by {whom} in pid {}", std::process::id());
    std::fs::write(&dbf, &contents)?;

    println!("[{whom}] created database with contents: {contents}");

    Ok(p)
}

fn main() -> Result<()> {
    let Some(name) = std::env::args().nth(1) else {
        bail!("what name shall we use for the temporary directory?");
    };
    let Some(whom) = std::env::args().nth(2) else {
        bail!("by what number is this worker to be known?");
    };
    let Ok(whom) = whom.parse::<u64>() else {
        bail!("{whom:?} is not a good worker number");
    };

    let p = ensure_datafiles(whom, &name)?;

    println!("[{whom}] database -> {p:?}");
    Ok(())
}
