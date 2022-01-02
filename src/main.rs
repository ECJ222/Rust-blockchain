mod models;
fn main() {
    let difficulty = 1;
    let mut blockchain = models::blockchain::Blockchain::new(difficulty);

    models::blockchain::Blockchain::add_block(&mut blockchain);
}
