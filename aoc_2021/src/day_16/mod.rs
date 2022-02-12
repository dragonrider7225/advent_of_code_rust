use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Index,
    sync::atomic::AtomicUsize,
};

#[derive(Clone, Copy, Debug)]
struct LeftoverBits {
    bits: [bool; 4],
    idx: usize,
}

impl LeftoverBits {
    fn len(&self) -> usize {
        4 - self.idx
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl LeftoverBits {
    fn take_bits(&mut self, num_bits: usize) -> Option<&[bool]> {
        match num_bits {
            0 => Some(&[]),
            1..=4 if self.idx + num_bits <= 4 => {
                self.idx += num_bits;
                Some(&self.bits[(self.idx - num_bits)..self.idx])
            }
            _ => None,
        }
    }
}

impl Default for LeftoverBits {
    fn default() -> Self {
        Self {
            bits: [false; 4],
            idx: 4,
        }
    }
}

impl Index<usize> for LeftoverBits {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bits[self.idx + index]
    }
}

impl TryFrom<u8> for LeftoverBits {
    type Error = io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let mut ret = Self::default();
        match value {
            b'0' => {}
            b'1' => ret.bits[3] = true,
            b'2' => ret.bits[2] = true,
            b'3' => ret.bits[2..4].copy_from_slice(&[true, true]),
            b'4' => ret.bits[1] = true,
            b'5' => ret.bits[1..4].copy_from_slice(&[true, false, true]),
            b'6' => ret.bits[1..3].copy_from_slice(&[true, true]),
            b'7' => ret.bits[1..4].copy_from_slice(&[true; 3]),
            b'8' => ret.bits[0] = true,
            b'9' => ret.bits = [true, false, false, true],
            b'A' => ret.bits = [true, false, true, false],
            b'B' => ret.bits = [true, false, true, true],
            b'C' => ret.bits = [true, true, false, false],
            b'D' => ret.bits = [true, true, false, true],
            b'E' => ret.bits = [true, true, true, false],
            b'F' => ret.bits = [true, true, true, true],
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid hex digit {:?}", value),
            ))?,
        }
        ret.idx = 0;
        Ok(ret)
    }
}

#[derive(Clone, Debug)]
enum Payload {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    Literal(u64),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

impl Payload {
    fn iter<'a>(&'a self) -> <&'a Payload as IntoIterator>::IntoIter {
        IntoIterator::into_iter(self)
    }

    fn value(&self) -> u64 {
        match self {
            Self::Sum(packets) => packets.iter().map(Packet::value).sum(),
            Self::Product(packets) => packets.iter().map(Packet::value).product(),
            Self::Minimum(packets) => packets.iter().map(Packet::value).min().unwrap(),
            Self::Maximum(packets) => packets.iter().map(Packet::value).max().unwrap(),
            Self::Literal(value) => *value,
            Self::GreaterThan(packets) => (packets[0].value() > packets[1].value()).into(),
            Self::LessThan(packets) => (packets[0].value() < packets[1].value()).into(),
            Self::EqualTo(packets) => (packets[0].value() == packets[1].value()).into(),
        }
    }
}

impl Display for Payload {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sum(packets) => {
                write!(f, "{}", packets[0])?;
                for packet in packets.iter().skip(1) {
                    write!(f, " + {}", packet)?;
                }
                Ok(())
            }
            Self::Product(packets) => {
                write!(f, "{}", packets[0])?;
                for packet in packets.iter().skip(1) {
                    write!(f, " * {}", packet)?;
                }
                Ok(())
            }
            Self::Minimum(packets) => {
                write!(f, "min({}", packets[0])?;
                for packet in packets.iter().skip(1) {
                    write!(f, ", {}", packet)?;
                }
                write!(f, ")")
            }
            Self::Maximum(packets) => {
                write!(f, "max({}", packets[0])?;
                for packet in packets.iter().skip(1) {
                    write!(f, ", {}", packet)?;
                }
                write!(f, ")")
            }
            Self::Literal(value) => write!(f, "{}", value),
            Self::GreaterThan(packets) => {
                write!(f, "{} > {}", packets[0], packets[1])
            }
            Self::LessThan(packets) => {
                write!(f, "{} < {}", packets[0], packets[1])
            }
            Self::EqualTo(packets) => {
                write!(f, "{} == {}", packets[0], packets[1])
            }
        }
    }
}

