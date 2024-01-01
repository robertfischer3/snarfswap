use crypto::{sha256, generate_key_pair};
use transaction::Transaction;
mod payment_channel {
    struct PaymentChannel {
        sender: KeyPair,
        receiver: KeyPair,
        expiry: u64,
        balance: u64,
        open_transaction: Transaction,
    }

    impl PaymentChannel {
        // Generates initial on-chain funding transaction
        fn escrow_addr(&self) -> String {
            let multisig = MultiSig::from_pubkeys(
                vec![self.sender.pubkey(), self.receiver.pubkey()]
            );
            multisig.address()
        }
        fn open(&self) -> Transaction {

            let funding_amount = 1_000;
            let input = TransactionInput {
                txid: "...", // Funding txid
                index: 0, // Funding tx output index
                unlock_sig: self.sender.sign(payload),
            };

            let output = TransactionOutput {
                value: funding_amount,
                addr: self.escrow_addr(), // Multisig escrow address
            };

            let tx = Transaction {
                // Inputs, outputs, signatures, etc
                inputs: vec![input],
                outputs: vec![output],
            };
            tx
        }

        // Creates off-chain payment transaction
        fn create_payment(&self, amount: u64) -> Transaction {
            let tx = Transaction {
                // Lock with hash of secret
            };
            tx
        }

        // Redeems payment by revealing secret
        fn redeem_payment(&self, secret: String) -> Transaction {
            let tx = Transaction {
                // Validate hash and unlock
            };
            tx
        }

        // Refunds balance to participants
        fn close(&self) -> Transaction {
            let tx = Transaction {
                // Split based on balances
            };
            tx
        }
    }
}