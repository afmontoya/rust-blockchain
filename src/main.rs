mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut my_blockchain = Blockchain::new();

    my_blockchain.add_block("Transaction: Alice pays Bob 10 BTC".to_string());
    my_blockchain.add_block("Transaction: Bob pays Charlie 5 BTC".to_string());

    for block in &my_blockchain.chain {
        println!("{:#?}", block);
    }

    println!("Is blockchain valid? {}", my_blockchain.is_valid_chain());
}
