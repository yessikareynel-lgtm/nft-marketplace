#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map};

#[contract]
pub struct NftMarketplace;

#[contractimpl]
impl NftMarketplace {
    /// Initialize the contract
    pub fn init(env: Env) {
        if env.storage().instance().get::<_, bool>(&"initialized").is_some() {
            panic!("Already initialized");
        }
        env.storage().instance().set(&"initialized", &true);
    }

    /// List an NFT for sale. Owner lists their NFT with a price.
    /// Stores listing in a Map: token_id -> (owner, price)
    pub fn list_nft(env: Env, owner: Address, token_id: u64, price: u64) {
        owner.require_auth();

        let mut listings: Map<u64, (Address, u64)> = env
            .storage()
            .instance()
            .get(&"listings")
            .unwrap_or(Map::new(&env));

        listings.set(token_id, (owner, price));
        env.storage().instance().set(&"listings", &listings);
    }

    /// Buy an NFT. Buyer pays the price, NFT transfers to buyer, listing removed.
    pub fn buy_nft(env: Env, buyer: Address, token_id: u64) {
        buyer.require_auth();

        let mut listings: Map<u64, (Address, u64)> = env
            .storage()
            .instance()
            .get(&"listings")
            .unwrap_or(Map::new(&env));

        let listing = listings.get(token_id).unwrap_or_else(|| {
            panic!("NFT not listed for sale");
        });

        let (owner, price) = listing;

        if buyer == owner {
            panic!("Cannot buy your own NFT");
        }

        // Note: Actual token transfer and payment processing would require
        // integration with Stellar's token contract. This is a placeholder
        // for the marketplace logic.

        listings.remove(token_id);
        env.storage().instance().set(&"listings", &listings);
    }

    /// Get the listing price for a token_id, or None if not listed.
    pub fn get_listing(env: Env, token_id: u64) -> Option<(Address, u64)> {
        let listings: Map<u64, (Address, u64)> = env
            .storage()
            .instance()
            .get(&"listings")
            .unwrap_or(Map::new(&env));

        listings.get(token_id)
    }

    /// Cancel a listing. Only the owner can delist their NFT.
    pub fn cancel_listing(env: Env, owner: Address, token_id: u64) {
        owner.require_auth();

        let mut listings: Map<u64, (Address, u64)> = env
            .storage()
            .instance()
            .get(&"listings")
            .unwrap_or(Map::new(&env));

        let listing = listings.get(token_id).unwrap_or_else(|| {
            panic!("NFT not listed for sale");
        });

        let (listing_owner, _) = listing;

        if owner != listing_owner {
            panic!("Not the owner of this listing");
        }

        listings.remove(token_id);
        env.storage().instance().set(&"listings", &listings);
    }
}