impl<'a> IntoIterator for &'a Payload {
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = ::std::slice::Iter<'a, Packet>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Payload::Sum(packets)
            | Payload::Product(packets)
            | Payload::Minimum(packets)
            | Payload::Maximum(packets)
            | Payload::GreaterThan(packets)
            | Payload::LessThan(packets)
            | Payload::EqualTo(packets) => packets.iter(),
            Payload::Literal(_) => IntoIterator::into_iter(&[]),
        }
    }
}

#[derive(Clone, Debug)]
struct Packet {
    version: u32,
    payload: Payload,
}

impl Packet {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        fn read_impl(
            mut bits: LeftoverBits,
            input: &mut dyn BufRead,
        ) -> io::Result<(Packet, LeftoverBits, usize)> {
            static DEPTH: AtomicUsize = AtomicUsize::new(0);

            macro_rules! deepen {
                () => {
                    DEPTH.fetch_add(1, ::std::sync::atomic::Ordering::Relaxed);
                };
            }
            macro_rules! surface {
                () => {
                    DEPTH.fetch_sub(1, ::std::sync::atomic::Ordering::Relaxed);
                };
            }
            macro_rules! println_with_depth {
                ($($args:tt)*) => {{
                    // for _ in 0..DEPTH.load(::std::sync::atomic::Ordering::Relaxed) {
                    //     print!("  ");
                    // }
                    // println!($($args)*);
                }};
            }
            macro_rules! read_bits {
                () => {
                    read_bits!(false)
                };
                ($print:expr) => {{
                    let print = $print;
                    let mut buf = [0];
                    if 0 != input.read(&mut buf)? {
                        let new_bits = LeftoverBits::try_from(buf[0]);
                        if print {
                            print!("{:?}", new_bits);
                        }
                        new_bits
                    } else {
                        Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""))
                    }
                }};
            }
            macro_rules! read_u32 {
                ($num_bits:expr $(,$args:tt)*) => { read_t!($num_bits; u32 $(,$args)*) };
            }
            macro_rules! read_t {
                ($num_bits:expr; $t:ident) => {
                    read_t!($num_bits; $t, false)
                };
                ($num_bits:expr; $t:ident, $print:expr) => {{
                    let print = $print;
                    let mut value = 0;
                    let mut remaining_bits = $num_bits;
                    while remaining_bits > bits.len() {
                        remaining_bits -= bits.len();
                        bits.take_bits(bits.len())
                            .unwrap()
                            .into_iter()
                            .copied()
                            .for_each(|bit| {
                                let bit = $t::from(bit);
                                if print {
                                    print!("{}", bit);
                                }
                                value = value * 2 + bit
                            });
                        bits = read_bits!()?;
                    }
                    bits.take_bits(remaining_bits)
                        .unwrap()
                        .into_iter()
                        .copied()
                        .for_each(|bit| {
                            let bit = $t::from(bit);
                            if print {
                                print!("{}", bit);
                            }
                            value = value * 2 + bit
                        });
                    value
                }};
            }

            deepen!();
            println_with_depth!("Parsing packet");
            let version = read_u32!(3);
            println_with_depth!("Version is {}", version);
            let type_id = read_u32!(3);
            println_with_depth!("Type id is {}", type_id);
            let (payload, payload_width) = match type_id {
                4 => {
                    let mut value = 0;
                    let mut payload_width = 0;
                    while {
                        if bits.is_empty() {
                            bits = read_bits!()?;
                        }
                        bits.take_bits(1).unwrap()[0]
                    } {
                        value = value * 16 + read_t!(4; u64);
                        payload_width += 5;
                    }
                    value = value * 16 + read_t!(4; u64);
                    payload_width += 5;
                    (Payload::Literal(value), payload_width)
                }
                type_id => {
                    if bits.is_empty() {
                        bits = read_bits!()?;
                    }
                    let type_length_id = bits.take_bits(1).unwrap()[0];
                    println_with_depth!("Type length ID is {}", u32::from(type_length_id));
                    let (packets, payload_width) = if type_length_id {
                        let num_packets = read_u32!(11);
                        println_with_depth!("Payload contains {} packets", num_packets);
                        let (packets, leftovers, payload_width) = (0..num_packets).try_fold(
                            (vec![], bits, 0),
                            |(mut acc, bits, width), _| {
                                let (packet, bits, packet_width) = read_impl(bits, input)?;
                                acc.push(packet);
                                io::Result::Ok((acc, bits, width + packet_width))
                            },
                        )?;
                        bits = leftovers;
                        (packets, 12 + payload_width)
                    } else {
                        let payload_width = read_u32!(15) as usize;
                        println_with_depth!(
                            "Payload contains packets with total width {}",
                            payload_width
                        );
                        let mut remaining_length = payload_width;
                        let mut packets = vec![];
                        while remaining_length > 0 {
                            let (packet, leftovers, packet_width) = read_impl(bits, input)?;
                            remaining_length -= packet_width;
                            packets.push(packet);
                            bits = leftovers;
                        }
                        (packets, 16 + payload_width)
                    };
                    let payload = match type_id {
                        0 => Payload::Sum(packets),
                        1 => Payload::Product(packets),
                        2 => Payload::Minimum(packets),
                        3 => Payload::Maximum(packets),
                        5 => Payload::GreaterThan(packets),
                        6 => Payload::LessThan(packets),
                        7 => Payload::EqualTo(packets),
                        _ => unreachable!(),
                    };
                    (payload, payload_width)
                }
            };
            println_with_depth!("Payload is {:?}", payload);
            println_with_depth!("Packet width is {}", 6 + payload_width);
            surface!();
            Ok((Packet { version, payload }, bits, 6 + payload_width))
        }

        Ok(read_impl(LeftoverBits::default(), input)?.0)
    }
}

