use std::{
    collections::HashMap,
    io,
};

pub fn run() -> io::Result<()> {
    fn get_line_bytes() -> io::Result<impl Iterator<Item = Vec<u8>>> {
        Ok(super::super::get_lines("2.txt")?.map(|s| s.into_bytes()))
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
        let ids: Vec<_> = get_line_bytes()?.collect();
        'lv0: for i in 0..ids.len() {
            'lv1: for j in 0..i {
                let a = &ids[i];
                let b = &ids[j];
                let mut diff = None;
                for i in 0..a.len() {
                    if a[i] != b[i] {
                        if diff != None {
                            continue 'lv1;
                        }
                        diff = Some(i);
                    }
                }
                match diff {
                    Some(i) => {
                        let mut common = Vec::with_capacity(a.len() - 1);
                        for j in 0..a.len() {
                            if j < i {
                                common.push(a[j]);
                            } else if j > i {
                                common.push(a[j - 1]);
                            }
                        }
                        println!(
                            "Common letters are {}",
                            String::from_utf8_lossy(&common));
                        break 'lv0;
                    }
                    None => {}
                }
            }
        }
    }
    Ok(())
}
