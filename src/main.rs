use blockchainlib::*;

fn main() {
    let block = Block::new(13, now(), vec![0; 32], 0, "Initial Block!".to_owned());

    println!("{:#?}", &block);
}
