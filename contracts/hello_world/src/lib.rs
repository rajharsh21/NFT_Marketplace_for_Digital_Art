#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, String, Address, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct NFT {
    pub token_id: u64,   // Unique ID for each NFT
    pub artist: Address, // Address of the artist who created the NFT
    pub title: String,   // Title of the artwork
    pub description: String, // Description of the artwork
    pub price: u64,      // Price of the NFT in Lumens (XLM)
    pub owner: Address,  // Address of the current owner of the NFT
    pub royalty: u64,    // Royalty percentage for the artist on resale
}

#[contract]
pub struct NFTMarketplaceContract;

#[contractimpl]
impl NFTMarketplaceContract {

    // Function to mint a new NFT
    pub fn mint_nft(env: Env, artist: Address, title: String, description: String, price: u64, royalty: u64) -> u64 {
        let mut token_id: u64 = env.storage().instance().get(&symbol_short!("TOKEN_ID")).unwrap_or(0);
        token_id += 1;
        
        let nft = NFT {
            token_id: token_id,
            artist: artist.clone(),
            title: title,
            description: description,
            price: price,
            owner: artist.clone(), // Initially, the artist is the owner
            royalty: royalty,
        };

        env.storage().instance().set(&symbol_short!("NFT"), &nft);
        env.storage().instance().set(&symbol_short!("TOKEN_ID"), &token_id);

        log!(&env, "NFT Minted with Token ID: {}", token_id);
        token_id
    }

    // Function to list an NFT for sale
    pub fn list_nft_for_sale(env: Env, token_id: u64, price: u64) {
        let mut nft = Self::get_nft(env.clone(), token_id);
        nft.price = price;
        env.storage().instance().set(&symbol_short!("NFT"), &nft);

        log!(&env, "NFT Listed for Sale with Token ID: {}", token_id);
    }

    // Function to purchase an NFT
    pub fn purchase_nft(env: Env, token_id: u64, buyer: Address) {
        let mut nft = Self::get_nft(env.clone(), token_id);
        let price = nft.price;
        let royalty = (price * nft.royalty) / 100;

        // Perform token transfers (not shown: implement logic for transferring XLM and royalties)
        // Assume env.transfer() transfers XLM (pseudo-code):
        // env.transfer(nft.owner, buyer, price - royalty);
        // env.transfer(buyer, nft.artist, royalty);

        nft.owner = buyer.clone();
        env.storage().instance().set(&symbol_short!("NFT"), &nft);

        log!(&env, "NFT Purchased with Token ID: {}", token_id);
    }

    // Function to get NFT details
    pub fn get_nft(env: Env, token_id: u64) -> NFT {
        env.storage().instance().get(&symbol_short!("NFT")).unwrap_or_else(|| {
            panic!("NFT with Token ID: {} does not exist", token_id);
        })
    }
}
