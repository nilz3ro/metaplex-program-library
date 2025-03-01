use crate::{
    deprecated_instruction::{MintPrintingTokensViaTokenArgs, SetReservationListArgs},
    state::{
        Collection, CollectionDetails, Creator, Data, DataV2, Uses, EDITION,
        EDITION_MARKER_BIT_SIZE, PREFIX,
    },
};
use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};
#[cfg(feature = "serde-feature")]
use {
    serde::{Deserialize, Serialize},
    serde_with::{As, DisplayFromStr},
};

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
/// Args for update call
pub struct UpdateMetadataAccountArgs {
    pub data: Option<Data>,
    #[cfg_attr(
        feature = "serde-feature",
        serde(with = "As::<Option<DisplayFromStr>>")
    )]
    pub update_authority: Option<Pubkey>,
    pub primary_sale_happened: Option<bool>,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
/// Args for update call
pub struct UpdateMetadataAccountArgsV2 {
    pub data: Option<DataV2>,
    #[cfg_attr(
        feature = "serde-feature",
        serde(with = "As::<Option<DisplayFromStr>>")
    )]
    pub update_authority: Option<Pubkey>,
    pub primary_sale_happened: Option<bool>,
    pub is_mutable: Option<bool>,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
/// Args for create call
pub struct CreateMetadataAccountArgs {
    /// Note that unique metadatas are disabled for now.
    pub data: Data,
    /// Whether you want your metadata to be updateable in the future.
    pub is_mutable: bool,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
/// Args for create call
pub struct CreateMetadataAccountArgsV2 {
    /// Note that unique metadatas are disabled for now.
    pub data: DataV2,
    /// Whether you want your metadata to be updateable in the future.
    pub is_mutable: bool,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
/// Args for create call
pub struct CreateMetadataAccountArgsV3 {
    /// Note that unique metadatas are disabled for now.
    pub data: DataV2,
    /// Whether you want your metadata to be updateable in the future.
    pub is_mutable: bool,
    /// If this is a collection parent NFT.
    pub collection_details: Option<CollectionDetails>,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct CreateMasterEditionArgs {
    /// If set, means that no more than this number of editions can ever be minted. This is immutable.
    pub max_supply: Option<u64>,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct MintNewEditionFromMasterEditionViaTokenArgs {
    pub edition: u64,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct ApproveUseAuthorityArgs {
    pub number_of_uses: u64,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct UtilizeArgs {
    pub number_of_uses: u64,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct SetCollectionSizeArgs {
    pub size: u64,
}

/// Instructions supported by the Metadata program.
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, Clone, ShankInstruction)]
#[rustfmt::skip]

pub enum MetadataInstruction {
    /// Create Metadata object.
    #[account(0, writable, name="metadata", desc="Metadata key (pda of ['metadata', program id, mint id])")]
    #[account(1, name="mint", desc="Mint of token asset")]
    #[account(2, signer, name="mint_authority", desc="Mint authority")]
    #[account(3, signer, writable, name="payer", desc="payer")]
    #[account(4, name="update_authority", desc="update authority info")]
    #[account(5, name="system_program", desc="System program")]
    #[account(6, name="rent", desc="Rent info")]
    CreateMetadataAccount(CreateMetadataAccountArgs),

    /// Update a Metadata
    #[account(0, writable, name="metadata", desc="Metadata account")]
    #[account(1, signer, name="update_authority", desc="Update authority key")]
    UpdateMetadataAccount(UpdateMetadataAccountArgs),

    /// Register a Metadata as a Master Edition V1, which means Editions can be minted.
    /// Henceforth, no further tokens will be mintable from this primary mint. Will throw an error if more than one
    /// token exists, and will throw an error if less than one token exists in this primary mint.
    #[account(0, writable, name="edition", desc="Unallocated edition V1 account with address as pda of ['metadata', program id, mint, 'edition']")]
    #[account(1, writable, name="mint", desc="Metadata mint")]
    #[account(2, writable, name="printing_mint", desc="Printing mint - A mint you control that can mint tokens that can be exchanged for limited editions of your master edition via the MintNewEditionFromMasterEditionViaToken endpoint")]
    #[account(3, writable, name="one_time_printing_authorization_mint", desc="One time authorization printing mint - A mint you control that prints tokens that gives the bearer permission to mint any number of tokens from the printing mint one time via an endpoint with the token-metadata program for your metadata. Also burns the token.")]
    #[account(4, signer, name="update_authority", desc="Current Update authority key")]
    #[account(5, signer, name="printing_mint_authority", desc="Printing mint authority - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY.")]
    #[account(6, signer, name="mint_authority", desc="Mint authority on the metadata's mint - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY")]
    #[account(7, name="metadata", desc="Metadata account")]
    #[account(8, signer, name="payer", desc="payer")]
    #[account(9, name="token_program", desc="Token program")]
    #[account(10, name="system_program", desc="System program")]
    #[account(11, name="rent", desc="Rent info")]
    #[account(12, signer, name="one_time_printing_authorization_mint_authority", desc="One time authorization printing mint authority - must be provided if using max supply. THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY.")]
    DeprecatedCreateMasterEdition(CreateMasterEditionArgs),

    /// Given an authority token minted by the Printing mint of a master edition, and a brand new non-metadata-ed mint with one token
    /// make a new Metadata + Edition that is a child of the master edition denoted by this authority token.
    #[account(0, writable, name="metadata", desc="New Metadata key (pda of ['metadata', program id, mint id])")]
    #[account(1, writable, name="edition", desc="New Edition V1 (pda of ['metadata', program id, mint id, 'edition'])")]
    #[account(2, writable, name="master_edition", desc="Master Record Edition V1 (pda of ['metadata', program id, master metadata mint id, 'edition'])")]
    #[account(3, writable, name="mint", desc="Mint of new token - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY")]
    #[account(4, signer, name="mint_authority", desc="Mint authority of new mint")]
    #[account(5, writable, name="printing_mint", desc="Printing Mint of master record edition")]
    #[account(6, writable, name="master_token_account", desc="Token account containing Printing mint token to be transferred")]
    #[account(7, writable, name="edition_marker", desc="Edition pda to mark creation - will be checked for pre-existence. (pda of ['metadata', program id, master mint id, edition_number])")]
    #[account(8, signer, name="burn_authority", desc="Burn authority for this token")]
    #[account(9, signer, name="payer", desc="payer")]
    #[account(10, name="master_update_authority", desc="update authority info for new metadata account")]
    #[account(11, name="master_metadata", desc="Master record metadata account")]
    #[account(12, name="token_program", desc="Token program")]
    #[account(13, name="system_program", desc="System program")]
    #[account(14, name="rent", desc="Rent info")]
    #[account(15, optional, writable, name="reservation_list", desc="Reservation List - If present, and you are on this list, you can get an edition number given by your position on the list.")]
    DeprecatedMintNewEditionFromMasterEditionViaPrintingToken,

    /// Allows updating the primary sale boolean on Metadata solely through owning an account
    /// containing a token from the metadata's mint and being a signer on this transaction.
    /// A sort of limited authority for limited update capability that is required for things like
    /// Metaplex to work without needing full authority passing.
    #[account(0, writable, name="metadata", desc="Metadata key (pda of ['metadata', program id, mint id])")]
    #[account(1, signer, name="owner", desc="Owner on the token account")]
    #[account(2, name="token", desc="Account containing tokens from the metadata's mint")]
    UpdatePrimarySaleHappenedViaToken,

    /// Reserve up to 200 editions in sequence for up to 200 addresses in an existing reservation PDA, which can then be used later by
    /// redeemers who have printing tokens as a reservation to get a specific edition number
    /// as opposed to whatever one is currently listed on the master edition. Used by Auction Manager
    /// to guarantee printing order on bid redemption. AM will call whenever the first person redeems a
    /// printing bid to reserve the whole block
    /// of winners in order and then each winner when they get their token submits their mint and account
    /// with the pda that was created by that first bidder - the token metadata can then cross reference
    /// these people with the list and see that bidder A gets edition #2, so on and so forth.
    ///
    /// NOTE: If you have more than 20 addresses in a reservation list, this may be called multiple times to build up the list,
    /// otherwise, it simply wont fit in one transaction. Only provide a total_reservation argument on the first call, which will
    /// allocate the edition space, and in follow up calls this will specifically be unnecessary (and indeed will error.)
    #[account(0, writable, name="master_edition", desc="Master Edition V1 key (pda of ['metadata', program id, mint id, 'edition'])")]
    #[account(1, writable, name="reservation_list", desc="PDA for ReservationList of ['metadata', program id, master edition key, 'reservation', resource-key]")]
    #[account(2, signer, name="resource", desc="The resource you tied the reservation list too")]
    DeprecatedSetReservationList(SetReservationListArgs),

    /// Create an empty reservation list for a resource who can come back later as a signer and fill the reservation list
    /// with reservations to ensure that people who come to get editions get the number they expect. See SetReservationList for more.
    #[account(0, writable, name="reservation_list", desc="PDA for ReservationList of ['metadata', program id, master edition key, 'reservation', resource-key]")]
    #[account(1, signer, name="payer", desc="Payer")]
    #[account(2, signer, name="update_authority", desc="Update authority")]
    #[account(3, name="master_edition", desc=" Master Edition V1 key (pda of ['metadata', program id, mint id, 'edition'])")]
    #[account(4, name="resource", desc="A resource you wish to tie the reservation list to. This is so your later visitors who come to redeem can derive your reservation list PDA with something they can easily get at. You choose what this should be.")]
    #[account(5, name="metadata", desc="Metadata key (pda of ['metadata', program id, mint id])")]
    #[account(6, name="system_program", desc="System program")]
    #[account(7, name="rent", desc="Rent info")]
    DeprecatedCreateReservationList,

    /// Sign a piece of metadata that has you as an unverified creator so that it is now verified.
    #[account(0, writable, name="metadata", desc="Metadata (pda of ['metadata', program id, mint id])")]
    #[account(1, signer, name="creator", desc="Creator")]
    SignMetadata,

    /// Using a one time authorization token from a master edition v1, print any number of printing tokens from the printing_mint
    /// one time, burning the one time authorization token.
    #[account(0, writable, name="destination", desc="Destination account")]
    #[account(1, writable, name="token", desc="Token account containing one time authorization token")]
    #[account(2, writable, name="one_time_printing_authorization_mint", desc="One time authorization mint")]
    #[account(3, writable, name="printing_mint", desc="Printing mint")]
    #[account(4, signer, name="burn_authority", desc="Burn authority")]
    #[account(5, name="metadata", desc="Metadata key (pda of ['metadata', program id, mint id])")]
    #[account(6, name="master_edition", desc="Master Edition V1 key (pda of ['metadata', program id, mint id, 'edition'])")]
    #[account(7, name="token_program", desc="Token program")]
    #[account(8, name="rent", desc="Rent")]
    DeprecatedMintPrintingTokensViaToken(MintPrintingTokensViaTokenArgs),

    /// Using your update authority, mint printing tokens for your master edition.
    #[account(0, writable, name="destination", desc="Destination account")]
    #[account(1, writable, name="printing_mint", desc="Printing mint")]
    #[account(2, signer, name="update_authority", desc="Update authority")]
    #[account(3, name="metadata", desc="Metadata key (pda of ['metadata', program id, mint id])")]
    #[account(4, name="master_edition", desc="Master Edition V1 key (pda of ['metadata', program id, mint id, 'edition'])")]
    #[account(5, name="token_program", desc="Token program")]
    #[account(6, name="rent", desc="Rent")]
    DeprecatedMintPrintingTokens(MintPrintingTokensViaTokenArgs),

    /// Register a Metadata as a Master Edition V2, which means Edition V2s can be minted.
    /// Henceforth, no further tokens will be mintable from this primary mint. Will throw an error if more than one
    /// token exists, and will throw an error if less than one token exists in this primary mint.
    #[account(0, writable, name="edition", desc="Unallocated edition V2 account with address as pda of ['metadata', program id, mint, 'edition']")]
    #[account(1, writable, name="mint", desc="Metadata mint")]
    #[account(2, signer, name="update_authority", desc="Update authority")]
    #[account(3, signer, name="mint_authority", desc="Mint authority on the metadata's mint - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY")]
    #[account(4, signer, writable, name="payer", desc="payer")]
    #[account(5, name="metadata", desc="Metadata account")]
    #[account(6, name="token_program", desc="Token program")]
    #[account(7, name="system_program", desc="System program")]
    #[account(8, name="rent", desc="Rent info")]
    CreateMasterEdition(CreateMasterEditionArgs),

    /// Given a token account containing the master edition token to prove authority, and a brand new non-metadata-ed mint with one token
    /// make a new Metadata + Edition that is a child of the master edition denoted by this authority token.
    #[account(0, writable, name="new_metadata", desc="New Metadata key (pda of ['metadata', program id, mint id])")]
    #[account(1, writable, name="new_edition", desc="New Edition (pda of ['metadata', program id, mint id, 'edition'])")]
    #[account(2, writable, name="master_edition", desc="Master Record Edition V2 (pda of ['metadata', program id, master metadata mint id, 'edition'])")]
    #[account(3, writable, name="new_mint", desc="Mint of new token - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY")]
    #[account(4, writable, name="edition_mark_pda", desc="Edition pda to mark creation - will be checked for pre-existence. (pda of ['metadata', program id, master metadata mint id, 'edition', edition_number]) where edition_number is NOT the edition number you pass in args but actually edition_number = floor(edition/EDITION_MARKER_BIT_SIZE).")]
    #[account(5, signer, name="new_mint_authority", desc="Mint authority of new mint")]
    #[account(6, signer, writable, name="payer", desc="payer")]
    #[account(7, signer, name="token_account_owner", desc="owner of token account containing master token (#8)")]
    #[account(8, name="token_account", desc="token account containing token from master metadata mint")]
    #[account(9, name="new_metadata_update_authority", desc="Update authority info for new metadata")]
    #[account(10, name="metadata", desc="Master record metadata account")]
    #[account(11, name="token_program", desc="Token program")]
    #[account(12, name="system_program", desc="System program")]
    #[account(13, optional, name="rent", desc="Rent info")]
    MintNewEditionFromMasterEditionViaToken(MintNewEditionFromMasterEditionViaTokenArgs),

    /// Converts the Master Edition V1 to a Master Edition V2, draining lamports from the two printing mints
    /// to the owner of the token account holding the master edition token. Permissionless.
    /// Can only be called if there are currenly no printing tokens or one time authorization tokens in circulation.
    #[account(0, writable, name="master_edition", desc="Master Record Edition V1 (pda of ['metadata', program id, master metadata mint id, 'edition'])")]
    #[account(1, writable, name="one_time_auth", desc="One time authorization mint")]
    #[account(2, writable, name="printing_mint", desc="Printing mint")]
    ConvertMasterEditionV1ToV2,

    /// Proxy Call to Mint Edition using a Store Token Account as a Vault Authority.
    #[account(0, writable, name="new_metadata", desc="New Metadata key (pda of ['metadata', program id, mint id])")]
    #[account(1, writable, name="new_edition", desc="New Edition (pda of ['metadata', program id, mint id, 'edition'])")]
    #[account(2, writable, name="master_edition", desc="Master Record Edition V2 (pda of ['metadata', program id, master metadata mint id, 'edition']")]
    #[account(3, writable, name="new_mint", desc="Mint of new token - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY")]
    #[account(4, writable, name="edition_mark_pda", desc="Edition pda to mark creation - will be checked for pre-existence. (pda of ['metadata', program id, master metadata mint id, 'edition', edition_number]) where edition_number is NOT the edition number you pass in args but actually edition_number = floor(edition/EDITION_MARKER_BIT_SIZE).")]
    #[account(5, signer, name="new_mint_authority", desc="Mint authority of new mint")]
    #[account(6, signer, writable, name="payer", desc="payer")]
    #[account(7, signer, name="vault_authority", desc="Vault authority")]
    #[account(8, name="safety_deposit_store", desc="Safety deposit token store account")]
    #[account(9, name="safety_deposit_box", desc="Safety deposit box")]
    #[account(10, name="vault", desc="Vault")]
    #[account(11, name="new_metadata_update_authority", desc="Update authority info for new metadata")]
    #[account(12, name="metadata", desc="Master record metadata account")]
    #[account(13, name="token_program", desc="Token program")]
    #[account(14, name="token_vault_program", desc="Token vault program")]
    #[account(15, name="system_program", desc="System program")]
    #[account(16, optional, name="rent", desc="Rent info")]
    MintNewEditionFromMasterEditionViaVaultProxy(MintNewEditionFromMasterEditionViaTokenArgs),

    /// Puff a Metadata - make all of it's variable length fields (name/uri/symbol) a fixed length using a null character
    /// so that it can be found using offset searches by the RPC to make client lookups cheaper.
    #[account(0, writable, name="metadata", desc="Metadata account")]
    PuffMetadata,

    /// Update a Metadata with is_mutable as a parameter
    #[account(0, writable, name="metadata", desc="Metadata account")]
    #[account(1, signer, name="update_authority", desc="Update authority key")]
    UpdateMetadataAccountV2(UpdateMetadataAccountArgsV2),

    /// Create Metadata object.
    #[account(0, writable, name="metadata", desc="Metadata key (pda of ['metadata', program id, mint id])")]
    #[account(1, name="mint", desc="Mint of token asset")]
    #[account(2, signer, name="mint_authority", desc="Mint authority")]
    #[account(3, signer, writable, name="payer", desc="payer")]
    #[account(4, name="update_authority", desc="update authority info")]
    #[account(5, name="system_program", desc="System program")]
    #[account(6, optional, name="rent", desc="Rent info")]
    CreateMetadataAccountV2(CreateMetadataAccountArgsV2),

    /// Register a Metadata as a Master Edition V2, which means Edition V2s can be minted.
    /// Henceforth, no further tokens will be mintable from this primary mint. Will throw an error if more than one
    /// token exists, and will throw an error if less than one token exists in this primary mint.
    #[account(0, writable, name="edition", desc="Unallocated edition V2 account with address as pda of ['metadata', program id, mint, 'edition']")]
    #[account(1, writable, name="mint", desc="Metadata mint")]
    #[account(2, signer, name="update_authority", desc="Update authority")]
    #[account(3, signer, name="mint_authority", desc="Mint authority on the metadata's mint - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY")]
    #[account(4, signer, writable, name="payer", desc="payer")]
    #[account(5, writable, name="metadata", desc="Metadata account")]
    #[account(6, name="token_program", desc="Token program")]
    #[account(7, name="system_program", desc="System program")]
    #[account(8, optional, name="rent", desc="Rent info")]
    CreateMasterEditionV3(CreateMasterEditionArgs),

    /// If a MetadataAccount Has a Collection allow the UpdateAuthority of the Collection to Verify the NFT Belongs in the Collection.
    #[account(0, writable, name="metadata", desc="Metadata account")]
    #[account(1, signer, writable, name="collection_authority", desc="Collection Update authority")]
    #[account(2, signer, writable, name="payer", desc="payer")]
    #[account(3, name="collection_mint", desc="Mint of the Collection")]
    #[account(4, name="collection", desc="Metadata Account of the Collection")]
    #[account(5, name="collection_master_edition_account", desc="MasterEdition2 Account of the Collection Token")]
    VerifyCollection,

    /// Utilize or Use an NFT , burns the NFT and returns the lamports to the update authority if the use method is burn and its out of uses.
    /// Use Authority can be the Holder of the NFT, or a Delegated Use Authority.
    #[account(0, writable, name="metadata", desc="Metadata account")]
    #[account(1, writable, name="token_account", desc="Token Account Of NFT")]
    #[account(2, writable, name="mint", desc="Mint of the Metadata")]
    #[account(3, signer, writable, name="use_authority", desc="A Use Authority / Can be the current Owner of the NFT")]
    #[account(4, name="owner", desc="Owner")]
    #[account(5, name="token_program", desc="Token program")]
    #[account(6, name="ata_program", desc="Associated Token program")]
    #[account(7, name="system_program", desc="System program")]
    // Rent is technically not needed but there isn't a way to "ignore" an account without 
    // preventing latter accounts from being passed in.
    #[account(8, name="rent", desc="Rent info")]
    #[account(9, optional, writable, name="use_authority_record", desc="Use Authority Record PDA If present the program Assumes a delegated use authority")]
    #[account(10, optional, name="burner", desc="Program As Signer (Burner)")]
    Utilize(UtilizeArgs),

    /// Approve another account to call [utilize] on this NFT.
    #[account(0, writable, name="use_authority_record", desc="Use Authority Record PDA")]
    #[account(1, signer, writable, name="owner", desc="Owner")]
    #[account(2, signer, writable, name="payer", desc="Payer")]
    #[account(3, name="user", desc="A Use Authority")]
    #[account(4, writable, name="owner_token_account", desc="Owned Token Account Of Mint")]
    #[account(5, name="metadata", desc="Metadata account")]
    #[account(6, name="mint", desc="Mint of Metadata")]
    #[account(7, name="burner", desc="Program As Signer (Burner)")]
    #[account(8, name="token_program", desc="Token program")]
    #[account(9, name="system_program", desc="System program")]
    #[account(10, optional, name="rent", desc="Rent info")]
    ApproveUseAuthority(ApproveUseAuthorityArgs),

    /// Revoke account to call [utilize] on this NFT.
    #[account(0, writable, name="use_authority_record", desc="Use Authority Record PDA")]
    #[account(1, signer, writable, name="owner", desc="Owner")]
    #[account(2, name="user", desc="A Use Authority")]
    #[account(3, writable, name="owner_token_account", desc="Owned Token Account Of Mint")]
    #[account(4, name="mint", desc="Mint of Metadata")]
    #[account(5, name="metadata", desc="Metadata account")]
    #[account(6, name="token_program", desc="Token program")]
    #[account(7, name="system_program", desc="System program")]
    #[account(8, optional, name="rent", desc="Rent info")]
    RevokeUseAuthority,

    /// If a MetadataAccount Has a Collection allow an Authority of the Collection to unverify an NFT in a Collection.
    #[account(0, writable, name="metadata", desc="Metadata account")]
    #[account(1, signer, writable, name="collection_authority", desc="Collection Authority")]
    #[account(2, name="collection_mint", desc="Mint of the Collection")]
    #[account(3, name="collection", desc="Metadata Account of the Collection")]
    #[account(4, name="collection_master_edition_account", desc="MasterEdition2 Account of the Collection Token")]
    #[account(5, optional, name="collection_authority_record", desc="Collection Authority Record PDA")]
    UnverifyCollection,

    /// Approve another account to verify NFTs belonging to a collection, [verify_collection] on the collection NFT.
    #[account(0, writable, name="collection_authority_record", desc="Collection Authority Record PDA")]
    #[account(1, name="new_collection_authority", desc="A Collection Authority")]
    #[account(2, signer, writable, name="update_authority", desc="Update Authority of Collection NFT")]
    #[account(3, signer, writable, name="payer", desc="Payer")]
    #[account(4, name="metadata", desc="Collection Metadata account")]
    #[account(5, name="mint", desc="Mint of Collection Metadata")]
    #[account(6, name="system_program", desc="System program")]
    #[account(7, optional, name="rent", desc="Rent info")]
    ApproveCollectionAuthority,

    /// Revoke account to call [verify_collection] on this NFT.
    #[account(0, writable, name="collection_authority_record", desc="Collection Authority Record PDA")]
    #[account(1, writable, name="delegate_authority", desc="Delegated Collection Authority")]
    #[account(2, signer, writable, name="revoke_authority", desc="Update Authority, or Delegated Authority, of Collection NFT")]
    #[account(3, name="metadata", desc="Metadata account")]
    #[account(4, name="mint", desc="Mint of Metadata")]
    RevokeCollectionAuthority,

    /// Allows the same Update Authority (Or Delegated Authority) on an NFT and Collection to perform [update_metadata_accounts_v2] 
    /// with collection and [verify_collection] on the NFT/Collection in one instruction.
    #[account(0, writable, name="metadata", desc="Metadata account")]
    #[account(1, signer, writable, name="collection_authority", desc="Collection Update authority")]
    #[account(2, signer, writable, name="payer", desc="Payer")]
    #[account(3, name="update_authority", desc="Update Authority of Collection NFT and NFT")]
    #[account(4, name="collection_mint", desc="Mint of the Collection")]
    #[account(5, name="collection", desc="Metadata Account of the Collection")]
    #[account(6, name="collection_master_edition_account", desc="MasterEdition2 Account of the Collection Token")]
    #[account(7, optional, name="collection_authority_record", desc="Collection Authority Record PDA")]
    SetAndVerifyCollection,

    /// Allow freezing of an NFT if this user is the delegate of the NFT.
    #[account(0, signer, writable, name="delegate", desc="Delegate")]
    #[account(1, writable, name="token_account", desc="Token account to freeze")]
    #[account(2, name="edition", desc="Edition")]
    #[account(3, name="mint", desc="Token mint")]
    #[account(4, name="token_program", desc="Token Program")]
    FreezeDelegatedAccount,

    /// Allow thawing of an NFT if this user is the delegate of the NFT.
    #[account(0, signer, writable, name="delegate", desc="Delegate")]
    #[account(1, writable, name="token_account", desc="Token account to thaw")]
    #[account(2, name="edition", desc="Edition")]
    #[account(3, name="mint", desc="Token mint")]
    #[account(4, name="token_program", desc="Token Program")]
    ThawDelegatedAccount,

    /// Remove Creator Verificaton.
    #[account(0, writable, name="metadata", desc="Metadata (pda of ['metadata', program id, mint id])")]
    #[account(1, signer, name="creator", desc="Creator")]
    RemoveCreatorVerification,

    /// Completely burn a NFT, including closing the metadata account.
    #[account(0, writable, name="metadata", desc="Metadata (pda of ['metadata', program id, mint id])")]
    #[account(1, signer, writable, name="owner", desc="NFT owner")]
    #[account(2, writable, name="mint", desc="Mint of the NFT")]
    #[account(3, writable, name="token_account", desc="Token account to close")]
    #[account(4, writable, name="master_edition_account", desc="MasterEdition2 of the NFT")]
    #[account(5, name="spl token program", desc="SPL Token Program")]
    #[account(6, optional, writable, name="collection_metadata", desc="Metadata of the Collection")]
    BurnNft,

    /// Verify Collection V2, new in v1.3--supports Collection Details.
    /// If a MetadataAccount Has a Collection allow the UpdateAuthority of the Collection to Verify the NFT Belongs in the Collection.
    #[account(0, writable, name="metadata", desc="Metadata account")]
    #[account(1, signer, name="collection_authority", desc="Collection Update authority")]
    #[account(2, signer, writable, name="payer", desc="payer")]
    #[account(3, name="collection_mint", desc="Mint of the Collection")]
    #[account(4, writable, name="collection", desc="Metadata Account of the Collection")]
    #[account(5, name="collection_master_edition_account", desc="MasterEdition2 Account of the Collection Token")]
    #[account(6, optional, name="collection_authority_record", desc="Collection Authority Record PDA")]
    VerifySizedCollectionItem,

    /// Unverify Collection V2, new in v1.3--supports Collection Details.
    /// If a MetadataAccount Has a Collection allow an Authority of the Collection to unverify an NFT in a Collection.
    #[account(0, writable, name="metadata", desc="Metadata account")]
    #[account(1, signer, name="collection_authority", desc="Collection Authority")]
    #[account(2, signer, writable, name="payer", desc="payer")]
    #[account(3, name="collection_mint", desc="Mint of the Collection")]
    #[account(4, writable, name="collection", desc="Metadata Account of the Collection")]
    #[account(5, name="collection_master_edition_account", desc="MasterEdition2 Account of the Collection Token")]
    #[account(6, optional, name="collection_authority_record", desc="Collection Authority Record PDA")]
    UnverifySizedCollectionItem,

    // Set And Verify V2, new in v1.3--supports Collection Details.
    /// Allows the same Update Authority (Or Delegated Authority) on an NFT and Collection to perform [update_metadata_accounts_v2] 
    /// with collection and [verify_collection] on the NFT/Collection in one instruction.
    #[account(0, writable, name="metadata", desc="Metadata account")]
    #[account(1, signer, name="collection_authority", desc="Collection Update authority")]
    #[account(2, signer, writable, name="payer", desc="payer")]
    #[account(3, name="update_authority", desc="Update Authority of Collection NFT and NFT")]
    #[account(4, name="collection_mint", desc="Mint of the Collection")]
    #[account(5, writable, name="collection", desc="Metadata Account of the Collection")]
    #[account(6, writable, name="collection_master_edition_account", desc="MasterEdition2 Account of the Collection Token")]
    #[account(7, optional, name="collection_authority_record", desc="Collection Authority Record PDA")]
    SetAndVerifySizedCollectionItem,

    /// Create Metadata object.
    #[account(0, writable, name="metadata", desc="Metadata key (pda of ['metadata', program id, mint id])")]
    #[account(1, name="mint", desc="Mint of token asset")]
    #[account(2, signer, name="mint_authority", desc="Mint authority")]
    #[account(3, signer, writable, name="payer", desc="payer")]
    #[account(4, name="update_authority", desc="update authority info")]
    #[account(5, name="system_program", desc="System program")]
    #[account(6, optional, name="rent", desc="Rent info")]
    CreateMetadataAccountV3(CreateMetadataAccountArgsV3),

    /// Set size of an existing collection.
    #[account(0, writable, name="collection_metadata", desc="Collection Metadata account")]
    #[account(1, signer, writable, name="collection_authority", desc="Collection Update authority")]
    #[account(2, name="collection_mint", desc="Mint of the Collection")]
    #[account(3, optional, name="collection_authority_record", desc="Collection Authority Record PDA")]
    SetCollectionSize(SetCollectionSizeArgs),

    /// Set the token standard of the asset.
    #[account(0, writable, name="metadata", desc="Metadata account")]
    #[account(1, signer, writable, name="update_authority", desc="Metadata update authority")]
    #[account(2, name="mint", desc="Mint account")]
    #[account(3, optional, name="edition", desc="Edition account")]
    SetTokenStandard,

    /// Set size of an existing collection using CPI from the Bubblegum program.  This is how
    /// collection size is incremented and decremented for compressed NFTs.
    #[account(0, writable, name="collection_metadata", desc="Collection Metadata account")]
    #[account(1, signer, writable, name="collection_authority", desc="Collection Update authority")]
    #[account(2, name="collection_mint", desc="Mint of the Collection")]
    #[account(3, signer, name="bubblegum_signer", desc="Signing PDA of Bubblegum program")]
    #[account(4, optional, name="collection_authority_record", desc="Collection Authority Record PDA")]
    BubblegumSetCollectionSize(SetCollectionSizeArgs),

    /// Completely burn a print edition NFT.
    #[account(0, writable, name="metadata", desc="Metadata (pda of ['metadata', program id, mint id])")]
    #[account(1, signer, writable, name="owner", desc="NFT owner")]
    #[account(2, writable, name="print_edition_mint", desc="Mint of the print edition NFT")]
    #[account(3, writable, name="master_edition_mint", desc="Mint of the original/master NFT")]
    #[account(4, writable, name="print_edition_token_account", desc="Token account the print edition NFT is in")]
    #[account(5, name="master_edition_token_account", desc="Token account the Master Edition NFT is in")]
    #[account(6, writable, name="master_edition_account", desc="MasterEdition2 of the original NFT")]
    #[account(7, writable, name="print_edition_account", desc="Print Edition account of the NFT")]
    #[account(8, writable, name="edition_marker_account", desc="Edition Marker PDA of the NFT")]
    #[account(9, name="spl token program", desc="SPL Token Program")]
    BurnEditionNft,
}

/// Creates an CreateMetadataAccounts instruction
/// #[deprecated(since="1.1.0", note="please use `create_metadata_accounts_v3` instead")]
#[allow(clippy::too_many_arguments)]
pub fn create_metadata_accounts(
    program_id: Pubkey,
    metadata_account: Pubkey,
    mint: Pubkey,
    mint_authority: Pubkey,
    payer: Pubkey,
    update_authority: Pubkey,
    name: String,
    symbol: String,
    uri: String,
    creators: Option<Vec<Creator>>,
    seller_fee_basis_points: u16,
    update_authority_is_signer: bool,
    is_mutable: bool,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(metadata_account, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(mint_authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(update_authority, update_authority_is_signer),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: MetadataInstruction::CreateMetadataAccount(CreateMetadataAccountArgs {
            data: Data {
                name,
                symbol,
                uri,
                seller_fee_basis_points,
                creators,
            },
            is_mutable,
        })
        .try_to_vec()
        .unwrap(),
    }
}

/// Creates an CreateMetadataAccounts instruction
#[allow(clippy::too_many_arguments)]
/// #[deprecated(since="1.3.0", note="please use `create_metadata_accounts_v3` instead")]
pub fn create_metadata_accounts_v2(
    program_id: Pubkey,
    metadata_account: Pubkey,
    mint: Pubkey,
    mint_authority: Pubkey,
    payer: Pubkey,
    update_authority: Pubkey,
    name: String,
    symbol: String,
    uri: String,
    creators: Option<Vec<Creator>>,
    seller_fee_basis_points: u16,
    update_authority_is_signer: bool,
    is_mutable: bool,
    collection: Option<Collection>,
    uses: Option<Uses>,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(metadata_account, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(mint_authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(update_authority, update_authority_is_signer),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: MetadataInstruction::CreateMetadataAccountV2(CreateMetadataAccountArgsV2 {
            data: DataV2 {
                name,
                symbol,
                uri,
                seller_fee_basis_points,
                creators,
                collection,
                uses,
            },
            is_mutable,
        })
        .try_to_vec()
        .unwrap(),
    }
}

/// update metadata account instruction
/// #[deprecated(since="1.1.0", note="please use `update_metadata_accounts_v2` instead")]
pub fn update_metadata_accounts(
    program_id: Pubkey,
    metadata_account: Pubkey,
    update_authority: Pubkey,
    new_update_authority: Option<Pubkey>,
    data: Option<Data>,
    primary_sale_happened: Option<bool>,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(metadata_account, false),
            AccountMeta::new_readonly(update_authority, true),
        ],
        data: MetadataInstruction::UpdateMetadataAccount(UpdateMetadataAccountArgs {
            data,
            update_authority: new_update_authority,
            primary_sale_happened,
        })
        .try_to_vec()
        .unwrap(),
    }
}

// update metadata account v2 instruction
pub fn update_metadata_accounts_v2(
    program_id: Pubkey,
    metadata_account: Pubkey,
    update_authority: Pubkey,
    new_update_authority: Option<Pubkey>,
    data: Option<DataV2>,
    primary_sale_happened: Option<bool>,
    is_mutable: Option<bool>,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(metadata_account, false),
            AccountMeta::new_readonly(update_authority, true),
        ],
        data: MetadataInstruction::UpdateMetadataAccountV2(UpdateMetadataAccountArgsV2 {
            data,
            update_authority: new_update_authority,
            primary_sale_happened,
            is_mutable,
        })
        .try_to_vec()
        .unwrap(),
    }
}

/// puff metadata account instruction
pub fn puff_metadata_account(program_id: Pubkey, metadata_account: Pubkey) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![AccountMeta::new(metadata_account, false)],
        data: MetadataInstruction::PuffMetadata.try_to_vec().unwrap(),
    }
}

/// creates a update_primary_sale_happened_via_token instruction
#[allow(clippy::too_many_arguments)]
pub fn update_primary_sale_happened_via_token(
    program_id: Pubkey,
    metadata: Pubkey,
    owner: Pubkey,
    token: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(metadata, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(token, false),
        ],
        data: MetadataInstruction::UpdatePrimarySaleHappenedViaToken
            .try_to_vec()
            .unwrap(),
    }
}

