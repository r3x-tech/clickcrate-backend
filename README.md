# ClickCrate POC

Sell Real World Assets (RWAs) in games at the click of a button



## Get Started

**ClickCrate API _(@/clickcrate-api)_** - Node.js API deployed to gcloud that interacts with on-chain Solana ClickCrate program

**ClickCrate Program _(@/programs/lib.rs)_** - Core on-chain Solana ClickCrate program

**ClickCrate POC Integration in minigame** - ClickCrate API integrated into [Slip & Slither Game](https://slipandslither.r3x.tech/) together with OraHacks NFTs ([Slip & Slither Repo](https://github.com/r3x-tech/slip-and-slither))

[ClickCrate Shopify Store](https://c77256-c7.myshopify.com/) - Shopify Store from where items are sourced for POC




## Introduction to ClickCrate

Digital Sales is already hard but over coming years it will get even harder.

People are increasingly experiencing ad fatigue resulting in Click-through-Rates failing advertisers and ads efficiency falling ~15-20% YoY. The ever increasing Cost per Click as well as Cost per Mile as online sales increase are making the final product sale more and more expensive. Retail Media Networks rule e-commerce and ads with 80% of market attributed to 4 companies resulting in a direct sales monopoly. But perhaps worst of all in an unprecedented mark-up in the history of sales, app stores take a 30% cut from all sales in the form of in-app purchases. This is unsustainable and a shift has already begun. Increasing as ad spend and market competition is driving sales to move beyond the online store shopping experiences that we all know to a growing 310 million social media users. Novel methods like influencer
driven sales, direct in-post purchasing, and flash sales during streams are seeing traction. Through it all it has become apparent that direct sales remain the most cost effective option. However, as social media is gobbled up by the big players gaming remains the next biggest and a somewhat untapped market for direct sales.

That is why we built CLICKCRATE. An easy to use API and soon to come SDK that allows game developers to sell real world assets in their game in as little as two clicks. To start when simply request an API Key, create/register your in-game Point of Purchase (POP) through the create-clickcrate/register-clickcrate endpoints, and when you’re ready to start earning integrate the make-purchase api call into your the game logic associated with your Point of Purchase (POP) to allow users to purchase embedded real world assets.

While your CLICKCRATE is being created sellers are similarly creating/registering their products and once they are in our system our dynamic algorithm secured by blockchain and soon to be powered by AI will automatically find the perfect product to place in your Point of Purchase (POP). As the inventory for a product placed into your Point of Purchase (POP) sells you will receive earnings according to the fee that you specified during your CLICKCRATE’s creation.

But the best part? Your sales are direct and your earnings from product placements are 100% yours, not beholden to the 30% fee of app stores.

What are you waiting for? It’s time for players to unlock a truly seamless ecommerce experience that connects the digital and physical worlds so checkout the ClickCrate POC here: https://slipandslither.r3x.tech/

Or just watch the demo video found in our pitch deck here:
https://www.canva.com/design/DAGB5dWBiq8/72e2D0tnSvi-ZG4G357blA/view?utm_content=DAGB5dWBiq8&utm_campaign=designshare&utm_medium=link&utm_source=editor

## Architecture

![Image Alt Text](assets/architecture.jpeg)

## User Flows


*Player:*
1. See a product you want to buy while playing a game 
2. Review/view product in game embed
3. Fill out info (ie. credit card if needed and shipping info) & sign tx to confirm purchase
4. Receive order updates and info to the contact method you provided via the embed to the integrate service (ie. Shopify). Order management actions like returns can be managed through Shopify and your provided contact method.


*Game Dev:*
1. Create ClickCrate (Product Purchase Point aka POP) as a collection NFT on-chain specifying parameters in metadata. For example:
    1. Name (Name of the product purchase point)
    2. Description (Short description of the product purchase point)
    3. Image (Image showing the product purchase point)
    4. Placement Type (Some placement types might be in game placement ie. digital product replica can be bought as an rwa in the game, related purchase ie. a RWA product which is somehow related to an in game digital asset that can be purchased and placed in the clickcrate for that associated game digital asset, targeted advertisement/product placement ie. a POP that can accept multiple RWAs which are matched/placed by an AI based algorithm/system based on a user’s graph and additional info a seller provides) 
    5. Asset Type (Categories/Details of acceptable products like any, t-shirt, shoe, beverage, etc.)
    6. Additional Placement Requirements (optional additional requirements for products specified by the game developer)
    7. User Graph Uri (ie. can be empty but user graph could be uploaded as JSON to say SHDW drive and provided in this field as a URI)
    8. Sale Fee (to start would be a flat fee the creator specifies like $0.01 of every purchase of that product that is made)
2. Register ClickCrate via API or on-chain program to make it live and available for product placements
3. Receive payouts for products sold in your ClickCrate to the wallet with which you created the ClickCrate 


 *Advertiser/Product Creator/Seller:*
1. Create a product as an NFT on-chain specifying parameters in metadata. For example: 
    1. Name (Name of the product you would like to sell)
    2. Description (Short description of the product that will be sold)
    3. Image (Image showing the product purchase point)
    4. Placement Type (Some placement types might be in game placement ie. digital product replica can be bought as an rwa in the game, related purchase ie. a RWA product which is somehow related to an in game digital asset that can be purchased and placed in the clickcrate for that associated game digital asset, targeted advertisement/product placement ie. a POP that can accept multiple RWAs which are matched/placed by an AI based algorithm/system based on a user’s graph and additional info a seller provides) 
    5. Asset Type (Categories/Details of acceptable products like any, t-shirt, shoe, beverage, etc.)
    6. Additional Placement Requirements (optionally additional requirements specified by the creator)
    7. User Profile Uri (for the product creator these are the params that should be found in the user graph json from the user graph uri in the click crate. ie. can be empty but ideally these params would be uploaded as JSON to say shdw drive and provided in this field as a uri.)
    8. Listing Origin Id (the identifier of the store/marketplace/url where the product is listed for sale. To start only Shopify Store ids are supported)
2. List/register the product via API or on-chain program to make it live and available for product placements. Make sure to provide a valid Third Party Access Token which is used to access the products of the store/marketplace/url where the product is listed for sale. To start only Shopify store access tokens are supported and are encrypted.
3. Product gets placed into a matching product placement location. Product is sold until inventory runs out at which point it is unlisted from the product purchase point and can no longer be purchased.
4. Payouts and all regular order management like fulfillment are handled via existing product listing software integration (ie. Shopify)
