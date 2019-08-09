use std::{error, io};

fn prompt<T, E>(p: &str) -> io::Result<T>
  where
    T: FromStr<Err = E>,
    E: error::Error,
{
    let mut stdout = io::stdout();
    stdout.write_all(&p.as_bytes()[..])?;
    stdout.flush()?;
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf)?;
    Ok(<T as FromStr>::from_str(&buf)?);
}

fn run_day(day: u32) -> io::Result<()> {
    unimplemented!()
}
