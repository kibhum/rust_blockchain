use crate::blockchain::block_and_blockchain::Serialization;
use std::fmt::Display;

#[derive(Debug)]
pub struct Transaction {
    sender_address: Vec<u8>,
    recipient_address: Vec<u8>,
    value: u64,
}

impl Transaction {
    pub fn new(sender: Vec<u8>, recipient: Vec<u8>, value: u64) -> Transaction {
        Transaction {
            sender_address: sender,
            recipient_address: recipient,
            value,
        }
    }
}

impl Serialization<Transaction> for Transaction {
    fn serialization(&self) -> Vec<u8> {
        let mut bin = Vec::<u8>::new();
        // 1. Find the length of the sender's address
        let sender_length = self.sender_address.len();
        // 2. create a vector for the bytes version of sender_length and concatenate to the bin vector
        bin.extend(sender_length.to_be_bytes().to_vec());
        // 3. Concatenate the senders address as it's already in bytes
        bin.extend(&self.sender_address);
        // 4. Find the length of the recipient's address
        let recipient_length = self.recipient_address.len();
        // 5. create a vector for the bytes version of recipient_length and concatenate to the bin vector
        bin.extend(recipient_length.to_be_bytes().to_vec());
        // 6. Concatenate the recipient address as it's already in bytes
        bin.extend(&self.recipient_address);
        // 7. Find the length of the value after converting it to bytes
        let value_length = self.value.to_be_bytes().len();
        // 8. create a vector for the bytes version of value_length and concatenate to the bin vector
        bin.extend(value_length.to_be_bytes().to_vec());
        // 9. Convert the value to bytes and concatenate it to the bin vector
        bin.extend(self.value.to_be_bytes().to_vec());
        bin
    }

    fn deserialization(bytes: Vec<u8>) -> Transaction {
        let mut pos = 0;
        // 1. Getting the sender's length. In serialization,it was the first element hence it occupied the first 8 bytes
        // Try into converts the slice into array
        let sender_length = usize::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        // 2. Getting the sender's address
        let mut sender_address = Vec::<u8>::new();
        sender_address.extend_from_slice(&bytes[pos..pos + sender_length]);
        pos += sender_length;
        // 3. Getting the recipient's length
        let recipient_length = usize::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        // 4. Getting the recipient's address
        let mut recipient_address = Vec::<u8>::new();
        recipient_address.extend_from_slice(&bytes[pos..pos + recipient_length]);
        pos += recipient_length;
        // 5. Getting the value's length
        let value_length = usize::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let value: u64 = u64::from_be_bytes(bytes[pos..pos + value_length].try_into().unwrap());

        Transaction {
            sender_address,
            recipient_address,
            value,
        }
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}\nSender Address: {:?}\nRecipient Address: {:?}\nValue: {:?}\n {}",
            "-".repeat(40),
            self.sender_address,
            self.recipient_address,
            self.value,
            "-".repeat(40)
        )
    }
}
