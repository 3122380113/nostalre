//! Packets Ragnarok Solo's server sends at packet versions the generated `packets` crate does not list them for.

use std::any::Any;

use packets::packets::Packet;

/// ZC_HP_INFO (0x0977): a monster's HP for its bar. rAthena sends it from packet version 20120404; Solo's
/// server sends it to every version.
#[derive(Clone, Debug)]
pub struct PacketZcHpInfo {
    pub raw: Vec<u8>,
    pub gid: u32,
    pub hp: i32,
    pub max_hp: i32,
}

impl PacketZcHpInfo {
    pub const ID: [u8; 2] = [0x77, 0x09];
    pub const LEN: usize = 14;

    /// `buffer` holds exactly [`Self::LEN`] bytes, starting with [`Self::ID`].
    pub fn from(buffer: &[u8]) -> Self {
        let u32_at = |at: usize| {
            u32::from_le_bytes([buffer[at], buffer[at + 1], buffer[at + 2], buffer[at + 3]])
        };
        Self {
            raw: buffer.to_vec(),
            gid: u32_at(2),
            hp: u32_at(6) as i32,
            max_hp: u32_at(10) as i32,
        }
    }
}

impl Packet for PacketZcHpInfo {
    fn id(&self, _packetver: u32) -> &str {
        "0x7709"
    }
    fn name(&self) -> &str {
        "PacketZcHpInfo"
    }
    fn base_len(&self, _packetver: u32) -> usize {
        Self::LEN
    }
    fn display(&self) {
        println!("{self:?}");
    }
    fn debug(&self) {
        println!("{self:?}");
    }
    fn pretty_debug(&self) {
        println!("{self:#?}");
    }
    fn fill_raw(&mut self) {}
    fn fill_raw_with_packetver(&mut self, _packetver: Option<u32>) {}
    fn raw(&self) -> &Vec<u8> {
        &self.raw
    }
    fn raw_mut(&mut self) -> &mut Vec<u8> {
        &mut self.raw
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn layout(&self, _packetver: u32) -> Vec<(String, usize, usize)> {
        [("packet_id", 0, 2), ("gid", 2, 4), ("hp", 6, 4), ("max_hp", 10, 4)]
            .map(|(name, offset, len)| (name.to_string(), offset, len))
            .to_vec()
    }
    fn to_json(&self, _packetver: u32) -> String {
        format!(
            r#"{{"gid":{},"hp":{},"max_hp":{}}}"#,
            self.gid, self.hp, self.max_hp
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_hp_info() {
        let mut raw = vec![0x77, 0x09];
        raw.extend(110_015_601u32.to_le_bytes());
        raw.extend(45i32.to_le_bytes());
        raw.extend(60i32.to_le_bytes());
        let packet = PacketZcHpInfo::from(&raw);
        assert_eq!(
            (packet.gid, packet.hp, packet.max_hp),
            (110_015_601, 45, 60)
        );
    }
}
