use core::fmt;
use std::fs::File;
use std::io::Write;

use fake::faker::boolean::en::*;
use fake::faker::company::en::*;
use fake::faker::name::en::*;
use fake::{Dummy, Fake, Faker};

#[derive(Debug, Dummy)]
struct Interaction {
    #[dummy(faker = "FirstName()")]
    first_name: String,
    #[dummy(faker = "LastName()")]
    last_name: String,
    #[dummy(faker = "CompanyName()")]
    company: String,
    #[dummy(faker = "(Faker, 3..5)")]
    items: Vec<Item>,
    #[dummy(faker = "Boolean(70)")]
    paid: bool,
    #[dummy(faker = "Boolean(40)")]
    delay_shipping: bool,
}

#[derive(Debug, Dummy)]
pub struct Item {
    #[dummy(faker = "1..100")]
    product_id: usize,

    qty: u8,

    #[dummy(faker = "CompanyName()")]
    company: String,
}

fn main() {
    let mut file = File::create("short.txt").unwrap();

    for _ in 0..10000 {
        let interaction: Interaction = Faker.fake();

        file.write_fmt(format_args!("{:?},\n", interaction)).unwrap();
    }
}