/// creates a create_master_edition instruction
#[allow(clippy::too_many_arguments)]
/// [deprecated(since="1.1.0", note="please use `create_master_edition_v3` instead")]
pub fn create_master_edition(
    program_id: Pubkey,
    edition: Pubkey,
    mint: Pubkey,
    update_authority: Pubkey,
    mint_authority: Pubkey,
    metadata: Pubkey,
    payer: Pubkey,
    max_supply: Option<u64>,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(edition, false),
        AccountMeta::new(mint, false),
        AccountMeta::new_readonly(update_authority, true),
        AccountMeta::new_readonly(mint_authority, true),
        AccountMeta::new(payer, true),
        AccountMeta::new_readonly(metadata, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::CreateMasterEdition(CreateMasterEditionArgs { max_supply })
            .try_to_vec()
            .unwrap(),
    }
}

/// creates a create_master_edition instruction
#[allow(clippy::too_many_arguments)]
pub fn create_master_edition_v3(
    program_id: Pubkey,
    edition: Pubkey,
    mint: Pubkey,
    update_authority: Pubkey,
    mint_authority: Pubkey,
    metadata: Pubkey,
    payer: Pubkey,
    max_supply: Option<u64>,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(edition, false),
        AccountMeta::new(mint, false),
        AccountMeta::new_readonly(update_authority, true),
        AccountMeta::new_readonly(mint_authority, true),
        AccountMeta::new(payer, true),
        AccountMeta::new(metadata, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
    ];

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::CreateMasterEditionV3(CreateMasterEditionArgs { max_supply })
            .try_to_vec()
            .unwrap(),
    }
}

