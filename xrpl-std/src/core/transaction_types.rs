// Complete list of XRPL transaction type constants
// From Transactions.macro

// ============== Payment and Transfer Transactions ==============
pub const TT_PAYMENT: i32 = 0;                           // Payment
pub const TT_ESCROW_CREATE: i32 = 1;                     // EscrowCreate
pub const TT_ESCROW_FINISH: i32 = 2;                     // EscrowFinish
pub const TT_ACCOUNT_SET: i32 = 3;                       // AccountSet
pub const TT_ESCROW_CANCEL: i32 = 4;                     // EscrowCancel
pub const TT_REGULAR_KEY_SET: i32 = 5;                   // SetRegularKey
// 6 is deprecated
pub const TT_OFFER_CREATE: i32 = 7;                      // OfferCreate
pub const TT_OFFER_CANCEL: i32 = 8;                      // OfferCancel
// 9 is deprecated
pub const TT_TICKET_CREATE: i32 = 10;                    // TicketCreate
// 11 is deprecated
pub const TT_SIGNER_LIST_SET: i32 = 12;                  // SignerListSet

// ============== Payment Channel Transactions ==============
pub const TT_PAYCHAN_CREATE: i32 = 13;                   // PaymentChannelCreate
pub const TT_PAYCHAN_FUND: i32 = 14;                     // PaymentChannelFund
pub const TT_PAYCHAN_CLAIM: i32 = 15;                    // PaymentChannelClaim

// ============== Check Transactions ==============
pub const TT_CHECK_CREATE: i32 = 16;                     // CheckCreate
pub const TT_CHECK_CASH: i32 = 17;                       // CheckCash
pub const TT_CHECK_CANCEL: i32 = 18;                     // CheckCancel

// ============== Authorization Transactions ==============
pub const TT_DEPOSIT_PREAUTH: i32 = 19;                  // DepositPreauth
pub const TT_TRUST_SET: i32 = 20;                        // TrustSet
pub const TT_ACCOUNT_DELETE: i32 = 21;                   // AccountDelete
// 22 is reserved

// ============== NFT Transactions ==============
pub const TT_NFTOKEN_MINT: i32 = 25;                     // NFTokenMint
pub const TT_NFTOKEN_BURN: i32 = 26;                     // NFTokenBurn
pub const TT_NFTOKEN_CREATE_OFFER: i32 = 27;             // NFTokenCreateOffer
pub const TT_NFTOKEN_CANCEL_OFFER: i32 = 28;             // NFTokenCancelOffer
pub const TT_NFTOKEN_ACCEPT_OFFER: i32 = 29;             // NFTokenAcceptOffer

// ============== Clawback Transactions ==============
pub const TT_CLAWBACK: i32 = 30;                         // Clawback
pub const TT_AMM_CLAWBACK: i32 = 31;                     // AMMClawback

// ============== AMM (Automated Market Maker) Transactions ==============
pub const TT_AMM_CREATE: i32 = 35;                       // AMMCreate
pub const TT_AMM_DEPOSIT: i32 = 36;                      // AMMDeposit
pub const TT_AMM_WITHDRAW: i32 = 37;                     // AMMWithdraw
pub const TT_AMM_VOTE: i32 = 38;                         // AMMVote
pub const TT_AMM_BID: i32 = 39;                          // AMMBid
pub const TT_AMM_DELETE: i32 = 40;                       // AMMDelete

// ============== Cross-Chain Bridge Transactions ==============
pub const TT_XCHAIN_CREATE_CLAIM_ID: i32 = 41;           // XChainCreateClaimID
pub const TT_XCHAIN_COMMIT: i32 = 42;                    // XChainCommit
pub const TT_XCHAIN_CLAIM: i32 = 43;                     // XChainClaim
pub const TT_XCHAIN_ACCOUNT_CREATE_COMMIT: i32 = 44;     // XChainAccountCreateCommit
pub const TT_XCHAIN_ADD_CLAIM_ATTESTATION: i32 = 45;     // XChainAddClaimAttestation
pub const TT_XCHAIN_ADD_ACCOUNT_CREATE_ATTESTATION: i32 = 46; // XChainAddAccountCreateAttestation
pub const TT_XCHAIN_MODIFY_BRIDGE: i32 = 47;             // XChainModifyBridge
pub const TT_XCHAIN_CREATE_BRIDGE: i32 = 48;             // XChainCreateBridge

