use crate::*;
use borsh::ser::BorshSerialize;
use mpl_token_metadata::{
    id,
    instruction::{self, CreateMasterEditionArgs, MetadataInstruction},
    state::{EDITION, PREFIX},
};
use solana_program::{
    borsh::try_from_slice_unchecked,
    instruction::{AccountMeta, Instruction},
    sysvar,
};

use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

#[derive(Debug)]
pub struct MasterEditionV2 {
    pub pubkey: Pubkey,
    pub metadata_pubkey: Pubkey,
    pub mint_pubkey: Pubkey,
}

impl MasterEditionV2 {
    pub fn new(metadata: &Metadata) -> Self {
        let program_id = id();
        let mint_pubkey = metadata.mint.pubkey();

        let master_edition_seeds = &[
            PREFIX.as_bytes(),
            program_id.as_ref(),
            mint_pubkey.as_ref(),
            EDITION.as_bytes(),
        ];
        let (pubkey, _) = Pubkey::find_program_address(master_edition_seeds, &id());

        MasterEditionV2 {
            pubkey,
            metadata_pubkey: metadata.pubkey,
            mint_pubkey,
        }
    }

    pub async fn get_data(
        &self,
        context: &mut ProgramTestContext,
    ) -> mpl_token_metadata::state::MasterEditionV2 {
        let account = get_account(context, &self.pubkey).await;
        try_from_slice_unchecked(&account.data).unwrap()
    }

    pub async fn get_data_from_account(
        context: &mut ProgramTestContext,
        pubkey: &Pubkey,
    ) -> mpl_token_metadata::state::MasterEditionV2 {
        let account = get_account(context, pubkey).await;
        try_from_slice_unchecked(&account.data).unwrap()
    }

    pub async fn create_with_invalid_token_program(
        &self,
        context: &mut ProgramTestContext,
        max_supply: Option<u64>,
    ) -> Result<(), BanksClientError> {
        let fake_token_program = Keypair::new();

        let fake_instruction = Instruction {
            program_id: mpl_token_metadata::id(),
            accounts: vec![
                AccountMeta::new(self.pubkey, false),
                AccountMeta::new(self.mint_pubkey, false),
                AccountMeta::new_readonly(context.payer.pubkey(), true),
                AccountMeta::new_readonly(context.payer.pubkey(), true),
                AccountMeta::new_readonly(context.payer.pubkey(), true),
                AccountMeta::new_readonly(self.metadata_pubkey, false),
                AccountMeta::new_readonly(fake_token_program.pubkey(), false),
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
                AccountMeta::new_readonly(sysvar::rent::id(), false),
            ],
            data: MetadataInstruction::CreateMasterEdition(CreateMasterEditionArgs { max_supply })
                .try_to_vec()
                .unwrap(),
        };

        let tx = Transaction::new_signed_with_payer(
            &[fake_instruction],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await
    }

    pub async fn create(
        &self,
        context: &mut ProgramTestContext,
        max_supply: Option<u64>,
    ) -> Result<(), BanksClientError> {
        let tx = Transaction::new_signed_with_payer(
            &[instruction::create_master_edition(
                id(),
                self.pubkey,
                self.mint_pubkey,
                context.payer.pubkey(),
                context.payer.pubkey(),
                self.metadata_pubkey,
                context.payer.pubkey(),
                max_supply,
            )],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await
    }

    pub async fn create_v3(
        &self,
        context: &mut ProgramTestContext,
        max_supply: Option<u64>,
    ) -> Result<(), BanksClientError> {
        let tx = Transaction::new_signed_with_payer(
            &[instruction::create_master_edition_v3(
                id(),
                self.pubkey,
                self.mint_pubkey,
                context.payer.pubkey(),
                context.payer.pubkey(),
                self.metadata_pubkey,
                context.payer.pubkey(),
                max_supply,
            )],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await
    }

    pub async fn mint_editions(
        &self,
        context: &mut ProgramTestContext,
        nft: &Metadata,
        number: u64,
    ) -> Result<Vec<EditionMarker>, BanksClientError> {
        let mut editions = Vec::new();

        for i in 1..=number {
            let print_edition = EditionMarker::new(nft, self, i);
            print_edition.create(context).await?;
            editions.push(print_edition);
        }

        Ok(editions)
    }
}
