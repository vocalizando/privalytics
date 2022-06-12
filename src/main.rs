use crate::saving::{EntryLoad, EntrySave};
use crate::structures::analytics::{Entry, Metadata};

mod structures;
mod saving;

fn main() {
    println!("Hello World!");
    let test = Entry {
        metadata: Metadata {
            date: 73812738921,
            duid: "djkaskdasj".to_string(),
            page: None,
            uid: None
        },
        data: Default::default()
    };
    println!("save - {:?}", test.save("./owo.bson").unwrap());

    println!("load - {:?}", Entry::load("./owo.bson").unwrap());
}