// ============== DID Transactions ==============
pub const TT_DID_SET: i32 = 49;                          // DIDSet
pub const TT_DID_DELETE: i32 = 50;                       // DIDDelete

// ============== Oracle Transactions ==============
pub const TT_ORACLE_SET: i32 = 51;                       // OracleSet
pub const TT_ORACLE_DELETE: i32 = 52;                    // OracleDelete

// ============== Ledger State Fix ==============
pub const TT_LEDGER_STATE_FIX: i32 = 53;                 // LedgerStateFix

// ============== Multi-Purpose Token (MPT) Transactions ==============
pub const TT_MPTOKEN_ISSUANCE_CREATE: i32 = 54;          // MPTokenIssuanceCreate
pub const TT_MPTOKEN_ISSUANCE_DESTROY: i32 = 55;         // MPTokenIssuanceDestroy
pub const TT_MPTOKEN_ISSUANCE_SET: i32 = 56;             // MPTokenIssuanceSet
pub const TT_MPTOKEN_AUTHORIZE: i32 = 57;                // MPTokenAuthorize

// ============== Credential Transactions ==============
pub const TT_CREDENTIAL_CREATE: i32 = 58;                // CredentialCreate
pub const TT_CREDENTIAL_ACCEPT: i32 = 59;                // CredentialAccept
pub const TT_CREDENTIAL_DELETE: i32 = 60;                // CredentialDelete

// ============== NFT Modification ==============
pub const TT_NFTOKEN_MODIFY: i32 = 61;                   // NFTokenModify

// ============== Permissioned Domain Transactions ==============
pub const TT_PERMISSIONED_DOMAIN_SET: i32 = 62;          // PermissionedDomainSet
pub const TT_PERMISSIONED_DOMAIN_DELETE: i32 = 63;       // PermissionedDomainDelete

// ============== Delegation Transactions ==============
pub const TT_DELEGATE_SET: i32 = 64;                     // DelegateSet

// ============== Vault Transactions ==============
pub const TT_VAULT_CREATE: i32 = 65;                     // VaultCreate
pub const TT_VAULT_SET: i32 = 66;                        // VaultSet
pub const TT_VAULT_DELETE: i32 = 67;                     // VaultDelete
pub const TT_VAULT_DEPOSIT: i32 = 68;                    // VaultDeposit
pub const TT_VAULT_WITHDRAW: i32 = 69;                   // VaultWithdraw
pub const TT_VAULT_CLAWBACK: i32 = 70;                   // VaultClawback

// ============== Batch Transaction ==============
pub const TT_BATCH: i32 = 71;                            // Batch

// ============== Smart Contract Transactions ==============
pub const TT_CONTRACT_CREATE: i32 = 72;                  // ContractCreate
pub const TT_CONTRACT_MODIFY: i32 = 73;                  // ContractModify
pub const TT_CONTRACT_DELETE: i32 = 74;                  // ContractDelete
pub const TT_CONTRACT_CLAWBACK: i32 = 75;                // ContractClawback
pub const TT_CONTRACT_USER_DELETE: i32 = 76;             // ContractUserDelete
pub const TT_CONTRACT_CALL: i32 = 77;                    // ContractCall

// ============== System-Generated Transactions ==============
pub const TT_AMENDMENT: i32 = 100;                       // EnableAmendment
pub const TT_FEE: i32 = 101;                            // SetFee
pub const TT_UNL_MODIFY: i32 = 102;                     // UNLModify