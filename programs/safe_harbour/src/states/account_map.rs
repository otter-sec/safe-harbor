use anchor_lang::prelude::*;
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct AccountMapEntry {
    pub key: Pubkey,
    pub value: Pubkey,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct AccountMap {
    pub items: Vec<AccountMapEntry>,
}

impl AccountMap {
    pub fn insert(&mut self, key: Pubkey, value: Pubkey) {
        if let Some(i) = self.items.iter().position(|e| e.key == key) {
            self.items[i].value = value;
        } else {
            self.items.push(AccountMapEntry { key, value });
        }
    }

    pub fn get(&self, key: Pubkey) -> Option<Pubkey> {
        self.items.iter().find(|e| e.key == key).map(|e| e.value)
    }

    pub fn remove(&mut self, key: Pubkey) -> bool {
        if let Some(i) = self.items.iter().position(|e| e.key == key) {
            self.items.remove(i);
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn contains_key(&self, key: Pubkey) -> bool {
        self.items.iter().any(|e| e.key == key)
    }
}
