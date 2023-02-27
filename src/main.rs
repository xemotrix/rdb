mod dal;
mod util;

use dal::{Dal, Page};

fn copy_data(page: &mut Page, data: &str) {
    for (i, b) in data.bytes().enumerate() {
        page.data[i] = b;
    }
}

fn main() {
    let mut dal = Dal::new("db.db");

    let mut page = dal.allocate_empty_page();
    page.num = dal.freelist.get_next_page();

    copy_data(&mut page, "some data");

    dal.write_page(&page)
        .unwrap_or_else(|err| panic!("Error writing page: {err}"));
    dal.write_freelist()
        .unwrap_or_else(|err| panic!("Error writing freelist: {err}"));

    drop(dal);

    let mut dal2 = Dal::new("db.db");

    let mut page = dal2.allocate_empty_page();
    page.num = dal2.freelist.get_next_page();
    copy_data(&mut page, "some_data2");
    dal2.write_page(&page)
        .unwrap_or_else(|err| panic!("Error writing page: {err}"));

    let page_num = dal2.freelist.get_next_page();
    dal2.freelist.release_page(page_num);

    dal2.write_freelist()
        .unwrap_or_else(|err| panic!("Error writing freelist: {err}"));
}
