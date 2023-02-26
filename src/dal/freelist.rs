

use byteorder::{ByteOrder, BigEndian};
use crate::dal::consts::PAGE_NUM_SIZE;

pub struct Freelist {
    pub max_page: u64,
    pub released_pages: Vec<u64>,
}

impl Freelist {
    pub fn new() -> Freelist {
        Freelist {
            max_page: 0,
            released_pages: Vec::new(),
        }
    }

    pub fn get_next_page(&mut self) -> u64 {
        match self.released_pages.pop() {
            Some(page) => page,
            None => {
                self.max_page += 1;
                self.max_page
            }
        }
    }

    pub fn release_page(&mut self, page_num: u64) {
        self.released_pages.push(page_num);
    }

    pub fn serialize(&self, buf: &mut [u8]) -> Vec<u8> {
        let mut pos = 0;

        BigEndian::write_u16(&mut buf[pos..], self.max_page as u16);
        pos += 2;

        BigEndian::write_u16(&mut buf[pos..], self.released_pages.len() as u16);
        pos += 2;

        for page in &self.released_pages {
            BigEndian::write_u64(&mut buf[pos..], *page);
            pos += PAGE_NUM_SIZE;
        }
        buf.to_vec()
    }

    pub fn deserialize(&mut self, buf: &[u8]) {
        let mut pos = 0;

        self.max_page = BigEndian::read_u16(&buf[pos..]) as u64;
        pos += 2;

        let released_pages_len = BigEndian::read_u16(&buf[pos..]) as usize;
        pos += 2;

        for _ in 0..released_pages_len {
            self.released_pages.push(BigEndian::read_u64(&buf[pos..]));
            pos += PAGE_NUM_SIZE;
        }
    }
}
