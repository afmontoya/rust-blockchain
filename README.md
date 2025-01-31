# 🚀 Rust Blockchain Prototype
A simple blockchain prototype implemented in Rust. This project demonstrates **block 
linking, cryptographic hashing (SHA-256), and basic chain validation**.

## 📌 Features
✅ Implements **SHA-256 hashing** for block security.  
✅ Dynamically adds new **blocks with transactions**.  
✅ Validates **blockchain integrity** to detect tampering.  
✅ Rust’s **safe memory management** and high performance.  

## ⚡ Installation & Setup
### 🔹 1. Clone the Repository
```sh
git clone https://github.com/afmontoya/rust-blockchain.git
cd rust-blockchain

2. Install Rust (if not installed)

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Verify installation:

rustc --version
cargo --version

3. Run the Blockchain

cargo run

How It Works
1 Each block contains:

Index: Block position in the chain.
Timestamp: When it was created.
Transaction Data: Example transactions (e.g., "Alice pays Bob 10 BTC").
Previous Hash: Links it to the previous block.
Hash: SHA-256 cryptographic hash of the block.
2️⃣New blocks are added dynamically, and the blockchain checks its integrity.

3️⃣ Example Output:
Block {
    index: 0,
    timestamp: 1738304591,
    data: "Genesis Block",
    prev_hash: "0",
    hash: "84c70fe872b086a794dd802f657442b11f6d861678093955b941444c9b2790bb",
}
Block {
    index: 1,
    timestamp: 1738304591,
    data: "Transaction: Alice pays Bob 10 BTC",
    prev_hash: "84c70fe872b086a794dd802f657442b11f6d861678093955b941444c9b2790bb",
    hash: "980cdeb08292cf65515e291d9946b01303fcad317e659e85902228f97eb93563",
}
Is blockchain valid? true

🔥 Planned Features (Future Improvements)
🚀 Proof of Work (Mining): Implementing difficulty-based mining.
🚀 Transaction System: Add real sender/receiver transactions.
🚀 Networking: Make it peer-to-peer (P2P) for decentralized nodes.
🚀 REST API: Build an HTTP API to interact with the blockchain.
🚀 Database Integration: Persist blockchain to a database (PostgreSQL).

🎯 Contributing
💡 Contributions are welcome! If you'd like to add features or improve the project, feel 
free to fork the repo and submit a pull request.

Fork the project
Create a new branch
Commit your changes
Push to your fork
Submit a pull request
📜 License
This project is licensed under the MIT License.

🔗 Connect with Me
💬 Have questions or suggestions? Feel free to reach out!

GitHub: @afmontoya
Twitter: @montoyamedia