/// creates a mint_new_edition_from_master_edition instruction
#[allow(clippy::too_many_arguments)]
pub fn mint_new_edition_from_master_edition_via_token(
    program_id: Pubkey,
    new_metadata: Pubkey,
    new_edition: Pubkey,
    master_edition: Pubkey,
    new_mint: Pubkey,
    new_mint_authority: Pubkey,
    payer: Pubkey,
    token_account_owner: Pubkey,
    token_account: Pubkey,
    new_metadata_update_authority: Pubkey,
    metadata: Pubkey,
    metadata_mint: Pubkey,
    edition: u64,
) -> Instruction {
    let edition_number = edition.checked_div(EDITION_MARKER_BIT_SIZE).unwrap();
    let as_string = edition_number.to_string();
    let (edition_mark_pda, _) = Pubkey::find_program_address(
        &[
            PREFIX.as_bytes(),
            program_id.as_ref(),
            metadata_mint.as_ref(),
            EDITION.as_bytes(),
            as_string.as_bytes(),
        ],
        &program_id,
    );

    let accounts = vec![
        AccountMeta::new(new_metadata, false),
        AccountMeta::new(new_edition, false),
        AccountMeta::new(master_edition, false),
        AccountMeta::new(new_mint, false),
        AccountMeta::new(edition_mark_pda, false),
        AccountMeta::new_readonly(new_mint_authority, true),
        AccountMeta::new(payer, true),
        AccountMeta::new_readonly(token_account_owner, true),
        AccountMeta::new_readonly(token_account, false),
        AccountMeta::new_readonly(new_metadata_update_authority, false),
        AccountMeta::new_readonly(metadata, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
    ];

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::MintNewEditionFromMasterEditionViaToken(
            MintNewEditionFromMasterEditionViaTokenArgs { edition },
        )
        .try_to_vec()
        .unwrap(),
    }
}

