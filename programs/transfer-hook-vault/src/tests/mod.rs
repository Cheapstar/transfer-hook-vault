#[cfg(test)]
mod test {
    use crate::{
        constant::{VAULT, WHITELISTED_ENTRY},
        tests::test,
    };
    use anchor_lang::solana_program::program_pack::Pack;
    use anchor_spl::associated_token::{
        self, get_associated_token_address, spl_associated_token_account,
    };
    use litesvm_token::spl_token::{self, state::Account as Token2022Account, ID as TOKEN_PROGRAM};
    use spl_token_2022::ID as TOKEN_2022_PROGRAM;

    use {
        crate::instructions::MintConfig,
        anchor_lang::{prelude::*, InstructionData, ToAccountMetas},
        anchor_spl::associated_token::AssociatedToken,
        litesvm::LiteSVM,
        litesvm_token::{CreateAccount, CreateAssociatedTokenAccount, CreateMint},
        solana_instruction::Instruction,
        solana_keypair::Keypair,
        solana_message::Message,
        solana_native_token::LAMPORTS_PER_SOL,
        solana_pubkey::Pubkey,
        solana_sdk_ids::system_program::ID as SYSTEM_PROGRAM,
        solana_signer::Signer,
        solana_transaction::Transaction,
        spl_associated_token_account::get_associated_token_address_with_program_id,
        spl_associated_token_account::ID as ASSOCIATED_TOKEN_PROGRAM,
        std::path::PathBuf,
    };

    static PROGRAM_ID: Pubkey = crate::ID;
    const SECURITY_SEEDS: u64 = 123u64;

    pub struct TestConfig {
        admin: Keypair,
        user: Keypair,
        svm: LiteSVM,
        mint: Keypair,
        vault: Pubkey,
        admin_ata: Pubkey,
        user_ata: Pubkey,
        vault_ata: Pubkey,
        admin_vault_data: Pubkey,
        user_vault_data: Pubkey,
    }

    impl TestConfig {
        pub fn new() -> TestConfig {
            let mut svm = LiteSVM::new();

            let admin = Keypair::new();

            svm.airdrop(&admin.pubkey(), 1000 * LAMPORTS_PER_SOL)
                .expect("Failed to airdrop");

            let program_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../target/deploy/transfer_hook_vault.so");

            let program_data = std::fs::read(&program_path).expect("Failed to read program file");
            svm.add_program(PROGRAM_ID, &program_data);

            let user = Keypair::new();

            svm.airdrop(&user.pubkey(), 1000 * LAMPORTS_PER_SOL)
                .expect("Failed to airdrop");

            let mint = Keypair::new();

            let (vault, _) = Pubkey::find_program_address(
                &[VAULT.as_bytes(), SECURITY_SEEDS.to_le_bytes().as_ref()],
                &PROGRAM_ID,
            );
            let vault_ata = get_associated_token_address_with_program_id(
                &vault,
                &mint.pubkey(),
                &TOKEN_2022_PROGRAM,
            );
            let admin_ata = get_associated_token_address_with_program_id(
                &admin.pubkey(),
                &mint.pubkey(),
                &TOKEN_2022_PROGRAM,
            );
            let user_ata = get_associated_token_address_with_program_id(
                &user.pubkey(),
                &mint.pubkey(),
                &TOKEN_2022_PROGRAM,
            );

            let (admin_vault_data, _) = Pubkey::find_program_address(
                &[
                    WHITELISTED_ENTRY.as_bytes(),
                    admin.pubkey().as_ref(),
                    mint.pubkey().as_ref(),
                    SECURITY_SEEDS.to_le_bytes().as_ref(),
                ],
                &PROGRAM_ID,
            );
            let (user_vault_data, _) = Pubkey::find_program_address(
                &[
                    WHITELISTED_ENTRY.as_bytes(),
                    user.pubkey().as_ref(),
                    mint.pubkey().as_ref(),
                    SECURITY_SEEDS.to_le_bytes().as_ref(),
                ],
                &PROGRAM_ID,
            );

            println!("================ PDA / ATA DEBUG ================");

            println!("PROGRAM_ID          : {}", PROGRAM_ID);
            println!("SECURITY_SEEDS (u64): {}", SECURITY_SEEDS);
            println!("SECURITY_SEEDS (le) : {:?}", SECURITY_SEEDS.to_le_bytes());

            println!("MINT PUBKEY         : {}", mint.pubkey());
            println!("VAULT PDA           : {}", vault);
            println!("VAULT SEED STRING   : {:?}", VAULT.as_bytes());

            println!("--- ATAs (Token-2022) ---");
            println!("VAULT ATA           : {}", vault_ata);
            println!("ADMIN ATA           : {}", admin_ata);
            println!("USER ATA            : {}", user_ata);

            println!("--- USERS ---");
            println!("ADMIN PUBKEY        : {}", admin.pubkey());
            println!("USER PUBKEY         : {}", user.pubkey());

            println!("--- WHITELIST PDAs ---");
            println!("ADMIN VAULT DATA PDA: {}", admin_vault_data);
            println!("USER  VAULT DATA PDA: {}", user_vault_data);

            println!("WHITELISTED_ENTRY   : {:?}", WHITELISTED_ENTRY.as_bytes());

            println!("=================================================");

            TestConfig {
                admin,
                user,
                svm,
                mint,
                vault,
                admin_ata,
                user_ata,
                vault_ata,
                admin_vault_data,
                user_vault_data,
            }
        }

