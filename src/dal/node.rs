use crate::dal::consts::PAGE_NUM_SIZE;
use crate::dal::Dal;
use crate::util::copy_data;
use byteorder::{BigEndian, ByteOrder};

pub struct Item {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

impl Item {
    pub fn new(key: Vec<u8>, value: Vec<u8>) -> Item {
        Item { key, value }
    }
}

pub struct Node<'a> {
    pub dal: &'a Dal,
    pub page_num: u64,
    pub items: Vec<Item>,
    pub child_nodes: Vec<u64>,
}


impl<'a> Node<'a> {
    pub fn is_leaf(&self) -> bool {
        self.child_nodes.is_empty()
    }

    pub fn serialize(&self, buf: &mut [u8]) -> Vec<u8> {
        let mut left_pos = 0;
        let mut right_pos = buf.len() - 1;

        let is_leaf = self.is_leaf();

        buf[left_pos] = is_leaf as u8;
        left_pos += 1;

        BigEndian::write_u16(&mut buf[left_pos..], self.items.len() as u16);
        left_pos += 2;

        for (i, item) in self.items.iter().enumerate() {
            if !is_leaf {
                let child_node = self.child_nodes[i];
                BigEndian::write_u64(&mut buf[left_pos..], child_node);
                left_pos += PAGE_NUM_SIZE;
            }

            let klen = item.key.len();
            let vlen = item.value.len();

            let offset = (right_pos - klen - vlen - 2) as u16;
            BigEndian::write_u16(&mut buf[left_pos..], offset);
            left_pos += 2;

            right_pos -= vlen;
            copy_data(&mut buf[right_pos..], &item.value);

            right_pos -= 1;
            buf[right_pos] = vlen as u8;

            right_pos -= klen;
            copy_data(&mut buf[right_pos..], &item.key);

            right_pos -= 1;
            buf[right_pos] = klen as u8;
        }

        if !is_leaf {
            let last_child_node = self.child_nodes.last().unwrap().to_owned();

            BigEndian::write_u64(&mut buf[left_pos..], last_child_node)

        }

        buf.to_vec()
    }

    pub fn deserialize(&mut self, buf: &'a [u8]) {
        let mut left_pos = 0;

        let is_leaf = buf[0] != 0;

        let items_count = BigEndian::read_u16(&buf[1..3]);

        left_pos += 3;

        for i in 0..items_count {
            if !is_leaf {
                let page_num = BigEndian::read_u64(&buf[left_pos..]);
                left_pos += PAGE_NUM_SIZE;
                self.child_nodes.push(page_num);
            }

            let mut offset = BigEndian::read_u16(&buf[left_pos..]) as usize;
            left_pos += 2;

            let klen = buf[offset] as u16;
            offset += 1;

            let key = &buf[offset..offset + klen as usize];
            offset += klen as usize;

            let vlen = buf[offset] as u16;
            offset += 1;

            let value = &buf[offset..offset + vlen as usize];
            offset += vlen as usize;

            self.items.push(Item::new(key.to_vec(), value.to_vec()));
        }

    }
}