/// Sign Metadata
#[allow(clippy::too_many_arguments)]
pub fn sign_metadata(program_id: Pubkey, metadata: Pubkey, creator: Pubkey) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(metadata, false),
            AccountMeta::new_readonly(creator, true),
        ],
        data: MetadataInstruction::SignMetadata.try_to_vec().unwrap(),
    }
}

/// Remove Creator Verificaton
#[allow(clippy::too_many_arguments)]
pub fn remove_creator_verification(
    program_id: Pubkey,
    metadata: Pubkey,
    creator: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(metadata, false),
            AccountMeta::new_readonly(creator, true),
        ],
        data: MetadataInstruction::RemoveCreatorVerification
            .try_to_vec()
            .unwrap(),
    }
}

/// Converts a master edition v1 to v2
#[allow(clippy::too_many_arguments)]
pub fn convert_master_edition_v1_to_v2(
    program_id: Pubkey,
    master_edition: Pubkey,
    one_time_auth: Pubkey,
    printing_mint: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(master_edition, false),
            AccountMeta::new(one_time_auth, false),
            AccountMeta::new(printing_mint, false),
        ],
        data: MetadataInstruction::ConvertMasterEditionV1ToV2
            .try_to_vec()
            .unwrap(),
    }
}

/// creates a mint_edition_proxy instruction
#[deprecated(since = "1.4.0")]
#[allow(clippy::too_many_arguments)]
pub fn mint_edition_from_master_edition_via_vault_proxy(
    program_id: Pubkey,
    new_metadata: Pubkey,
    new_edition: Pubkey,
    master_edition: Pubkey,
    new_mint: Pubkey,
    edition_mark_pda: Pubkey,
    new_mint_authority: Pubkey,
    payer: Pubkey,
    vault_authority: Pubkey,
    safety_deposit_store: Pubkey,
    safety_deposit_box: Pubkey,
    vault: Pubkey,
    new_metadata_update_authority: Pubkey,
    metadata: Pubkey,
    token_program: Pubkey,
    token_vault_program_info: Pubkey,
    edition: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(new_metadata, false),
        AccountMeta::new(new_edition, false),
        AccountMeta::new(master_edition, false),
        AccountMeta::new(new_mint, false),
        AccountMeta::new(edition_mark_pda, false),
        AccountMeta::new_readonly(new_mint_authority, true),
        AccountMeta::new(payer, true),
        AccountMeta::new_readonly(vault_authority, true),
        AccountMeta::new_readonly(safety_deposit_store, false),
        AccountMeta::new_readonly(safety_deposit_box, false),
        AccountMeta::new_readonly(vault, false),
        AccountMeta::new_readonly(new_metadata_update_authority, false),
        AccountMeta::new_readonly(metadata, false),
        AccountMeta::new_readonly(token_program, false),
        AccountMeta::new_readonly(token_vault_program_info, false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
    ];

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::MintNewEditionFromMasterEditionViaVaultProxy(
            MintNewEditionFromMasterEditionViaTokenArgs { edition },
        )
        .try_to_vec()
        .unwrap(),
    }
}

