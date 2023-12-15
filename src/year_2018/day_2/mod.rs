use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn run() -> io::Result<()> {
    fn get_line_bytes() -> io::Result<Vec<Vec<u8>>> {
        BufReader::new(File::open("2018_02.txt")?)
            .lines()
            .map(|line| line.map(|line| line.into_bytes()))
            .collect::<io::Result<Vec<_>>>()
    }
    {
        // Part 1
        let mut double = 0u32;
        let mut triple = 0u32;
        for id in get_line_bytes()? {
            let mut byte_freqs = HashMap::new();
            for byte in id {
                *byte_freqs.entry(byte).or_default() += 1;
            }
            let mut counted_double = false;
            let mut counted_triple = false;
            for (_, freq) in byte_freqs {
                match freq {
                    2 if !counted_double => {
                        double += 1;
                        counted_double = true;
                    }
                    3 if !counted_triple => {
                        triple += 1;
                        counted_triple = true;
                    }
                    _ => {}
                }
                if counted_double && counted_triple {
                    break;
                }
            }
        }
        println!("Checksum is {}", double * triple);
    }
    {
        // Part 2
        let ids = get_line_bytes()?;
        'lv0: for i in 0..ids.len() {
            'lv1: for j in 0..i {
                let a = &ids[i];
                let b = &ids[j];
                let mut diff = None;
                for i in 0..a.len() {
                    if a[i] != b[i] {
                        if diff.is_some() {
                            continue 'lv1;
                        }
                        diff = Some(i);
                    }
                }
                if let Some(i) = diff {
                    let mut common = Vec::with_capacity(a.len() - 1);
                    for j in 0..a.len() {
                        match j.cmp(&i) {
                            Ordering::Less => common.push(a[j]),
                            Ordering::Equal => {}
                            Ordering::Greater => common.push(a[j - 1]),
                        }
                    }
                    println!("Common letters are {}", String::from_utf8_lossy(&common));
                    break 'lv0;
                }
            }
        }
    }
    Ok(())
}
