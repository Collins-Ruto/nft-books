use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, json_types::U128, log, near_bindgen, AccountId, Promise};

#[near_bindgen]
#[derive( BorshDeserialize, BorshSerialize)]
pub struct Nft {
    author: String,
    title: String,
    owner: String,
}

impl Default for Nft {
    fn default() -> Self {
        Self {
            author: "collins ruto".to_string(),
            title: "The untrekked".to_string(),
            owner: "Elon musk".to_string()
        }
    }
}

#[near_bindgen]
impl Nft {
    pub fn get_nft(self) -> String {
        self.title
    }
}