/// # Verify Collection
///
/// If a MetadataAccount Has a Collection allow the UpdateAuthority of the Collection to Verify the NFT Belongs in the Collection
///
/// ### Accounts:
///
///   0. `[writable]` Metadata account
///   1. `[signer]` Collection Update authority
///   2. `[signer]` payer
///   3. `[]` Mint of the Collection
///   4. `[]` Metadata Account of the Collection
///   5. `[]` MasterEdition2 Account of the Collection Token
#[allow(clippy::too_many_arguments)]
pub fn verify_collection(
    program_id: Pubkey,
    metadata: Pubkey,
    collection_authority: Pubkey,
    payer: Pubkey,
    collection_mint: Pubkey,
    collection: Pubkey,
    collection_master_edition_account: Pubkey,
    collection_authority_record: Option<Pubkey>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(metadata, false),
        AccountMeta::new(collection_authority, true),
        AccountMeta::new(payer, true),
        AccountMeta::new_readonly(collection_mint, false),
        AccountMeta::new_readonly(collection, false),
        AccountMeta::new_readonly(collection_master_edition_account, false),
    ];

    match collection_authority_record {
        Some(collection_authority_record) => {
            accounts.push(AccountMeta::new_readonly(
                collection_authority_record,
                false,
            ));
        }
        None => (),
    }

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::VerifyCollection.try_to_vec().unwrap(),
    }
}

