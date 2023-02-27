mod consts;
mod freelist;
mod meta;
mod node;

use anyhow::Result;
use freelist::Freelist;
use meta::{Meta, META_PAGE_NUM};
use std::fs::File;
use std::io::{
    prelude::{Read, Write},
    Seek, SeekFrom,
};
use std::path::Path;

extern crate page_size;

#[derive(Debug)]
pub struct Page {
    pub num: u64,
    pub data: Vec<u8>,
}

pub struct Dal {
    pub file: File,
    pub page_size: usize,
    pub freelist: Freelist,
    pub meta: Meta,
}

impl Dal {
    pub fn new(path: &str) -> Dal {
        let file_existed = Path::new(path).exists();

        let file = File::options()
            .write(true)
            .read(true)
            .create(true)
            .open(path)
            .unwrap_or_else(|err| panic!("Error opening file {path}: {err}"));

        let mut dal = Dal {
            file,
            page_size: page_size::get(),
            freelist: Freelist::new(),
            meta: Meta::new(),
        };

        match file_existed {
            true => {
                dal.meta = dal
                    .read_meta()
                    .unwrap_or_else(|err| panic!("Error reading meta: {err}"));
                dal.freelist = dal
                    .read_freelist()
                    .unwrap_or_else(|err| panic!("Error reading freelist: {err}"));
            }
            false => {
                dal.freelist = Freelist::new();
                dal.meta.freelist_page = dal.freelist.get_next_page();
                dal.write_freelist()
                    .unwrap_or_else(|err| panic!("Error writing freelist: {err}"));
                dal.write_meta()
                    .unwrap_or_else(|err| panic!("Error writing meta: {err}"));
            }
        }
        dal
    }

    pub fn allocate_empty_page(&self) -> Page {
        Page {
            num: 0,
            data: vec![0; self.page_size],
        }
    }

    pub fn read_page(&mut self, page_num: u64) -> Result<Page> {
        let mut p = self.allocate_empty_page();
        let offset = page_num * self.page_size as u64;
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(&mut p.data)?;
        Ok(p)
    }

    pub fn write_page(&mut self, page: &Page) -> Result<()> {
        let offset = page.num * self.page_size as u64;
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all(&page.data)?;
        Ok(())
    }

    fn write_meta(&mut self) -> Result<Page> {
        let mut page = self.allocate_empty_page();
        page.num = META_PAGE_NUM;
        self.meta.serialize(&mut page.data);
        self.write_page(&page)?;
        Ok(page)
    }

    fn read_meta(&mut self) -> Result<Meta> {
        let page = self.read_page(META_PAGE_NUM)?;
        let mut meta = Meta::new();
        meta.deserialize(&page.data);
        Ok(meta)
    }

    pub fn write_freelist(&mut self) -> Result<Page> {
        let mut page = self.allocate_empty_page();
        page.num = self.meta.freelist_page;
        self.freelist.serialize(&mut page.data);
        self.write_page(&page)?;
        self.meta.freelist_page = page.num;
        Ok(page)
    }

    fn read_freelist(&mut self) -> Result<Freelist> {
        let page = self.read_page(self.meta.freelist_page)?;
        let mut freelist = Freelist::new();
        freelist.deserialize(&page.data);
        Ok(freelist)
    }
}
