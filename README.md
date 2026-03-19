# 🛡️ Stellar Soroban Escrow Smart Contract

## 📌 Project Description

This project is a basic escrow smart contract built using Soroban on the Stellar blockchain. It provides a secure and trustless way for two parties (a buyer and a seller) to transact without needing to trust each other directly.

The contract temporarily holds funds and only releases them when predefined conditions are met.

---

## ⚙️ What it Does

The escrow contract works as a neutral intermediary:

1. The buyer initializes the contract and locks funds.
2. The contract securely stores transaction details.
3. The seller delivers the product or service.
4. The buyer approves the transaction to release funds.
5. Alternatively, the seller can trigger a refund if needed.

---

## ✨ Features

* 🔐 Secure escrow mechanism using smart contracts
* 👤 Role-based authorization (buyer & seller)
* 💸 Controlled fund release
* 🔄 Refund option for failed transactions
* 📦 Lightweight and easy-to-understand design
* 🧩 Easily extendable for advanced use cases

---

## 🚀 Deployed Smart Contract Link

https://stellar.expert/explorer/testnet/contract/CAZWMYYUEAQR3AKQIMDHZDJFI25SFSCSTC4W77SBPBC3U2FQK5LD6GEC
<img width="1366" height="768" alt="Screenshot (9)" src="https://github.com/user-attachments/assets/dfaedf7b-9cc4-43e3-9b96-19b156485fdf" />


---

## 🧠 Future Improvements

* Integration with Soroban token interface for real asset transfers
* Multi-escrow support (handle multiple transactions at once)
* Time-lock mechanism for automatic refunds
* Dispute resolution with third-party arbitrator
* Event logging for better transparency

---

## 🛠️ Tech Stack

* Rust
* Soroban SDK
* Stellar Network

---

## 📄 License

This project is open-source and available under the MIT License.