/// # Unverify Collection
///
/// If a MetadataAccount Has a Collection allow an Authority of the Collection to unverify an NFT in a Collection
///
/// ### Accounts:
///
///   0. `[writable]` Metadata account
///   1. `[signer]` Collection Authority
///   2. `[signer]` payer
///   3. `[]` Mint of the Collection
///   4. `[]` Metadata Account of the Collection
///   5. `[]` MasterEdition2 Account of the Collection Token
#[allow(clippy::too_many_arguments)]
pub fn unverify_collection(
    program_id: Pubkey,
    metadata: Pubkey,
    collection_authority: Pubkey,
    collection_mint: Pubkey,
    collection: Pubkey,
    collection_master_edition_account: Pubkey,
    collection_authority_record: Option<Pubkey>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(metadata, false),
        AccountMeta::new(collection_authority, true),
        AccountMeta::new_readonly(collection_mint, false),
        AccountMeta::new_readonly(collection, false),
        AccountMeta::new_readonly(collection_master_edition_account, false),
    ];

    match collection_authority_record {
        Some(collection_authority_record) => {
            accounts.push(AccountMeta::new_readonly(
                collection_authority_record,
                false,
            ));
        }
        None => (),
    }

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::UnverifyCollection
            .try_to_vec()
            .unwrap(),
    }
}

///# Utilize
///
///Utilize or Use an NFT , burns the NFT and returns the lamports to the update authority if the use method is burn and its out of uses.
///Use Authority can be the Holder of the NFT, or a Delegated Use Authority.
///
///### Accounts:
///
///   0. `[writable]` Metadata account
///   1. `[writable]` Token Account Of NFT
///   2. `[writable]` Mint of the Metadata
///   2. `[signer]` A Use Authority / Can be the current Owner of the NFT
///   3. `[signer]` Payer
///   4. `[]` Owner
///   5. `[]` Token program
///   6. `[]` Associated Token program
///   7. `[]` System program
///   8. Optional `[]` Rent info
///   9. Optional `[writable]` Use Authority Record PDA If present the program Assumes a delegated use authority
#[allow(clippy::too_many_arguments)]
pub fn utilize(
    program_id: Pubkey,
    metadata: Pubkey,
    token_account: Pubkey,
    mint: Pubkey,
    use_authority_record_pda: Option<Pubkey>,
    use_authority: Pubkey,
    owner: Pubkey,
    burner: Option<Pubkey>,
    number_of_uses: u64,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(metadata, false),
        AccountMeta::new(token_account, false),
        AccountMeta::new(mint, false),
        AccountMeta::new(use_authority, true),
        AccountMeta::new_readonly(owner, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(spl_associated_token_account::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
    ];
    match use_authority_record_pda {
        Some(use_authority_record_pda) => {
            accounts.push(AccountMeta::new(use_authority_record_pda, false));
        }
        None => (),
    }

    match burner {
        Some(burner) => {
            accounts.push(AccountMeta::new_readonly(burner, false));
        }
        None => (),
    }

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::Utilize(UtilizeArgs { number_of_uses })
            .try_to_vec()
            .unwrap(),
    }
}

