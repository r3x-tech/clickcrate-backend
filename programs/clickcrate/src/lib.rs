use anchor_lang::prelude::*;

declare_id!("zLTwC7jeA9miBYVxzbTmDzEMXTobXQ3nxD8BY6M29nF");

pub mod accounts {
    use super::*;

    #[derive(Accounts)]
    pub struct CreateClickCrate<'info> {
        #[account(init, payer = payer, space = 8 + 32 + 32 + 32 + 256 + 8)]
        pub clickcrate: Account<'info, ClickCrate>,
        #[account(mut)]
        pub payer: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct RegisterClickCrate<'info> {
        #[account(mut)]
        pub clickcrate: Account<'info, ClickCrate>,
    }

    #[derive(Accounts)]
    pub struct UnregisterClickCrate<'info> {
        #[account(mut)]
        pub clickcrate: Account<'info, ClickCrate>,
    }

    #[derive(Accounts)]
    pub struct CreateProduct<'info> {
        #[account(init, payer = payer, space = 8 + 32 + 32 + 32 + 256 + 32 + 32)]
        pub product: Account<'info, Product>,
        #[account(mut)]
        pub payer: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct RegisterProduct<'info> {
        #[account(mut)]
        pub product: Account<'info, Product>,
        #[account(mut)]
        pub clickcrate: Account<'info, ClickCrate>,
    }

    #[derive(Accounts)]
    pub struct UnregisterProduct<'info> {
        #[account(mut)]
        pub product: Account<'info, Product>,
    }

    #[derive(Accounts)]
    pub struct MakePurchase<'info> {
        #[account(init, payer = buyer, space = 8 + 32 + 32 + 8)]
        pub purchase: Account<'info, Purchase>,
        #[account(mut)]
        pub product: Account<'info, Product>,
        #[account(mut)]
        pub buyer: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[account]
    pub struct ClickCrate {
        pub name: [u8; 32],
        pub placement_type: [u8; 32],
        pub asset_type: [u8; 32],
        pub user_graph_uri: [u8; 256],
        pub sale_fee: u64,
        pub is_registered: bool,
    }

    #[account]
    pub struct Product {
        pub name: [u8; 32],
        pub placement_type: [u8; 32],
        pub asset_type: [u8; 32],
        pub user_profile_uri: [u8; 256],
        pub listing_origin_id: [u8; 32],
        pub listing_access_token: [u8; 32],
        pub is_listed: bool,
    }

    #[account]
    pub struct Purchase {
        pub buyer_address: [u8; 32],
        pub product_address: [u8; 32],
        pub amount_paid: u64,
        pub timestamp: i64,
    }
}

pub mod instructions {
    use super::*;

    pub fn create_clickcrate(
        ctx: Context<CreateClickCrate>,
        name: String,
        placement_type: String,
        asset_type: String,
        user_graph_uri: String,
        sale_fee: u64,
    ) -> Result<()> {
        let clickcrate = &mut ctx.accounts.clickcrate;
        clickcrate.name = name.as_bytes().try_into().unwrap();
        clickcrate.placement_type = placement_type.as_bytes().try_into().unwrap();
        clickcrate.asset_type = asset_type.as_bytes().try_into().unwrap();
        clickcrate.user_graph_uri = user_graph_uri.as_bytes().try_into().unwrap();
        clickcrate.sale_fee = sale_fee;
        clickcrate.is_registered = false;
        Ok(())
    }

    pub fn register_clickcrate(ctx: Context<RegisterClickCrate>) -> Result<()> {
        let clickcrate = &mut ctx.accounts.clickcrate;
        clickcrate.is_registered = true;
        Ok(())
    }

    pub fn unregister_clickcrate(ctx: Context<UnregisterClickCrate>) -> Result<()> {
        let clickcrate = &mut ctx.accounts.clickcrate;
        clickcrate.is_registered = false;
        Ok(())
    }

    pub fn create_product(
        ctx: Context<CreateProduct>,
        name: String,
        placement_type: String,
        asset_type: String,
        user_profile_uri: String,
        listing_origin_id: String,
        listing_access_token: String,
    ) -> Result<()> {
        let product = &mut ctx.accounts.product;
        product.name = name.as_bytes().try_into().unwrap();
        product.placement_type = placement_type.as_bytes().try_into().unwrap();
        product.asset_type = asset_type.as_bytes().try_into().unwrap();
        product.user_profile_uri = user_profile_uri.as_bytes().try_into().unwrap();
        product.listing_origin_id = listing_origin_id.as_bytes().try_into().unwrap();
        product.listing_access_token = listing_access_token.as_bytes().try_into().unwrap();
        product.is_listed = false;
        Ok(())
    }

    pub fn register_product(ctx: Context<RegisterProduct>) -> Result<()> {
        let product = &mut ctx.accounts.product;
        product.is_listed = true;
        Ok(())
    }

    pub fn unregister_product(ctx: Context<UnregisterProduct>) -> Result<()> {
        let product = &mut ctx.accounts.product;
        product.is_listed = false;
        Ok(())
    }

    pub fn make_purchase(ctx: Context<MakePurchase>) -> Result<()> {
        let purchase = &mut ctx.accounts.purchase;
        purchase.buyer_address = ctx.accounts.buyer.key().to_bytes();
        purchase.product_address = ctx.accounts.product.key().to_bytes();
        purchase.amount_paid = ctx.accounts.product.sale_fee;
        purchase.timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[program]
pub mod clickcrate {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create_clickcrate(
        ctx: Context<CreateClickCrate>,
        name: String,
        placement_type: String,
        asset_type: String,
        user_graph_uri: String,
        sale_fee: u64,
    ) -> Result<()> {
        instructions::create_clickcrate(
            ctx,
            name,
            placement_type,
            asset_type,
            user_graph_uri,
            sale_fee,
        )
    }

    pub fn register_clickcrate(ctx: Context<RegisterClickCrate>) -> Result<()> {
        instructions::register_clickcrate(ctx)
    }

    pub fn unregister_clickcrate(ctx: Context<UnregisterClickCrate>) -> Result<()> {
        instructions::unregister_clickcrate(ctx)
    }

    pub fn create_product(
        ctx: Context<CreateProduct>,
        name: String,
        placement_type: String,
        asset_type: String,
        user_profile_uri: String,
        listing_origin_id: String,
        listing_access_token: String,
    ) -> Result<()> {
        instructions::create_product(
            ctx,
            name,
            placement_type,
            asset_type,
            user_profile_uri,
            listing_origin_id,
            listing_access_token,
        )
    }

    pub fn register_product(ctx: Context<RegisterProduct>) -> Result<()> {
        instructions::register_product(ctx)
    }

    pub fn unregister_product(ctx: Context<UnregisterProduct>) -> Result<()> {
        instructions::unregister_product(ctx)
    }

    pub fn make_purchase(ctx: Context<MakePurchase>) -> Result<()> {}
}

#[derive(Accounts)]
pub struct Initialize {}
