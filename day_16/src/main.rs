use std::str::FromStr;

const FILE_PATH: &str = "input.txt";

#[derive(Debug, Copy, Clone)]
enum SubPacketLength {
    Bits(usize),
    Count(usize),
}

#[derive(Debug, Clone)]
enum PacketType {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    GtrThan([Packet; 2]),
    LesserThan([Packet; 2]),
    EqTo([Packet; 2]),
}

#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    packet_type: Box<PacketType>,
}

impl SubPacketLength {
    fn get_length(s: &str, len: usize) -> Result<usize, String> {
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
            .chars()
            .next()
            .ok_or_else(|| format!("Could not get length type id from {}", s))?
        {
            '0' => Ok(Self::Bits(Self::get_length(s, 15)?)),
            '1' => Ok(Self::Count(Self::get_length(s, 11)?)),
            v => Err(format!("Invalid Length type id {}", v)),
        }
    }
}

impl Packet {
    fn packets_2d(packets: Vec<Self>) -> Result<[Self; 2], String> {
        packets
            .try_into()
            .map_err(|_| String::from("Expected exactly 2 sub packets"))
    }

    fn parse(binary: &str) -> Result<(Self, usize), String> {
        let mut index = 6;
        let version = binary
            .get(0..3)
            .map(|r| u8::from_str_radix(r, 2).map_err(|e| format!("Invalid version: {}", e)))
            .ok_or_else(|| format!("Could not retrieve version from {}", binary))??;
        let packet_type = match binary
            .get(3..6)
            .map(|r| u8::from_str_radix(r, 2).map_err(|e| format!("Invalid packet type: {}", e)))
            .ok_or_else(|| format!("Could not retrieve packet_type from {}", binary))??
        {
            4 => {
                let mut buff = String::new();
                while let Some(s) = binary.get(index..index + 5) {
                    buff = format!("{}{}", buff, &s[1..]);
                    index += 5;
                    if s.starts_with('0') {
                        break;
                    }
                }
                let value =
                    u64::from_str_radix(&buff, 2).map_err(|e| format!("Invalid literal {}", e))?;
                PacketType::Literal(value)
            }
            type_id => {
                let packet_length = SubPacketLength::from_str(&binary[index..])?;
                let packets = match packet_length {
                    SubPacketLength::Bits(len) => {
                        index += 16;
                        let mut packets = Vec::new();
                        let len = index + len;
                        while index < len {
                            let packet_str = &binary[index..len];
                            let (packet, delta) = Self::parse(packet_str)?;
                            packets.push(packet);
                            index += delta;
                        }
                        packets
                    }
                    SubPacketLength::Count(len) => {
                        index += 12;
                        (0..len).try_fold(vec![], |mut packets, _| {
                            let packet_str = &binary[index..];
                            let (packet, delta) = Self::parse(packet_str)?;
                            index += delta;
                            packets.push(packet);
                            Result::<_, String>::Ok(packets)
                        })?
                    }
                };
                match type_id {
                    0 => PacketType::Sum(packets),
                    1 => PacketType::Product(packets),
                    2 => PacketType::Min(packets),
                    3 => PacketType::Max(packets),
                    5 => PacketType::GtrThan(Self::packets_2d(packets)?),
                    6 => PacketType::LesserThan(Self::packets_2d(packets)?),
                    7 => PacketType::EqTo(Self::packets_2d(packets)?),
                    v => return Err(format!("{} is not a valid packet type id", v)),
                }
            }
        };
        let res = Self {
            version,
            packet_type: Box::new(packet_type),
        };
        Ok((res, index))
    }

    fn result(&self) -> u64 {
        match self.packet_type.as_ref() {
            PacketType::Literal(v) => *v,
            PacketType::Sum(packets) => packets.iter().map(Self::result).sum(),
            PacketType::Product(packets) => packets.iter().map(Self::result).product(),
            PacketType::Min(packets) => packets.iter().map(Self::result).min().unwrap_or(0),
            PacketType::Max(packets) => packets.iter().map(Self::result).max().unwrap_or(0),
            PacketType::GtrThan(packets) => (packets[0].result() > packets[1].result()) as u64,
            PacketType::LesserThan(packets) => (packets[0].result() < packets[1].result()) as u64,
            PacketType::EqTo(packets) => (packets[0].result() == packets[1].result()) as u64,
        }
    }

    fn version_sum(&self) -> u32 {
        u32::from(self.version)
            + match self.packet_type.as_ref() {
                PacketType::Sum(packets)
                | PacketType::Product(packets)
                | PacketType::Min(packets)
                | PacketType::Max(packets) => packets.iter().map(Self::version_sum).sum::<u32>(),
                PacketType::GtrThan(packets)
                | PacketType::LesserThan(packets)
                | PacketType::EqTo(packets) => packets.iter().map(Self::version_sum).sum::<u32>(),
                PacketType::Literal(_) => 0,
            }
    }
}

fn main() {
    let input = std::fs::read_to_string(FILE_PATH).unwrap();
    for (i, line) in input.lines().enumerate() {
        let line = line
            .chars()
            .map(|c| {
                format!(
                    "{:04b}",
                    u8::from_str_radix(c.to_string().as_str(), 16).unwrap()
                )
            })
            .collect::<String>();
        let (packet, _) = Packet::parse(&line).unwrap();
        println!(
            "Line {}: version sum = {}, result = {}",
            i,
            packet.version_sum(),
            packet.result()
        );
    }
}