        pub fn init_mint_ix(&mut self) {
            println!("Creating Mint ...");
            let ix = Instruction {
                program_id: PROGRAM_ID,
                accounts: crate::accounts::InitMint {
                    authority: self.admin.pubkey(),
                    mint: self.mint.pubkey(),
                    associated_token_program: ASSOCIATED_TOKEN_PROGRAM,
                    token_program: TOKEN_2022_PROGRAM,
                    system_program: SYSTEM_PROGRAM,
                }
                .to_account_metas(None)
                .to_vec(),
                data: crate::instruction::InitMint {
                    mint_config: MintConfig {
                        decimals: 6,
                        mint_authority: self.admin.pubkey(),
                        freeze_authority: self.admin.pubkey(),
                        transfer_hook_authority: PROGRAM_ID,
                    },
                }
                .data(),
            };

            let message = Message::new(&[ix], Some(&self.admin.pubkey()));
            let tx = Transaction::new(
                &[&self.admin, &self.mint],
                message,
                self.svm.latest_blockhash(),
            );

            let result = self.svm.send_transaction(tx).unwrap();
            for log in &result.logs {
                println!("{}", log);
            }
            println!("{}", result.pretty_logs());

            println!("Mint Created Successfully {}", self.mint.pubkey());
        }

        pub fn init_vault(&mut self) {
            println!("[Test] Initialize Vault Transaction ...");

            let init_ix = Instruction {
                program_id: PROGRAM_ID,
                accounts: crate::accounts::InitializeVault {
                    admin: self.admin.pubkey(),
                    vault: self.vault,
                    mint: self.mint.pubkey(),
                    vault_ata: self.vault_ata,
                    associated_token_program: ASSOCIATED_TOKEN_PROGRAM,
                    token_program: TOKEN_2022_PROGRAM,
                    system_program: SYSTEM_PROGRAM,
                }
                .to_account_metas(None)
                .to_vec(),
                data: crate::instruction::InitVault {
                    seeds: SECURITY_SEEDS,
                }
                .data(),
            };

            let message = Message::new(&[init_ix], Some(&self.admin.pubkey()));

            let tx = Transaction::new(&[&self.admin], message, self.svm.latest_blockhash());

            let result = self.svm.send_transaction(tx).unwrap();
            for log in &result.logs {
                println!("{}", log);
            }
            println!("{}", result.pretty_logs());

            println!("[Test] Initialize Vault Transaction Successfull");
        }