///# Approve Use Authority
///
///Approve another account to call [utilize] on this NFT
///
///### Args:
///
///See: [ApproveUseAuthorityArgs]
///
///### Accounts:
///
///   0. `[writable]` Use Authority Record PDA
///   1. `[writable]` Owned Token Account Of Mint
///   2. `[signer]` Owner
///   3. `[signer]` Payer
///   4. `[]` A Use Authority
///   5. `[]` Metadata account
///   6. `[]` Mint of Metadata
///   7. `[]` Program As Signer (Burner)
///   8. `[]` Token program
///   9. `[]` System program
///   10. Optional `[]` Rent info
#[allow(clippy::too_many_arguments)]
pub fn approve_use_authority(
    program_id: Pubkey,
    use_authority_record: Pubkey,
    user: Pubkey,
    owner: Pubkey,
    payer: Pubkey,
    owner_token_account: Pubkey,
    metadata: Pubkey,
    mint: Pubkey,
    burner: Pubkey,
    number_of_uses: u64,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(use_authority_record, false),
            AccountMeta::new(owner, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(user, false),
            AccountMeta::new(owner_token_account, false),
            AccountMeta::new_readonly(metadata, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(burner, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: MetadataInstruction::ApproveUseAuthority(ApproveUseAuthorityArgs { number_of_uses })
            .try_to_vec()
            .unwrap(),
    }
}

//# Revoke Use Authority
///
///Revoke account to call [utilize] on this NFT
///
///### Accounts:
///
///   0. `[writable]` Use Authority Record PDA
///   1. `[writable]` Owned Token Account Of Mint
///   2. `[signer]` Owner
///   3. `[signer]` Payer
///   4. `[]` A Use Authority
///   5. `[]` Metadata account
///   6. `[]` Mint of Metadata
///   7. `[]` Token program
///   8. `[]` System program
///   9. Optional `[]` Rent info
#[allow(clippy::too_many_arguments)]
pub fn revoke_use_authority(
    program_id: Pubkey,
    use_authority_record: Pubkey,
    user: Pubkey,
    owner: Pubkey,
    owner_token_account: Pubkey,
    metadata: Pubkey,
    mint: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(use_authority_record, false),
            AccountMeta::new(owner, true),
            AccountMeta::new_readonly(user, false),
            AccountMeta::new(owner_token_account, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(metadata, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: MetadataInstruction::RevokeUseAuthority
            .try_to_vec()
            .unwrap(),
    }
}

///# Approve Collection Authority
///
///Approve another account to verify NFTs belonging to a collection, [verify_collection] on the collection NFT
///
///### Accounts:
///   0. `[writable]` Collection Authority Record PDA
///   1. `[signer]` Update Authority of Collection NFT
///   2. `[signer]` Payer
///   3. `[]` A Collection Authority
///   4. `[]` Collection Metadata account
///   5. `[]` Mint of Collection Metadata
///   6. `[]` Token program
///   7. `[]` System program
///   8. Optional `[]` Rent info
#[allow(clippy::too_many_arguments)]
pub fn approve_collection_authority(
    program_id: Pubkey,
    collection_authority_record: Pubkey,
    new_collection_authority: Pubkey,
    update_authority: Pubkey,
    payer: Pubkey,
    metadata: Pubkey,
    mint: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(collection_authority_record, false),
            AccountMeta::new_readonly(new_collection_authority, false),
            AccountMeta::new(update_authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(metadata, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: MetadataInstruction::ApproveCollectionAuthority
            .try_to_vec()
            .unwrap(),
    }
}

//# Revoke Collection Authority
///
///Revoke account to call [verify_collection] on this NFT
///
///### Accounts:
///
///   0. `[writable]` Collection Authority Record PDA
///   1. `[writable]` The Authority that was delegated to
///   2. `[signer]` The Original Update Authority or Delegated Authority
///   2. `[]` Metadata account
///   3. `[]` Mint of Metadata
#[allow(clippy::too_many_arguments)]
pub fn revoke_collection_authority(
    program_id: Pubkey,
    collection_authority_record: Pubkey,
    delegate_authority: Pubkey,
    revoke_authority: Pubkey,
    metadata: Pubkey,
    mint: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(collection_authority_record, false),
            AccountMeta::new_readonly(delegate_authority, false),
            AccountMeta::new(revoke_authority, true),
            AccountMeta::new_readonly(metadata, false),
            AccountMeta::new_readonly(mint, false),
        ],
        data: MetadataInstruction::RevokeCollectionAuthority
            .try_to_vec()
            .unwrap(),
    }
}

//# Set And Verify Collection
///
///Allows the same Update Authority (Or Delegated Authority) on an NFT and Collection to perform [update_metadata_accounts_v2] with collection and [verify_collection] on the NFT/Collection in one instruction
///
/// ### Accounts:
///
///   0. `[writable]` Metadata account
///   1. `[signer]` Collection Update authority
///   2. `[signer]` payer
///   3. `[] Update Authority of Collection NFT and NFT
///   3. `[]` Mint of the Collection
///   4. `[]` Metadata Account of the Collection
///   5. `[]` MasterEdition2 Account of the Collection Token
#[allow(clippy::too_many_arguments)]
pub fn set_and_verify_collection(
    program_id: Pubkey,
    metadata: Pubkey,
    collection_authority: Pubkey,
    payer: Pubkey,
    update_authority: Pubkey,
    collection_mint: Pubkey,
    collection: Pubkey,
    collection_master_edition_account: Pubkey,
    collection_authority_record: Option<Pubkey>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(metadata, false),
        AccountMeta::new(collection_authority, true),
        AccountMeta::new(payer, true),
        AccountMeta::new_readonly(update_authority, false),
        AccountMeta::new_readonly(collection_mint, false),
        AccountMeta::new_readonly(collection, false),
        AccountMeta::new_readonly(collection_master_edition_account, false),
    ];

    match collection_authority_record {
        Some(collection_authority_record) => {
            accounts.push(AccountMeta::new_readonly(
                collection_authority_record,
                false,
            ));
        }
        None => (),
    }

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::SetAndVerifyCollection
            .try_to_vec()
            .unwrap(),
    }
}

///# Freeze delegated account
///
///Allow freezing of an NFT if this user is the delegate of the NFT
///
///### Accounts:
///   0. `[signer]` Delegate
///   1. `[writable]` Token account to freeze
///   2. `[]` Edition
///   3. `[]` Token mint
#[allow(clippy::too_many_arguments)]
pub fn freeze_delegated_account(
    program_id: Pubkey,
    delegate: Pubkey,
    token_account: Pubkey,
    edition: Pubkey,
    mint: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(delegate, true),
            AccountMeta::new(token_account, false),
            AccountMeta::new_readonly(edition, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: MetadataInstruction::FreezeDelegatedAccount
            .try_to_vec()
            .unwrap(),
    }
}

///# Thaw delegated account
///
///Allow thawing of an NFT if this user is the delegate of the NFT
///
///### Accounts:
///   0. `[signer]` Delegate
///   1. `[writable]` Token account to thaw
///   2. `[]` Edition
///   3. `[]` Token mint
#[allow(clippy::too_many_arguments)]
pub fn thaw_delegated_account(
    program_id: Pubkey,
    delegate: Pubkey,
    token_account: Pubkey,
    edition: Pubkey,
    mint: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(delegate, true),
            AccountMeta::new(token_account, false),
            AccountMeta::new_readonly(edition, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: MetadataInstruction::ThawDelegatedAccount
            .try_to_vec()
            .unwrap(),
    }
}

///# Burn NFT
///
/// Burn an NFT, closing its token, metadata and edition accounts.
///
/// 0. `[writable]` NFT metadata
/// 1. `[writable, signer]` Owner of NFT
/// 2. `[writable]` Mint of NFT
/// 3. `[writable]` NFT token account
/// 4. `[writable]` NFT edition account
/// 5. `[]` SPL Token program.
/// 6. Optional `[writable]` Collection metadata account
pub fn burn_nft(
    program_id: Pubkey,
    metadata: Pubkey,
    owner: Pubkey,
    mint: Pubkey,
    token: Pubkey,
    edition: Pubkey,
    spl_token: Pubkey,
    collection_metadata: Option<Pubkey>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(metadata, false),
        AccountMeta::new(owner, true),
        AccountMeta::new(mint, false),
        AccountMeta::new(token, false),
        AccountMeta::new(edition, false),
        AccountMeta::new_readonly(spl_token, false),
    ];

    if let Some(collection_metadata) = collection_metadata {
        accounts.push(AccountMeta::new(collection_metadata, false));
    }

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::BurnNft.try_to_vec().unwrap(),
    }
}

/// #[account(0, writable, name="metadata", desc="Metadata (pda of ['metadata', program id, mint id])")]
/// #[account(1, signer, writable, name="owner", desc="NFT owner")]
/// #[account(2, writable, name="print_edition_mint", desc="Mint of the print edition NFT")]
/// #[account(3, writable, name="master_edition_mint", desc="Mint of the original/master NFT")]
/// #[account(4, writable, name="print_edition_token_account", desc="Token account the print edition NFT is in")]
/// #[account(5, name="master_edition_token_account", desc="Token account the Master Edition NFT is in")]
/// #[account(6, writable, name="master_edition_account", desc="MasterEdition2 of the original NFT")]
/// #[account(7, writable, name="print_edition_account", desc="Print Edition account of the NFT")]
/// #[account(8, writable, name="edition_marker_account", desc="Edition Marker PDA of the NFT")]
/// #[account(9, name="spl token program", desc="SPL Token Program")]
pub fn burn_edition_nft(
    program_id: Pubkey,
    metadata: Pubkey,
    owner: Pubkey,
    print_edition_mint: Pubkey,
    master_edition_mint: Pubkey,
    print_edition_token: Pubkey,
    master_edition_token: Pubkey,
    master_edition: Pubkey,
    print_edition: Pubkey,
    edition_marker: Pubkey,
    spl_token: Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(metadata, false),
        AccountMeta::new(owner, true),
        AccountMeta::new(print_edition_mint, false),
        AccountMeta::new(master_edition_mint, false),
        AccountMeta::new(print_edition_token, false),
        AccountMeta::new(master_edition_token, false),
        AccountMeta::new(master_edition, false),
        AccountMeta::new(print_edition, false),
        AccountMeta::new(edition_marker, false),
        AccountMeta::new_readonly(spl_token, false),
    ];

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::BurnEditionNft.try_to_vec().unwrap(),
    }
}

