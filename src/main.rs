mod models;
fn main() {
    let nonce = String::from("000");
    let mut blockchain = models::blockchain::Blockchain::new();
    
    models::blockchain::Blockchain::add_block(&mut blockchain, nonce);
}