        pub fn mint_tokens(&mut self, recipient: Pubkey) {
            println!("[Test] Minting Tokens");
            let mint_tokens_ix = Instruction {
                program_id: PROGRAM_ID,
                accounts: crate::accounts::MintTokens {
                    mint_authority: self.admin.pubkey(),
                    recipient: self.admin.pubkey(),
                    mint: self.mint.pubkey(),
                    recipient_ata: recipient,
                    associated_token_program: ASSOCIATED_TOKEN_PROGRAM,
                    token_program: TOKEN_2022_PROGRAM,
                    system_program: SYSTEM_PROGRAM,
                }
                .to_account_metas(None)
                .to_vec(),
                data: crate::instruction::MintTokens { amount: 1000 }.data(),
            };

            let message = Message::new(&[mint_tokens_ix], Some(&self.admin.pubkey()));

            let tx = Transaction::new(&[&self.admin], message, self.svm.latest_blockhash());

            let result = self.svm.send_transaction(tx).unwrap();
            for log in &result.logs {
                println!("{}", log);
            }
            println!("{}", result.pretty_logs());

            println!("[Test] Tokens Minted Successfully");
        }

        pub fn add_user(&mut self, whitelisted_user: Pubkey, whitelist_user_data: Pubkey) {
            println!("[Test] Whitelist User");
            let add_user_ix = Instruction {
                program_id: PROGRAM_ID,
                accounts: crate::accounts::AddUser {
                    system_program: SYSTEM_PROGRAM,
                    admin: self.admin.pubkey(),
                    mint: self.mint.pubkey(),
                    vault: self.vault,
                    user_vault_data: whitelist_user_data,
                }
                .to_account_metas(None)
                .to_vec(),
                data: crate::instruction::AddToWhitelist {
                    user: whitelisted_user,
                    seeds: SECURITY_SEEDS,
                }
                .data(),
            };

            let message = Message::new(&[add_user_ix], Some(&self.admin.pubkey()));

            let tx = Transaction::new(&[&self.admin], message, self.svm.latest_blockhash());

            let result = self.svm.send_transaction(tx).unwrap();
            for log in &result.logs {
                println!("{}", log);
            }
            println!("{}", result.pretty_logs());

            println!("[Test] User Whitelisted Successfully");
        }

        pub fn deposit(&mut self, deposit_amount: u64, is_admin: bool) {
            println!("[Test] Deposit Tokens");

            let (deposit_user, deposit_user_ata, deposit_user_data) = if is_admin {
                (&self.admin, self.admin_ata, self.admin_vault_data)
            } else {
                (&self.user, self.user_ata, self.user_vault_data)
            };

            let deposit_tokens_ix = Instruction {
                program_id: PROGRAM_ID,
                accounts: crate::accounts::Deposit {
                    user: deposit_user.pubkey(),
                    mint: self.mint.pubkey(),
                    vault: self.vault,
                    user_ata: deposit_user_ata,
                    user_vault_data: deposit_user_data,
                    vault_ata: self.vault_ata,
                    associated_token_program: ASSOCIATED_TOKEN_PROGRAM,
                    token_program: TOKEN_2022_PROGRAM,
                    system_program: SYSTEM_PROGRAM,
                }
                .to_account_metas(None)
                .to_vec(),
                data: crate::instruction::Deposit {
                    deposit_amount: deposit_amount,
                }
                .data(),
            };

            let message = Message::new(&[deposit_tokens_ix], Some(&deposit_user.pubkey()));

            let tx = Transaction::new(&[deposit_user], message, self.svm.latest_blockhash());

            let result = self.svm.send_transaction(tx).unwrap();
            for log in &result.logs {
                println!("{}", log);
            }
            println!("{}", result.pretty_logs());

            println!("[Test] User Whitelisted Successfully");
        }

        pub fn assert_ata(&self, owner: &Pubkey, mint: &Pubkey, ata: &Pubkey, amount: u64) {
            let ata_account = self.svm.get_account(ata).unwrap();
            let ata_data = read_token_amount(&ata_account.data);

            assert_eq!(ata_data, amount);
        }
    }

