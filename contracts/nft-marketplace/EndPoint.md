### Execute Endpoints

1. **List an NFT for Sale**
   - **Endpoint**: `ExecuteMsg::ListNft`
   - **Parameters**:
     - `contract_address`: Address of the NFT contract
     - `token_id`: ID of the NFT to be listed
     - `auction_config`: Configuration for the auction (if applicable)

2. **List a Fractional NFT**
   - **Endpoint**: `ExecuteMsg::ListFractionalNft`
   - **Parameters**:
     - `contract_address`: Address of the NFT contract
     - `token_id`: ID of the NFT to be listed
     - `shares`: Number of shares for fractional ownership
     - `price_per_share`: Price per share

3. **Buy a Fractional NFT**
   - **Endpoint**: `ExecuteMsg::BuyFractionalNft`
   - **Parameters**:
     - `contract_address`: Address of the NFT contract
     - `token_id`: ID of the NFT
     - `shares`: Number of shares to buy

4. **Buy an NFT**
   - **Endpoint**: `ExecuteMsg::Buy`
   - **Parameters**:
     - `contract_address`: Address of the NFT contract
     - `token_id`: ID of the NFT to be bought

5. **Cancel a Listing**
   - **Endpoint**: `ExecuteMsg::Cancel`
   - **Parameters**:
     - `contract_address`: Address of the NFT contract
     - `token_id`: ID of the NFT whose listing is to be canceled

6. **Offer an NFT**
   - **Endpoint**: `ExecuteMsg::OfferNft`
   - **Parameters**:
     - `nft`: Details of the NFT
     - `funds_amount`: Amount of funds offered
     - `end_time`: End time of the offer

7. **Accept an NFT Offer**
   - **Endpoint**: `ExecuteMsg::AcceptNftOffer`
   - **Parameters**:
     - `offerer`: Address of the offerer
     - `nft`: Details of the NFT
     - `funds_amount`: Amount of funds offered

8. **Cancel an Offer**
   - **Endpoint**: `ExecuteMsg::CancelOffer`
   - **Parameters**:
     - `nfts`: List of NFTs whose offers need to be canceled

9. **Edit VAURA Token Address**
   - **Endpoint**: `ExecuteMsg::EditVauraToken`
   - **Parameters**:
     - `token_address`: New address of the VAURA token

----
### Query Endpoints

1. **Get Config**
   - **Endpoint**: `QueryMsg::Config`
   - **Function Call**:

2. **Get Listings by Contract Address**
   - **Endpoint**: `QueryMsg::ListingsByContractAddress`
   - **Parameters**:
     - `contract_address`: Address of the NFT contract
     - `start_after`: (Optional) Pagination parameter
     - `limit`: (Optional) Maximum number of listings to fetch

3. **Get a Listing**
   - **Endpoint**: `QueryMsg::Listing`
   - **Parameters**:
     - `contract_address`: Address of the NFT contract
     - `token_id`: ID of the NFT to query

4. **Get an Offer**
   - **Endpoint**: `QueryMsg::Offer`
   - **Parameters**:
     - `contract_address`: Address of the NFT contract
     - `token_id`: ID of the NFT
     - `offerer`: Address of the offerer

5. **Get NFT Offers**
   - **Endpoint**: `QueryMsg::NftOffers`
   - **Parameters**:
     - `contract_address`: Address of the NFT contract
     - `token_id`: ID of the NFT
     - `start_after_offerer`: (Optional) Pagination parameter
     - `limit`: (Optional) Maximum number of offers to fetch

6. **Get User Offers**
   - **Endpoint**: `QueryMsg::UserOffers`
   - **Parameters**:
     - `offerer`: Address of the offerer
     - `start_after_nft`: (Optional) Pagination parameter
     - `limit`: (Optional) Maximum number of offers to fetch
