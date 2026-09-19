//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Customer action protected by an OTP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OTPPurpose {
    #[serde(rename = "account_creation")]
    AccountCreation,
    #[serde(rename = "account_recovery")]
    AccountRecovery,
    #[serde(rename = "email_verification")]
    EmailVerification,
    #[serde(rename = "financial_account_verification")]
    FinancialAccountVerification,
    #[serde(rename = "password_reset")]
    PasswordReset,
    #[serde(rename = "payment_confirmation")]
    PaymentConfirmation,
    #[serde(rename = "payment_method_verification")]
    PaymentMethodVerification,
    #[serde(rename = "payout_confirmation")]
    PayoutConfirmation,
    #[serde(rename = "phone_verification")]
    PhoneVerification,
    #[serde(rename = "sensitive_action")]
    SensitiveAction,
    #[serde(rename = "sign_in")]
    SignIn,
    #[serde(rename = "transaction_confirmation")]
    TransactionConfirmation,
    #[serde(rename = "unspecified")]
    Unspecified,
}
