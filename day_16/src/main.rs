use std::str::FromStr;
use std::usize;

const FILE_PATH: &str = "test.txt";

#[derive(Debug, Copy, Clone)]
enum SubPacketLength {
    Bits(usize),
    Count(usize),
}

#[derive(Debug, Clone)]
enum PacketType {
    Literal(u64), // 4
    Operation { type_id: u8, packets: Vec<Packet> },
}

#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    packet_type: PacketType,
}

impl SubPacketLength {
    fn get_length(s: &str, len: usize) -> Result<usize, String> {
        println!("length in {}", &s[..=len]);
        let res = s
            .get(1..=len)
            .map(|r| u32::from_str_radix(r, 2).map_err(|e| format!("Invalid packet length: {}", e)))
            .ok_or_else(|| format!("Could not get packet length from {} ({})", s, len))??
            as usize;
        Ok(res)
    }
}

impl FromStr for SubPacketLength {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .get(0..=0)
            .ok_or_else(|| format!("Could not get length type id from {}", s))?
        {
            "0" => Ok(Self::Bits(Self::get_length(s, 15)?)),
            "1" => Ok(Self::Count(Self::get_length(s, 11)?)),
            v => Err(format!("Invalid Length type id {}", v)),
        }
    }
}

impl Packet {
    fn parse(s: &str) -> Result<(Self, usize), String> {
        let mut index = 6;
        let version = s
            .get(0..3)
            .map(|r| u8::from_str_radix(r, 2).map_err(|e| format!("Invalid version: {}", e)))
            .ok_or_else(|| format!("Could not retrieve version from {}", s))??;
        let packet_type = match s
            .get(3..6)
            .map(|r| u8::from_str_radix(r, 2).map_err(|e| format!("Invalid packet type: {}", e)))
            .ok_or_else(|| format!("Could not retrieve packet_type from {}", s))??
        {
            4 => {
                let mut buff = String::new();
                while let Some(s) = s.get(index..index + 5) {
                    buff = format!("{}{}", buff, &s[1..]);
                    index += 5;
                    if &s[0..=0] == "0" {
                        break;
                    }
                }
                PacketType::Literal(
                    u64::from_str_radix(&buff, 2)
                        .map_err(|e| format!("Invalid literal value {}", e))?,
                )
            }
            type_id => {
                let packet_length = SubPacketLength::from_str(&s[index..])?;
                index += 1;
                let packets = match packet_length {
                    SubPacketLength::Bits(len) => {
                        println!("Found bit len {}", len);
                        index += 15;
                        let mut packets = Vec::new();
                        let l = index + len;
                        while index < l {
                            println!("index = {}, len = {}", index, l);
                            let packet_str = &s[index..l];
                            let (packet, delta) = Self::parse(packet_str)?;
                            println!("Found packet {:?} and delta {}", packet, delta);
                            packets.push(packet);
                            index += delta;
                        }
                        index += len;
                        packets
                    }
                    SubPacketLength::Count(len) => {
                        println!("Found count len {}", len);
                        index += 11;
                        let mut packets = Vec::new();
                        for _ in 0..len {
                            println!("index = {}", index);
                            let packet_str = &s[index..];
                            let (packet, delta) = Self::parse(packet_str)?;
                            println!("Found packet {:?} and delta {}", packet, delta);
                            index += delta;
                            packets.push(packet);
                        }
                        packets
                    }
                };
                PacketType::Operation { type_id, packets }
            }
        };
        Ok((
            Self {
                version,
                packet_type,
            },
            index,
        ))
    }

    fn version_sum(&self) -> usize {
        let mut res = self.version as usize;
        if let PacketType::Operation { packets, .. } = &self.packet_type {
            res += packets.iter().map(Self::version_sum).sum::<usize>();
        }
        res
    }
}

fn main() {
    let input = std::fs::read_to_string(FILE_PATH).unwrap();
    for (i, line) in input.lines().enumerate() {
        let line = line
            .chars()
            .map(|c| match c {
                '0' => "0000",
                '1' => "0001",
                '2' => "0010",
                '3' => "0011",
                '4' => "0100",
                '5' => "0101",
                '6' => "0110",
                '7' => "0111",
                '8' => "1000",
                '9' => "1001",
                'A' => "1010",
                'B' => "1011",
                'C' => "1100",
                'D' => "1101",
                'E' => "1110",
                'F' => "1111",
                _ => panic!("Invalid char {}", c),
            })
            .collect::<String>();
        println!("{}", line);
        let (packet, _) = Packet::parse(&line).unwrap();
        println!("{:#?}", packet);
        println!("Line {}: {}", i, packet.version_sum());
    }
}
