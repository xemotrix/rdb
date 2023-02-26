use byteorder::{ByteOrder, BigEndian};
use crate::dal::consts::PAGE_NUM_SIZE;

pub const META_PAGE_NUM: u64 = 0;

#[derive(Default, Debug)]
pub struct Meta {
    pub freelist_page: u64,
}

impl Meta {
    pub fn new() -> Meta {
        Default::default()
    }

    pub fn serialize(&self, buf: &mut [u8]) {
        let mut pos = 0;
        BigEndian::write_u64(&mut buf[pos..], self.freelist_page);
        pos += PAGE_NUM_SIZE;
    }

    pub fn deserialize(&mut self, buf: &[u8]) {
        let mut pos = 0;
        self.freelist_page = BigEndian::read_u64(&buf[pos..]);
        pos += PAGE_NUM_SIZE;
    }

}
