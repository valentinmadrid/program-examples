use anchor_lang::prelude::*;


#[account]
pub struct AddressInfo {
    pub name: String,
    pub house_number: u8,
    pub street: String,
    pub city: String,
}

impl AddressInfo {

    pub const ACCOUNT_SPACE: usize = 8 + 40 + 1 + 40 + 40;

    pub fn new(
        name: String,
        house_number: u8,
        street: String,
        city: String,
    ) -> Self {
        
        AddressInfo {
            name,
            house_number,
            street,
            city,
        }
    }
}