impl Packet {
    fn version_sum(&self) -> u32 {
        self.version + self.payload.iter().map(Packet::version_sum).sum::<u32>()
    }

    fn value(&self) -> u64 {
        self.payload.value()
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(v{}: {})", self.version, self.payload)
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let root = Packet::read(input)?;
    println!("{}", root);
    Ok(root.version_sum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<u64> {
    let root = Packet::read(input)?;
    Ok(root.value())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 16 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2021_16.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 16 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2021_16.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    #[ignore]
    fn test_part1_a() -> io::Result<()> {
        let expected = 6;
        let actual = part1(&mut Cursor::new("D2FE28"))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part1_b() -> io::Result<()> {
        let expected = 9;
        let actual = part1(&mut Cursor::new("38006F45291200"))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part1_c() -> io::Result<()> {
        let expected = 14;
        let actual = part1(&mut Cursor::new("EE00D40C823060"))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part1_d() -> io::Result<()> {
        let expected = 16;
        let actual = part1(&mut Cursor::new("8A004A801A8002F478"))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part1_e() -> io::Result<()> {
        let expected = 12;
        let actual = part1(&mut Cursor::new("620080001611562C8802118E34"))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part1_f() -> io::Result<()> {
        let expected = 23;
        let actual = part1(&mut Cursor::new("C0015000016115A2E0802F182340"))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part1_g() -> io::Result<()> {
        let expected = 31;
        let actual = part1(&mut Cursor::new("A0016C880162017C3686B18A3D4780"))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2_a() -> io::Result<()> {
        let expected = 3;
        let actual = part2(&mut Cursor::new("C200B40A82"))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2_b() -> io::Result<()> {
        let expected = 54;
        let actual = part2(&mut Cursor::new("04005AC33890"))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2_c() -> io::Result<()> {
        let expected = 7;
        let actual = part2(&mut Cursor::new("880086C3E88112"))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2_d() -> io::Result<()> {
        let expected = 9;
        let actual = part2(&mut Cursor::new("CE00C43D881120"))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2_e() -> io::Result<()> {
        let expected = 1;
        let actual = part2(&mut Cursor::new("D8005AC2A8F0"))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2_f() -> io::Result<()> {
        let expected = 0;
        let actual = part2(&mut Cursor::new("F600BC2D8F"))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2_g() -> io::Result<()> {
        let expected = 0;
        let actual = part2(&mut Cursor::new("9C005AC2F8F0"))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2_h() -> io::Result<()> {
        let expected = 1;
        let actual = part2(&mut Cursor::new("9C0141080250320F1802104A08"))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
