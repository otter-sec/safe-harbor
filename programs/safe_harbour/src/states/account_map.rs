use anchor_lang::prelude::*;

/// Account map entry for efficient key-value storage
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct AccountMapEntry {
    pub key: Pubkey,
    pub value: Pubkey,
}

/// Account map for efficient key-value storage
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct AccountMap {
    pub items: Vec<AccountMapEntry>,
}

impl AccountMap {
    /// Insert or update a key-value pair in the map
    pub fn insert(&mut self, key: Pubkey, value: Pubkey) {
        if let Some(i) = self.items.iter().position(|e| e.key == key) {
            self.items[i].value = value;
        } else {
            self.items.push(AccountMapEntry { key, value });
        }
    }

    /// Get the value associated with a key
    pub fn get(&self, key: Pubkey) -> Option<Pubkey> {
        self.items.iter().find(|e| e.key == key).map(|e| e.value)
    }

    /// Remove a key-value pair from the map
    pub fn remove(&mut self, key: Pubkey) -> bool {
        if let Some(i) = self.items.iter().position(|e| e.key == key) {
            self.items.remove(i);
            true
        } else {
            false
        }
    }

    /// Get the number of entries in the map
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the map is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Check if the map contains a key
    pub fn contains_key(&self, key: Pubkey) -> bool {
        self.items.iter().any(|e| e.key == key)
    }
}