    pub fn create_ata(svm: &mut LiteSVM, payer: &Keypair, mint: &Pubkey) -> Pubkey {
        println!("Creating ATA");
        let ata = CreateAssociatedTokenAccount::new(svm, payer, mint)
            .owner(&payer.pubkey())
            .send()
            .unwrap();

        println!("Associated Token Account Created");
        ata
    }

    // #[test]
    // pub fn init_mint_test() {
    //     println!("[Init Mint] : Testing ");
    //     let mut test_config = TestConfig::new();

    //     let mint = test_config.init_mint_ix();

    //     println!("[Init Mint] : Succesful");
    // }

    // #[test]
    // pub fn init_vault() {
    //     println!("[Init Vault] : Testing ");
    //     let mut test_config = TestConfig::new();

    //     test_config.init_mint_ix();
    //     test_config.init_vault();

    //     let vault_account = test_config.svm.get_account(&test_config.vault).unwrap();
    //     let vault_data =
    //         crate::state::Vault::try_deserialize(&mut vault_account.data.as_ref()).unwrap();

    //     println!("Vault \n");
    //     println!("{{");
    //     println!("\tadmin : {}", vault_data.admin);
    //     println!("\tmint : {}", vault_data.mint);
    //     println!("\tamount : {}", vault_data.amount);
    //     println!("\tseeds : {}", vault_data.seeds);
    //     println!("\tnumber_of_users : {}", vault_data.number_of_users);
    //     println!("}}");

    //     println!("[Init Vault] : Succesfull");
    // }

    // #[test]
    // pub fn mint_tokens() {
    //     println!("[Mint Tokens] : Testing ");
    //     let mut test_config = TestConfig::new();

    //     test_config.init_mint_ix();
    //     test_config.mint_tokens(test_config.admin_ata);
    //     test_config.assert_ata(
    //         &test_config.admin_ata,
    //         &test_config.mint.pubkey(),
    //         &test_config.admin_ata,
    //         1000,
    //     );

    //     println!("[Mint Tokens] : Succesfull");
    // }
    // #[test]
    // pub fn whitelist_user() {
    //     println!("[Whitelist User] : Testing ");
    //     let mut test_config = TestConfig::new();

    //     test_config.init_mint_ix();
    //     test_config.init_vault();
    //     test_config.add_user(test_config.admin.pubkey(), test_config.admin_vault_data);

    //     println!("[Whitelist user] : Succesfull");
    // }

    #[test]
    pub fn deposit_test() {
        println!("[Deposit Tokens] : Testing ");

        let mut test_config = TestConfig::new();
        let result_pda =
            get_associated_token_address(&test_config.admin.pubkey(), &test_config.mint.pubkey());
        println!("Expected {}", result_pda);
        let deposit_amount = 500;
        test_config.init_mint_ix();
        test_config.init_vault();
        test_config.mint_tokens(test_config.admin_ata);
        test_config.add_user(test_config.admin.pubkey(), test_config.admin_vault_data);

        println!("VAULT : {}", test_config.vault);

        test_config.deposit(deposit_amount, true);

        test_config.assert_ata(
            &test_config.vault,
            &test_config.mint.pubkey(),
            &test_config.vault_ata,
            deposit_amount,
        );

        // had 1000 tokens now 500 deposited , i am tired
        test_config.assert_ata(
            &test_config.admin.pubkey(),
            &test_config.mint.pubkey(),
            &test_config.admin_ata,
            deposit_amount,
        );

        println!("[Deposit Tokens] : Succesfull");
    }

    /// this is because of the dependency error
    fn read_token_amount(data: &[u8]) -> u64 {
        // TokenAccount.amount offset = 64
        // owner (32) + mint (32)
        let amount_offset = 64;
        let amount_bytes = &data[amount_offset..amount_offset + 8];
        u64::from_le_bytes(amount_bytes.try_into().unwrap())
    }
}