/// # Verify Collection V2 -- Supports v1.3 Collection Details
///
/// If a MetadataAccount Has a Collection allow the UpdateAuthority of the Collection to Verify the NFT Belongs in the Collection
///
/// ### Accounts:
///
///   0. `[writable]` Metadata account
///   1. `[signer]` Collection Update authority
///   2. `[signer]` payer
///   3. `[]` Mint of the Collection
///   4. `[writable]` Metadata Account of the Collection
///   5. `[]` MasterEdition2 Account of the Collection Token
#[allow(clippy::too_many_arguments)]
pub fn verify_sized_collection_item(
    program_id: Pubkey,
    metadata: Pubkey,
    collection_authority: Pubkey,
    payer: Pubkey,
    collection_mint: Pubkey,
    collection: Pubkey,
    collection_master_edition_account: Pubkey,
    collection_authority_record: Option<Pubkey>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(metadata, false),
        AccountMeta::new_readonly(collection_authority, true),
        AccountMeta::new(payer, true),
        AccountMeta::new_readonly(collection_mint, false),
        AccountMeta::new(collection, false),
        AccountMeta::new_readonly(collection_master_edition_account, false),
    ];

    if let Some(record) = collection_authority_record {
        accounts.push(AccountMeta::new_readonly(record, false));
    }

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::VerifySizedCollectionItem
            .try_to_vec()
            .unwrap(),
    }
}

/// # Unverify Collection V2 -- Supports v1.3 Collection Details
///
/// If a MetadataAccount Has a Collection allow an Authority of the Collection to unverify an NFT in a Collection
///
/// ### Accounts:
///
///   0. `[writable]` Metadata account
///   1. `[signer]` Collection Authority
///   2. `[signer]` payer
///   3. `[]` Mint of the Collection
///   4. `[writable]` Metadata Account of the Collection
///   5. `[]` MasterEdition2 Account of the Collection Token
#[allow(clippy::too_many_arguments)]
pub fn unverify_sized_collection_item(
    program_id: Pubkey,
    metadata: Pubkey,
    collection_authority: Pubkey,
    payer: Pubkey,
    collection_mint: Pubkey,
    collection: Pubkey,
    collection_master_edition_account: Pubkey,
    collection_authority_record: Option<Pubkey>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(metadata, false),
        AccountMeta::new_readonly(collection_authority, true),
        AccountMeta::new(payer, true),
        AccountMeta::new_readonly(collection_mint, false),
        AccountMeta::new(collection, false),
        AccountMeta::new_readonly(collection_master_edition_account, false),
    ];

    match collection_authority_record {
        Some(collection_authority_record) => {
            accounts.push(AccountMeta::new_readonly(
                collection_authority_record,
                false,
            ));
        }
        None => (),
    }

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::UnverifySizedCollectionItem
            .try_to_vec()
            .unwrap(),
    }
}

//# Set And Verify Collection V2 -- Supports v1.3 Collection Details
///
///Allows the same Update Authority (Or Delegated Authority) on an NFT and Collection to perform [update_metadata_accounts_v2] with collection and [verify_collection] on the NFT/Collection in one instruction
///
/// ### Accounts:
///
///   0. `[writable]` Metadata account
///   1. `[signer]` Collection Update authority
///   2. `[signer]` payer
///   3. `[] Update Authority of Collection NFT and NFT
///   3. `[]` Mint of the Collection
///   4. `[writable]` Metadata Account of the Collection
///   5. `[]` MasterEdition2 Account of the Collection Token
#[allow(clippy::too_many_arguments)]
pub fn set_and_verify_sized_collection_item(
    program_id: Pubkey,
    metadata: Pubkey,
    collection_authority: Pubkey,
    payer: Pubkey,
    update_authority: Pubkey,
    collection_mint: Pubkey,
    collection: Pubkey,
    collection_master_edition_account: Pubkey,
    collection_authority_record: Option<Pubkey>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(metadata, false),
        AccountMeta::new(collection_authority, true),
        AccountMeta::new(payer, true),
        AccountMeta::new_readonly(update_authority, false),
        AccountMeta::new_readonly(collection_mint, false),
        AccountMeta::new(collection, false),
        AccountMeta::new_readonly(collection_master_edition_account, false),
    ];

    match collection_authority_record {
        Some(collection_authority_record) => {
            accounts.push(AccountMeta::new_readonly(
                collection_authority_record,
                false,
            ));
        }
        None => (),
    }

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::SetAndVerifySizedCollectionItem
            .try_to_vec()
            .unwrap(),
    }
}

//# Create Metadata Accounts V3 -- Supports v1.3 Collection Details
///
///Create a new Metadata Account
///
/// ### Accounts:
///
///   0. `[writable]` Metadata account
///   1. `[]` Mint account
///   2. `[signer]` Mint authority
///   3. `[signer]` payer
///   4. `[signer]` Update authority
///   5. `[]` System program
///   6. Optional `[]` Rent sysvar
///
/// Creates an CreateMetadataAccounts instruction
#[allow(clippy::too_many_arguments)]
pub fn create_metadata_accounts_v3(
    program_id: Pubkey,
    metadata_account: Pubkey,
    mint: Pubkey,
    mint_authority: Pubkey,
    payer: Pubkey,
    update_authority: Pubkey,
    name: String,
    symbol: String,
    uri: String,
    creators: Option<Vec<Creator>>,
    seller_fee_basis_points: u16,
    update_authority_is_signer: bool,
    is_mutable: bool,
    collection: Option<Collection>,
    uses: Option<Uses>,
    collection_details: Option<CollectionDetails>,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(metadata_account, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(mint_authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(update_authority, update_authority_is_signer),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: MetadataInstruction::CreateMetadataAccountV3(CreateMetadataAccountArgsV3 {
            data: DataV2 {
                name,
                symbol,
                uri,
                seller_fee_basis_points,
                creators,
                collection,
                uses,
            },
            is_mutable,
            collection_details,
        })
        .try_to_vec()
        .unwrap(),
    }
}

pub fn set_collection_size(
    program_id: Pubkey,
    metadata_account: Pubkey,
    update_authority: Pubkey,
    mint: Pubkey,
    collection_authority_record: Option<Pubkey>,
    size: u64,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(metadata_account, false),
        AccountMeta::new_readonly(update_authority, true),
        AccountMeta::new_readonly(mint, false),
    ];

    if let Some(record) = collection_authority_record {
        accounts.push(AccountMeta::new_readonly(record, false));
    }

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::SetCollectionSize(SetCollectionSizeArgs { size })
            .try_to_vec()
            .unwrap(),
    }
}

pub fn bubblegum_set_collection_size(
    program_id: Pubkey,
    metadata_account: Pubkey,
    update_authority: Pubkey,
    mint: Pubkey,
    bubblegum_signer: Pubkey,
    collection_authority_record: Option<Pubkey>,
    size: u64,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(metadata_account, false),
        AccountMeta::new_readonly(update_authority, true),
        AccountMeta::new_readonly(mint, false),
        AccountMeta::new_readonly(bubblegum_signer, true),
    ];

    if let Some(record) = collection_authority_record {
        accounts.push(AccountMeta::new_readonly(record, false));
    }

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::BubblegumSetCollectionSize(SetCollectionSizeArgs { size })
            .try_to_vec()
            .unwrap(),
    }
}

pub fn set_token_standard(
    program_id: Pubkey,
    metadata_account: Pubkey,
    update_authority: Pubkey,
    mint_account: Pubkey,
    edition_account: Option<Pubkey>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(metadata_account, false),
        AccountMeta::new(update_authority, true),
        AccountMeta::new_readonly(mint_account, false),
    ];
    let data = MetadataInstruction::SetTokenStandard.try_to_vec().unwrap();

    if let Some(edition_account) = edition_account {
        accounts.push(AccountMeta::new_readonly(edition_account, false));
    }

    Instruction {
        program_id,
        accounts,
        data,
    }
}
