//! Generated, typed Inttegro domain and request values.
// This file is generated from the canonical SDK contract. Do not edit manually.

use crate::{
    CustomData, CustomDataInput, CustomDataPatch, CustomerBalance, DoshAccount, FileMetadata,
    FinancialAccountVerification, JsonData, MessageHeaders, OrderPayoutSettings,
    PayoutDestinations, ProductDimensionDetails, Shipping, VariantValues,
};
use serde::{Deserialize, Serialize};

/// A typed `AppCredentialOwner` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppCredentialOwner {
    #[serde(rename = "child")]
    Child,
    #[serde(rename = "parent")]
    Parent,
}

/// A typed `AppManagementRole` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppManagementRole {
    #[serde(rename = "parent")]
    Parent,
    #[serde(rename = "child")]
    Child,
}

/// A typed `AppRelationshipKind` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppRelationshipKind {
    #[serde(rename = "placement")]
    Placement,
}

/// A typed `AppRelationshipStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppRelationshipStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "revoked")]
    Revoked,
}

/// A typed `BalanceTransactionType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BalanceTransactionType {
    #[serde(rename = "payment")]
    Payment,
    #[serde(rename = "refund")]
    Refund,
}

/// A typed `BankAccountType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BankAccountType {
    #[serde(rename = "ghana_bank_account")]
    GhanaBankAccount,
}

/// A typed `CheckoutOrderStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CheckoutOrderStatus {
    #[serde(rename = "preparing")]
    Preparing,
    #[serde(rename = "requires_payment")]
    RequiresPayment,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "expired")]
    Expired,
}

/// A typed `CheckoutPaymentStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CheckoutPaymentStatus {
    #[serde(rename = "requires_action")]
    RequiresAction,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}

/// A typed `ChimeEmailSchemaKind` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChimeEmailSchemaKind {
    #[serde(rename = "gmail_view_action")]
    GmailViewAction,
    #[serde(rename = "schema_org_order")]
    SchemaOrgOrder,
    #[serde(rename = "schema_org_invoice")]
    SchemaOrgInvoice,
}

/// A typed `ChimeRecipientType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChimeRecipientType {
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "email")]
    Email,
}

/// A typed `ChimeTransport` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChimeTransport {
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "email")]
    Email,
}

/// A typed `ContentSafetyStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentSafetyStatus {
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "quarantined")]
    Quarantined,
}

/// A typed `Currency` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "ghs")]
    Ghs,
    #[serde(rename = "usd")]
    Usd,
    #[serde(rename = "gbp")]
    Gbp,
    #[serde(rename = "eur")]
    Eur,
    #[serde(rename = "cny")]
    Cny,
}

/// A typed `DeliveryChannel` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeliveryChannel {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "sms")]
    Sms,
}

/// A typed `FileDelivery` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileDelivery {
    #[serde(rename = "stream")]
    Stream,
    #[serde(rename = "redirect")]
    Redirect,
}

/// A typed `FileDisposition` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileDisposition {
    #[serde(rename = "attachment")]
    Attachment,
    #[serde(rename = "inline")]
    Inline,
}

/// A typed `FileLinkDeliveryMode` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileLinkDeliveryMode {
    #[serde(rename = "redirect")]
    Redirect,
    #[serde(rename = "download")]
    Download,
    #[serde(rename = "inline")]
    Inline,
}

/// A typed `FileLinkKind` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileLinkKind {
    #[serde(rename = "public")]
    Public,
}

/// A typed `FileLinkStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileLinkStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "disabled")]
    Disabled,
}

/// A typed `FileScanStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileScanStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "passed")]
    Passed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "skipped")]
    Skipped,
}

/// A typed `FileSourceType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileSourceType {
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "upload_request")]
    UploadRequest,
    #[serde(rename = "service")]
    Service,
}

/// A typed `FileStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileStatus {
    #[serde(rename = "uploading")]
    Uploading,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "deleted")]
    Deleted,
}

/// A typed `FileStorageEncoding` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileStorageEncoding {
    #[serde(rename = "identity")]
    Identity,
    #[serde(rename = "br")]
    Brotli,
}

/// A typed `FinancialAccountType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FinancialAccountType {
    #[serde(rename = "wallet")]
    Wallet,
    #[serde(rename = "bank_account")]
    BankAccount,
    #[serde(rename = "dosh_account")]
    DoshAccount,
}

/// A typed `LineItemType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LineItemType {
    #[serde(rename = "product")]
    Product,
    #[serde(rename = "fee")]
    Fee,
    #[serde(rename = "shipping")]
    Shipping,
}

/// A typed `MessageTemplateChannel` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageTemplateChannel {
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "email")]
    Email,
}

/// A typed `MessageTemplateStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageTemplateStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "published")]
    Published,
    #[serde(rename = "archived")]
    Archived,
}

/// A typed `MessageTemplateVariableItemType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageTemplateVariableItemType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "datetime")]
    Datetime,
}

/// A typed `MessageTemplateVariableType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageTemplateVariableType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "datetime")]
    Datetime,
    #[serde(rename = "array")]
    Array,
}

/// A typed `MobileMoneyNetwork` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MobileMoneyNetwork {
    #[serde(rename = "airtel")]
    Airtel,
    #[serde(rename = "mtn")]
    Mtn,
    #[serde(rename = "telecel")]
    Telecel,
    #[serde(rename = "vodafone")]
    Vodafone,
}

/// A typed `OTPAlphabetType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OTPAlphabetType {
    #[serde(rename = "numeric")]
    Numeric,
    #[serde(rename = "alpha")]
    Alpha,
    #[serde(rename = "alphanumeric")]
    Alphanumeric,
}

/// A typed `OTPStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OTPStatus {
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "pending_delivery")]
    PendingDelivery,
    #[serde(rename = "pending_verification")]
    PendingVerification,
    #[serde(rename = "verified")]
    Verified,
}

/// A typed `OTPTransmissionStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OTPTransmissionStatus {
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "submitted")]
    Submitted,
}

/// A typed `OTPVerificationVerdict` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OTPVerificationVerdict {
    #[serde(rename = "fail")]
    Fail,
    #[serde(rename = "pass")]
    Pass,
}

/// A typed `OrderCreatedFromResourceType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderCreatedFromResourceType {
    #[serde(rename = "purchase_intent")]
    PurchaseIntent,
}

/// A typed `OrderDocumentKind` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderDocumentKind {
    #[serde(rename = "invoice")]
    Invoice,
    #[serde(rename = "receipt")]
    Receipt,
}

/// A typed `OrderStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderStatus {
    #[serde(rename = "preparing")]
    Preparing,
    #[serde(rename = "requires_payment")]
    RequiresPayment,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "unknown")]
    Unknown,
}

/// A typed `PaymentAttemptStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentAttemptStatus {
    #[serde(rename = "initiated")]
    Initiated,
    #[serde(rename = "executed")]
    Executed,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "unknown")]
    Unknown,
}

/// A typed `PaymentConfirmationChannel` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentConfirmationChannel {
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "push")]
    Push,
}

/// A typed `PaymentMethodType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentMethodType {
    #[serde(rename = "mobile_money")]
    MobileMoney,
    #[serde(rename = "bank_account")]
    BankAccount,
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "motito")]
    Motito,
}

/// A typed `PaymentNextActionType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentNextActionType {
    #[serde(rename = "confirm_payment")]
    ConfirmPayment,
    #[serde(rename = "execute")]
    Execute,
    #[serde(rename = "redirect")]
    Redirect,
    #[serde(rename = "authorize")]
    Authorize,
    #[serde(rename = "none")]
    None,
}

/// A typed `PaymentResultStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentResultStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "requires_confirmation")]
    RequiresConfirmation,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
}

/// A typed `PaymentStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentStatus {
    #[serde(rename = "initiated")]
    Initiated,
    #[serde(rename = "requires_action")]
    RequiresAction,
    #[serde(rename = "overdue")]
    Overdue,
    #[serde(rename = "executed")]
    Executed,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "unknown")]
    Unknown,
}

/// A typed `PayoutStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PayoutStatus {
    #[serde(rename = "initialized")]
    Initialized,
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "executing")]
    Executing,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "canceled")]
    Canceled,
}

/// A typed `ProductShipmentInputType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductShipmentInputType {
    #[serde(rename = "delivery")]
    Delivery,
    #[serde(rename = "download")]
    Download,
    #[serde(rename = "render")]
    Render,
    #[serde(rename = "stream")]
    Stream,
}

/// A typed `ProductShipmentType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductShipmentType {
    #[serde(rename = "delivery")]
    Delivery,
    #[serde(rename = "download")]
    Download,
    #[serde(rename = "render")]
    Render,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "stream")]
    Stream,
}

/// A typed `ProductType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductType {
    #[serde(rename = "physical")]
    Physical,
    #[serde(rename = "digital")]
    Digital,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "voucher")]
    Voucher,
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "cause")]
    Cause,
}

/// A typed `PurchaseIntentActivityType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PurchaseIntentActivityType {
    #[serde(rename = "expired_viewed")]
    ExpiredViewed,
    #[serde(rename = "order_created")]
    OrderCreated,
    #[serde(rename = "payment_failed")]
    PaymentFailed,
    #[serde(rename = "payment_started")]
    PaymentStarted,
    #[serde(rename = "viewed")]
    Viewed,
}

/// A typed `PurchaseIntentStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PurchaseIntentStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "used")]
    Used,
}

/// A typed `RefundReason` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RefundReason {
    #[serde(rename = "requested_by_customer")]
    RequestedByCustomer,
    #[serde(rename = "duplicate")]
    Duplicate,
    #[serde(rename = "fraudulent")]
    Fraudulent,
    #[serde(rename = "order_canceled")]
    OrderCanceled,
    #[serde(rename = "item_returned")]
    ItemReturned,
    #[serde(rename = "item_damaged")]
    ItemDamaged,
    #[serde(rename = "item_not_received")]
    ItemNotReceived,
    #[serde(rename = "item_not_as_described")]
    ItemNotAsDescribed,
    #[serde(rename = "custom")]
    Custom,
}

/// A typed `RefundStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RefundStatus {
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "succeeded")]
    Succeeded,
}

/// A typed `SecretKeyAuthResult` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecretKeyAuthResult {
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
}

/// A typed `SecretKeyStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecretKeyStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "expired")]
    Expired,
}

/// A typed `SecretKeyTokenType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecretKeyTokenType {
    #[serde(rename = "bearer")]
    Bearer,
}

/// A typed `UploadRequestStatus` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UploadRequestStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "uploading")]
    Uploading,
    #[serde(rename = "fulfilled")]
    Fulfilled,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "failed")]
    Failed,
}

/// A typed `UploadReviewDecision` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UploadReviewDecision {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "rejected")]
    Rejected,
}

/// A typed `UploadReviewType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UploadReviewType {
    #[serde(rename = "automatic")]
    Automatic,
    #[serde(rename = "manual")]
    Manual,
}

/// A typed `WalletType` value used by the Inttegro API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WalletType {
    #[serde(rename = "mobile_money")]
    MobileMoney,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChimeInlineRecipientInput {
    ChimeInlineRecipientInputVariant1(ChimeInlineRecipientInputVariant1),
    ChimeInlineRecipientInputVariant2(ChimeInlineRecipientInputVariant2),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChimeRecipientInput {
    ChimeInlineRecipientInputVariant1(ChimeInlineRecipientInputVariant1),
    ChimeInlineRecipientInputVariant2(ChimeInlineRecipientInputVariant2),
    ChimeSavedCustomerRecipientInput(ChimeSavedCustomerRecipientInput),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMessageTemplateRequest {
    CreateSMSMessageTemplateRequest(CreateSMSMessageTemplateRequest),
    CreateEmailMessageTemplateRequest(CreateEmailMessageTemplateRequest),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOrderRequest {
    CreateOrderNewCustomerInput(CreateOrderNewCustomerInput),
    CreateOrderExistingCustomerInput(CreateOrderExistingCustomerInput),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FinancialAccountCreateRequest {
    FinancialAccountWalletRequest(FinancialAccountWalletRequest),
    FinancialAccountBankRequest(FinancialAccountBankRequest),
    FinancialAccountDoshRequest(FinancialAccountDoshRequest),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LineItemInput {
    ProductLineItemInput(ProductLineItemInput),
    FeeLineItemInput(FeeLineItemInput),
    ShippingLineItemInput(ShippingLineItemInput),
}

pub type MessageTemplateAttachmentIDs = Vec<String>;

pub type MessageTemplateAttachmentIDsInput = Vec<String>;

pub type MessageTemplateVariablesInput = JsonData;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderLineItem {
    OrderProductLineItem(OrderProductLineItem),
    OrderFeeLineItem(OrderFeeLineItem),
    OrderShippingLineItem(OrderShippingLineItem),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProductDetailsInput {
    InlineProductDetailsInput(InlineProductDetailsInput),
    CatalogProductWithPriceDataInput(CatalogProductWithPriceDataInput),
    CatalogProductWithPriceReferenceInput(CatalogProductWithPriceReferenceInput),
}

pub type RefundReasonInput = RefundReason;

pub type RefundReasonValue = RefundReason;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReviewUploadRequestAttemptRequest {
    ReviewUploadRequestAttemptByIDRequest(ReviewUploadRequestAttemptByIDRequest),
    ReviewUploadRequestAttemptByOrdinalRequest(ReviewUploadRequestAttemptByOrdinalRequest),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastRequestMessageTemplate {
    StringValue(String),
    MessageTemplateReferenceInput(MessageTemplateReferenceInput),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProductLineItemInputProduct {
    InlineProductDetailsInput(InlineProductDetailsInput),
    CatalogProductWithPriceDataInput(CatalogProductWithPriceDataInput),
    CatalogProductWithPriceReferenceInput(CatalogProductWithPriceReferenceInput),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendChimeRequestRecipient {
    ChimeInlineRecipientInputVariant1(ChimeInlineRecipientInputVariant1),
    ChimeInlineRecipientInputVariant2(ChimeInlineRecipientInputVariant2),
    ChimeSavedCustomerRecipientInput(ChimeSavedCustomerRecipientInput),
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivatePaymentMethodRequest {
    pub payment_method_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddProductPriceRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    pub product_id: String,
    pub amount: AmountParams,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    pub name: String,
    pub phone_number: String,
    pub line1: String,
    pub town: String,
    pub country: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Amount {
    pub currency: Currency,
    pub value: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmountParams {
    pub currency: Currency,
    pub value: i64,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Application {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<ApplicationSecretKey>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<ApplicationRelationship>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationRelationship {
    pub id: String,
    pub kind: AppRelationshipKind,
    pub policy_version: String,
    pub status: AppRelationshipStatus,
    pub actor_app_id: String,
    pub creator_app_id: String,
    pub placement_parent_app_id: String,
    pub subject_app_id: String,
    pub child_app_id: String,
    pub child_standing: String,
    pub relationship_policy: ApplicationRelationshipPolicy,
    pub retained_creator_authority_exists: bool,
    pub created_at: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationRelationshipPolicy {
    pub child_standing: String,
    pub management: AppManagementRole,
    pub credentials: AppCredentialOwner,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationSecretKey {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArchivePaymentMethodRequest {
    pub payment_method_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalanceTransaction {
    pub amount: BalanceTransactionAmount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claimed_at: Option<String>,
    pub created_at: String,
    pub id: String,
    pub order_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payout_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refund_id: Option<String>,
    #[serde(rename = "type")]
    pub r#type: BalanceTransactionType,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalanceTransactionAmount {
    pub currency: String,
    pub value: i64,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalanceTransactionPage {
    pub number: i64,
    pub size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<BalanceTransaction>>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalanceValue {
    pub amount: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BillingDetailsInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressInput>,
    pub name: String,
    pub email_address: String,
    pub phone_number: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BroadcastCancelDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chime_ids: Option<Vec<String>>,
    pub content: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<ChimeEmailMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BroadcastError>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<String>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    pub recipients: Vec<String>,
    pub send_after: String,
    pub sender_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BroadcastCreationDetail {
    pub content: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<ChimeEmailMessage>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    pub recipients: Vec<String>,
    pub send_after: String,
    pub sender_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BroadcastDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chime_ids: Option<Vec<String>>,
    pub content: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<ChimeEmailMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BroadcastError>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<String>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    pub recipients: Vec<String>,
    pub send_after: String,
    pub sender_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BroadcastError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fix_code: Option<String>,
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BroadcastRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_meta: Option<BroadcastRequestRequestMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_template: Option<BroadcastRequestMessageTemplate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<ChimeEmailMessageInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    pub recipients: Vec<serde_json::Value>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BroadcastRequestRequestMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelBroadcastRequest {
    pub broadcast_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execute_refund: Option<bool>,
    pub order_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelPayoutRequest {
    pub payout_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelPurchaseIntentRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purchase_intent_id: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelRefundRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_meta: Option<RefundRequestMetaInput>,
    pub refund_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelScheduleRequest {
    pub schedule_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelUploadRequestRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_by: Option<FileActorInput>,
    pub id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogPrice {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    pub active: bool,
    pub nominal: Amount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<PriceEmbeddedProduct>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogPriceParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    pub amount: AmountParams,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogProductWithPriceDataInput {
    pub price: PriceParams,
    pub product_id: String,
    pub quantity: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogProductWithPriceReferenceInput {
    pub price_id: String,
    pub product_id: String,
    pub quantity: i64,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chime {
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<ChimeEmailMessage>,
    pub full_message: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    pub recipient: ChimeRecipient,
    pub sender_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmission: Option<ChimeTransmission>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeEmailEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounce_sub_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounce_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complaint_sub_type: Option<String>,
    pub id: String,
    pub occurred_at: String,
    pub provider: String,
    pub provider_message_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppress_recipient: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporary: Option<bool>,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeEmailMailbox {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeEmailMailboxInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeEmailMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(rename = "from")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_: Option<ChimeEmailMailbox>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<ChimeEmailMailbox>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<MessageHeaders>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safety: Option<ChimeEmailSafetyResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<ChimeEmailSchemaMarkup>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeEmailMessageInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<MessageHeaders>,
    pub subject: String,
    pub text: String,
    #[serde(rename = "from")]
    pub from_: ChimeEmailMailboxInput,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeEmailSafetyResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ContentSafetyStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_codes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sanitized_html: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalized_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ChimeEmailScannedLink>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scanner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quarantine_notes: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeEmailScannedLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ContentSafetyStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeEmailSchemaMarkup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ChimeEmailSchemaKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json_ld: Option<JsonData>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeInlineRecipientInputVariant1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub phone: ChimeInlineRecipientInputVariant1Phone,
    #[serde(rename = "type")]
    pub r#type: ChimeRecipientType,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeInlineRecipientInputVariant1Phone {
    pub number: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeInlineRecipientInputVariant2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub email: ChimeInlineRecipientInputVariant2Email,
    #[serde(rename = "type")]
    pub r#type: ChimeRecipientType,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeInlineRecipientInputVariant2Email {
    pub address: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimePage {
    pub number: i64,
    pub size: i64,
    pub chimes: Vec<Chime>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeRecipient {
    #[serde(rename = "type")]
    pub r#type: ChimeRecipientType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<ChimeRecipientPhone>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<ChimeRecipientEmail>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeRecipientEmail {
    pub address: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeRecipientPhone {
    pub number: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeSavedCustomerRecipientInput {
    pub customer_id: String,
    pub transport: ChimeTransport,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChimeTransmission {
    pub address: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_events: Option<Vec<ChimeEmailEvent>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_failure_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_failure_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<String>,
    pub gateway: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway_message_id: Option<String>,
    pub id: String,
    pub initialized_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_email_event_at: Option<String>,
    pub mechanism: ChimeTransport,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_via: Option<ChimeTransport>,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppressed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppression_reason: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompleteOrderRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_out_of_band: Option<bool>,
    pub order_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfirmPaymentRequest {
    pub order_id: String,
    pub payment_id: String,
    pub confirmation_id: String,
    pub token: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountryBank {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swift_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_code_prefix: Option<String>,
    pub branches: Vec<CountryBankBranch>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountryBankBranch {
    pub id: String,
    pub name: String,
    pub sort_code: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountryBankDirectory {
    pub bank_account_type: String,
    pub code_scheme: String,
    pub items: Vec<CountryBank>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountrySpecification {
    pub country_code: String,
    pub country_name: String,
    pub currencies: Vec<String>,
    pub payment_methods: Vec<String>,
    pub payout_schedules: Vec<String>,
    pub bt_aging_specs: Vec<String>,
    pub legal_entity_types: Vec<String>,
    pub financial_account_types: Vec<String>,
    pub id_document_types: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banks: Option<CountryBankDirectory>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateApplicationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_entity_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement_parent_application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_policy: Option<CreateApplicationRequestRelationshipPolicy>,
    pub name: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateApplicationRequestRelationshipPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub child_standing: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub management: Option<AppManagementRole>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<AppCredentialOwner>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCustomerRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<CustomerAddressInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CustomerAddressInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub name: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateEmailMessageTemplateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<MessageTemplateVariableInput>>,
    pub channel: MessageTemplateChannel,
    pub email: MessageTemplateEmailContentInput,
    pub name: String,
    pub purpose: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateFileLinkRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery: Option<FileLinkDeliveryInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<FileLinkAccessRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<FileActorInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub file_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateOrderExistingCustomerInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<PaymentMethodDataInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execute_payment: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finalize: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_meta: Option<CreateOrderExistingCustomerInputRequestMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_settings: Option<CreateOrderExistingCustomerInputCheckoutSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<InvoiceSettingsInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payout_settings: Option<OrderPayoutSettingsRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetailsInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingInput>,
    pub customer_id: String,
    pub line_items: Vec<serde_json::Value>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateOrderExistingCustomerInputCheckoutSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateOrderExistingCustomerInputRequestMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateOrderNewCustomerInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execute_payment: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finalize: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_meta: Option<CreateOrderNewCustomerInputRequestMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_settings: Option<CreateOrderNewCustomerInputCheckoutSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<InvoiceSettingsInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payout_settings: Option<OrderPayoutSettingsRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetailsInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<PaymentMethodDataInput>,
    pub customer_data: CustomerDataInput,
    pub line_items: Vec<serde_json::Value>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateOrderNewCustomerInputCheckoutSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateOrderNewCustomerInputRequestMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateProductRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipment: Option<ProductShipmentInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ProductDimensionsInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_dimension: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media: Option<ProductMediaInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<ProductAttributeInput>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(rename = "type")]
    pub r#type: ProductType,
    pub name: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePurchaseIntentRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<CreatePurchaseIntentRequestProduct>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<CreatePurchaseIntentRequestPrice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<CreatePurchaseIntentRequestUsage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub quantity: CreatePurchaseIntentRequestQuantity,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePurchaseIntentRequestPrice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal: Option<PriceParams>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original: Option<CreatePurchaseIntentRequestPriceOriginal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_id: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePurchaseIntentRequestPriceOriginal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal: Option<PriceParams>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePurchaseIntentRequestProduct {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant_set_id: Option<String>,
    pub id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePurchaseIntentRequestQuantity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    pub min: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePurchaseIntentRequestUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_use: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_use: Option<bool>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRefundLineItemInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<RefundReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_details: Option<String>,
    pub order_line_item_id: String,
    pub refund_amount: AmountParams,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRefundRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_details: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_meta: Option<RefundRequestMetaInput>,
    pub line_items: Vec<CreateRefundLineItemInput>,
    pub order_id: String,
    pub reason: RefundReason,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateSMSMessageTemplateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<MessageTemplateVariableInput>>,
    pub channel: MessageTemplateChannel,
    pub name: String,
    pub purpose: String,
    pub sms: MessageTemplateSMSContentInput,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUploadRequestRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<UploadRequestConstraintsInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<UploadRequestDisplayInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<FilePartyInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<FilePartyInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<FileResourceInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requester: Option<FileActorInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempts: Option<UploadRequestAttemptsRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub purpose: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrencyBalanceSnapshot {
    pub available: BalanceValue,
    pub includes_transactions_before: String,
    pub pending: BalanceValue,
    pub refund: CurrencyBalanceSnapshotRefund,
    pub reserved: CurrencyBalanceSnapshotReserved,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrencyBalanceSnapshotRefund {
    pub amount: i64,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrencyBalanceSnapshotReserved {
    pub amount: i64,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Customer {
    pub balance: CustomerBalance,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<CustomerAddress>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    pub guest: bool,
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CustomerAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    pub country: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerAddressInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    pub country: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerBalanceValue {
    pub as_of: String,
    pub available: Amount,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerDataInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataInput>,
    pub name: String,
    pub email_address: String,
    pub phone_number: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerPage {
    pub customers: Vec<Customer>,
    pub number: i64,
    pub size: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteFileRequest {
    pub file_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestroySecretKeyRequest {
    pub secret_key_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableAutomaticPayoutsRequest {}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisactivatePaymentMethodRequest {
    pub payment_method_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableAutomaticPayoutsRequest {}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fix_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    pub code: String,
    pub url: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeDetailsInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataInput>,
    pub amount: AmountParams,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeLineItemInput {
    #[serde(rename = "type")]
    pub r#type: LineItemType,
    pub fee: FeeDetailsInput,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub id: String,
    pub purpose: String,
    pub status: FileStatus,
    pub scan_status: FileScanStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    pub content_type: String,
    pub size: i64,
    pub checksum_sha256: String,
    pub created_by: FileActor,
    pub source: FileSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media: Option<FileMedia>,
    pub storage: PublicFileStorage,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery: Option<FileDeliveryDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_error: Option<FileLatestError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<FileMetadata>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileActor {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileActorInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileContentsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disposition: Option<FileDisposition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery: Option<FileDelivery>,
    pub file_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileDeliveryDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileLatestError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retryable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileLink {
    pub id: String,
    pub kind: FileLinkKind,
    pub file_id: String,
    pub purpose: String,
    pub status: FileLinkStatus,
    pub active: bool,
    pub delivery: FileLinkDelivery,
    pub access: FileLinkAccess,
    pub created_by: FileLinkActor,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked_by: Option<FileLinkActor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<FileMetadata>,
    pub created_at: String,
    pub updated_at: String,
    pub expires_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileLinkAccess {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_accesses: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_accessed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_download: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileLinkAccessRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_accesses: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_download: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_ip_ranges: Option<Vec<String>>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileLinkActor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileLinkCreation {
    pub file_link: FileLink,
    pub url: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileLinkDelivery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<FileLinkDeliveryMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disposition: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileLinkDeliveryInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<FileLinkDeliveryMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disposition: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileLinkPage {
    pub number: i64,
    pub size: i64,
    pub file_links: Vec<FileLink>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileMedia {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_space: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_alpha: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilePage {
    pub number: i64,
    pub size: i64,
    pub files: Vec<File>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileParty {
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilePartyInput {
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileReferenceInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    pub file_id: String,
    pub field: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileReferenceReconcileRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<FileReferenceInput>>,
    pub resource_type: String,
    pub resource_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileReferenceReconciliation {
    pub reconciled: bool,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileResource {
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileResourceInput {
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileSource {
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FileSourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upload_request_id: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileUploadReceipt {
    pub content_type: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub size: i64,
    pub status: FileStatus,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinalizeOrderRequest {
    pub order_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
    pub created_at: String,
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution: Option<FinancialInstitution>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_configuration: Option<FinancialAccountPullConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_configuration: Option<FinancialAccountPushConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplied: Option<ResourceSupply>,
    #[serde(rename = "type")]
    pub r#type: FinancialAccountType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<FinancialAccountVerification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<FinancialAccountBank>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disconnected_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dosh_account: Option<DoshAccount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<FinancialAccountOwner>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<FinancialAccountWallet>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountAddress {
    pub city: String,
    pub country: String,
    pub line_1: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    pub region: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountBank {
    #[serde(rename = "type")]
    pub r#type: BankAccountType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ghana_bank_account: Option<GhanaBankAccount>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountBankRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<FinancialAccountOwnerInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_configuration: Option<FinancialAccountBankRequestPullConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_configuration: Option<FinancialAccountBankRequestPushConfiguration>,
    pub currency: String,
    pub label: String,
    pub reference: String,
    #[serde(rename = "type")]
    pub r#type: FinancialAccountType,
    pub bank_account: FinancialAccountBankRequestBankAccount,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountBankRequestBankAccount {
    #[serde(rename = "type")]
    pub r#type: BankAccountType,
    pub ghana_bank_account: FinancialAccountBankRequestBankAccountGhanaBankAccount,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountBankRequestBankAccountGhanaBankAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swift_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder: Option<FinancialAccountOwnerInput>,
    pub number: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountBankRequestPullConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountBankRequestPushConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountDisableRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unset_as_payout_destination: Option<bool>,
    pub account_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountDoshRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_configuration: Option<FinancialAccountDoshRequestPullConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_configuration: Option<FinancialAccountDoshRequestPushConfiguration>,
    pub currency: String,
    pub label: String,
    pub owner: FinancialAccountOwnerInput,
    pub reference: String,
    #[serde(rename = "type")]
    pub r#type: FinancialAccountType,
    pub dosh_account: DoshAccount,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountDoshRequestPullConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountDoshRequestPushConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountEnablePullRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    pub account_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountIDRequest {
    pub account_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountOwner {
    pub address: FinancialAccountAddress,
    pub name: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountOwnerInput {
    pub name: String,
    pub address: FinancialAccountOwnerInputAddress,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountOwnerInputAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    pub country: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountOwnerUpdateInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<FinancialAccountOwnerUpdateInputAddress>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountOwnerUpdateInputAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountPage {
    pub accounts: Vec<FinancialAccount>,
    pub number: i64,
    pub size: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountPageRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    pub page_number: i64,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountPullConfiguration {
    pub enabled_at: String,
    pub mandate: FinancialAccountPullConfigurationMandate,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountPullConfigurationMandate {
    pub created_at: String,
    pub id: String,
    pub ip_address: String,
    pub user_agent: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountPushConfiguration {
    pub enabled_at: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataPatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<FinancialAccountOwnerUpdateInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    pub account_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountWallet {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: WalletType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_money: Option<FinancialAccountWalletMobileMoney>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountWalletMobileMoney {
    pub account_number: String,
    pub network: MobileMoneyNetwork,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountWalletRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_configuration: Option<FinancialAccountWalletRequestPullConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_configuration: Option<FinancialAccountWalletRequestPushConfiguration>,
    pub currency: String,
    pub label: String,
    pub owner: FinancialAccountOwnerInput,
    pub reference: String,
    #[serde(rename = "type")]
    pub r#type: FinancialAccountType,
    pub wallet: FinancialAccountWalletRequestWallet,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountWalletRequestPullConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountWalletRequestPushConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountWalletRequestWallet {
    #[serde(rename = "type")]
    pub r#type: WalletType,
    pub mobile_money: FinancialAccountWalletRequestWalletMobileMoney,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccountWalletRequestWalletMobileMoney {
    pub account_number: String,
    pub network: MobileMoneyNetwork,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialInstitution {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank: Option<FinancialInstitutionBank>,
    pub country: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_money_provider: Option<FinancialInstitutionMobileMoneyProvider>,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialInstitutionBank {
    pub bank_account_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<FinancialInstitutionBankBranch>,
    pub code_scheme: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_code_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swift_code: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialInstitutionBankBranch {
    pub id: String,
    pub name: String,
    pub sort_code: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialInstitutionMobileMoneyProvider {
    pub provider: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateSecretKeyRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneratedSecretKey {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub token_type: SecretKeyTokenType,
    pub issued_at: String,
    pub token: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPaymentMethodSettingsRequest {}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPayoutSettingsRequest {}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GhanaBankAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    pub holder: FinancialAccountOwner,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub number: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swift_code: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitiateOTPRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub async_delivery: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_template: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_alphabet: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_alphabet_type: Option<OTPAlphabetType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validity_duration_in_minutes: Option<i64>,
    pub recipient: String,
    pub service_name: String,
    pub token_size: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineProductDetailsInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    pub name: String,
    pub price: PriceParams,
    pub quantity: i64,
    #[serde(rename = "type")]
    pub r#type: ProductType,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoiceSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoiceSettingsInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListCountrySpecsRequest {}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupBalanceTransactionRequest {
    pub transaction_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupBalancesRequest {}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupBroadcastRequest {
    pub broadcast_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupChimeRequest {
    pub chime_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupCustomerRequest {
    pub customer_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupFileLinkRequest {
    pub id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupFileRequest {
    pub file_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupOTPRequest {
    pub transaction_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupOrderRequest {
    pub order_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupPaymentMethodRequest {
    pub payment_method_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupPayoutRequest {
    pub payout_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupPriceRequest {
    pub price_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupProductRequest {
    pub product_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupPurchaseIntentRequest {
    pub id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupRefundRequest {
    pub refund_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupScheduleRequest {
    pub schedule_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupSecretKeyRequest {
    pub secret_key_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupUploadRequestRequest {
    pub id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplate {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    pub channel: MessageTemplateChannel,
    pub purpose: String,
    pub locale: String,
    pub status: MessageTemplateStatus,
    pub version: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_version: Option<i64>,
    pub draft_version: i64,
    pub has_unpublished_changes: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<MessageTemplateVariable>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sms: Option<MessageTemplateSMSContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<MessageTemplateEmailContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateEmailContent {
    pub subject: String,
    pub html: String,
    #[serde(rename = "from")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_: Option<MessageTemplateMailbox>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<MessageTemplateMailbox>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<MessageHeaders>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateEmailContentInput {
    #[serde(rename = "from")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_: Option<MessageTemplateMailboxInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<MessageTemplateMailboxInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<MessageHeaders>,
    pub subject: String,
    pub html: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateIDRequest {
    pub id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateMailbox {
    pub address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateMailboxInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub address: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplatePreview {
    pub message_template: MessageTemplate,
    pub rendered: RenderedMessageTemplate,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateReferenceInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<JsonData>,
    pub template_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateSMSContent {
    pub message_template: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateSMSContentInput {
    pub message_template: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateSafetyResult {
    pub content_hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<MessageTemplateScannedLink>>,
    pub normalized_text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quarantine_notes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_codes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sanitized_html: Option<String>,
    pub scanner: String,
    pub status: ContentSafetyStatus,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateScannedLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    pub raw: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub scheme: String,
    pub status: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateVariable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<MessageTemplateVariableItem>>,
    pub name: String,
    pub required: bool,
    #[serde(rename = "type")]
    pub r#type: MessageTemplateVariableType,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateVariableInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<MessageTemplateVariableItemInput>>,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: MessageTemplateVariableType,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateVariableItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    pub name: String,
    pub required: bool,
    #[serde(rename = "type")]
    pub r#type: MessageTemplateVariableItemType,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateVariableItemInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: MessageTemplateVariableItemType,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplatesPage {
    pub number: i64,
    pub size: i64,
    pub message_templates: Vec<MessageTemplate>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OTPTransaction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    pub expires_at: String,
    pub full_message: String,
    pub id: String,
    pub initiated_at: String,
    pub status: OTPStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmission: Option<OTPTransmission>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OTPTransmission {
    pub recipient: String,
    pub sender_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_via: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OTPTransmissionStatus>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OTPVerification {
    pub transaction: OTPTransaction,
    pub verification_attempt: OTPVerificationAttempt,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OTPVerificationAttempt {
    pub attempted_at: String,
    pub id: String,
    pub presented_token: String,
    pub recipient: String,
    pub result: OTPVerificationAttemptResult,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OTPVerificationAttemptResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    pub verdict: OTPVerificationVerdict,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_settings: Option<OrderCheckoutSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_from: Option<OrderCreatedFrom>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub customer: OrderCustomer,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub id: String,
    pub initiated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<OrderInvoice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refunds: Option<Vec<Refund>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<InvoiceSettings>,
    pub status: OrderStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sealed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_item_group: Option<OrderLineItemGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment: Option<Payment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_due_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payout_settings: Option<OrderPayoutSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    pub country: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderCheckoutSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderCreatedFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<OrderCreatedFromResourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderCustomer {
    pub id: String,
    pub guest: bool,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<OrderAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<OrderAddress>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderDocumentDelivery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deliveries: Option<Vec<OrderDocumentDeliveryAttempt>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_kind: Option<OrderDocumentKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_channels: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<OrderDocumentDeliveryFailure>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_channels: Option<Vec<String>>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderDocumentDeliveryAttempt {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<DeliveryChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chime_id: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderDocumentDeliveryFailure {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<DeliveryChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderDocumentDeliveryRequest {
    pub order_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderDocumentDeliveryResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery: Option<OrderDocumentDelivery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderDocumentFormat {
    pub url: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderFeeLineItem {
    #[serde(rename = "type")]
    pub r#type: String,
    pub fee: OrderFeeLineItemFee,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderFeeLineItemFee {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    pub amount: Amount,
    pub label: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderInvoice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<OrderInvoiceFormat>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderInvoiceFormat {
    pub web: OrderDocumentFormat,
    pub pdf: OrderDocumentFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt: Option<OrderDocumentFormat>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderLineItemGroup {
    pub line_items: Vec<serde_json::Value>,
    pub total: Amount,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderPage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<Order>>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderPayoutSettingsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<OrderPayoutSettingsRequestDestination>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_fx: Option<bool>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderPayoutSettingsRequestDestination {
    pub financial_account_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderProductLineItem {
    #[serde(rename = "type")]
    pub r#type: String,
    pub product: OrderProductLineItemProduct,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderProductLineItemProduct {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub price: Price,
    pub quantity: i64,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderShippingLineItem {
    #[serde(rename = "type")]
    pub r#type: String,
    pub shipping: OrderShippingLineItemShipping,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderShippingLineItemShipping {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub fee: Amount,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageBalanceTransactionsRequest {
    pub page_number: i64,
    pub page_size: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageChimesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageCustomersRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    pub page_number: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageFileLinksRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<FileLinkStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageFilesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<FileStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageMessageTemplatesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<MessageTemplateStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<MessageTemplateChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageOrdersRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    pub page_size: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PagePayoutsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    pub page_number: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageProductsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    pub page_number: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PagePurchaseIntentsRequest {
    pub page_number: i64,
    pub page_size: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageRefundsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    pub page_number: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageSecretKeysRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageUploadRequestsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<UploadRequestStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<FileResourceInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayOrderRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<PaymentMethodDataInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_out_of_band: Option<bool>,
    pub order_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payment {
    pub id: String,
    pub status: PaymentStatus,
    pub statement_descriptor: String,
    pub amount: Amount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<BalanceTransaction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethodSnapshot>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_attempt: Option<PaymentAttempt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_action: Option<PaymentNextAction>,
    pub initiated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub due_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_offline: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payout_configuration: Option<PaymentPayoutConfiguration>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentAttempt {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentAttemptStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded_at: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<PaymentMethodBankAccount>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub customer_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<String>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_money: Option<PaymentMethodMobileMoney>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<PaymentMethodOwner>,
    #[serde(rename = "type")]
    pub r#type: PaymentMethodType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplied: Option<PaymentMethodSupplied>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<PaymentMethodVerification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodBankAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ghana_bank_account: Option<PaymentMethodBankAccountGhanaBankAccount>,
    #[serde(rename = "type")]
    pub r#type: BankAccountType,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodBankAccountGhanaBankAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub account_number: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swift_code: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodDataInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_money: Option<PaymentMethodDataInputMobileMoney>,
    #[serde(rename = "type")]
    pub r#type: PaymentMethodType,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodDataInputMobileMoney {
    pub network: MobileMoneyNetwork,
    pub account_number: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodDeletion {
    pub deleted: bool,
    pub payment_method_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodMobileMoney {
    pub account_number: String,
    pub last4: String,
    pub network: MobileMoneyNetwork,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodOwner {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<PaymentMethodOwnerAddress>,
    pub name: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodOwnerAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    pub country: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodOwnerInput {
    pub address: PaymentMethodOwnerInputAddress,
    pub name: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodOwnerInputAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    pub country: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodPage {
    pub number: i64,
    pub payment_methods: Vec<PaymentMethod>,
    pub size: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodPageRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_money: Option<PaymentMethodTypeSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<PaymentMethodTypeSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodTypeSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motito: Option<PaymentMethodTypeSetting>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodSnapshot {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<PaymentMethodSnapshotBankAccount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<JsonData>,
    pub created_at: String,
    pub customer_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_money: Option<PaymentMethodSnapshotMobileMoney>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<PaymentMethodSnapshotOwner>,
    #[serde(rename = "type")]
    pub r#type: PaymentMethodType,
    pub verified: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodSnapshotBankAccount {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ghana_bank_account: Option<PaymentMethodSnapshotGhanaBankAccount>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodSnapshotGhanaBankAccount {
    pub account_number: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swift_code: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodSnapshotMobileMoney {
    pub network: MobileMoneyNetwork,
    pub account_number: String,
    pub last4: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodSnapshotOwner {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<OrderAddress>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodSupplied {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt_id: Option<String>,
    pub by: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    pub supplied_at: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodTypeSetting {
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PaymentMethodType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub enabled: bool,
    pub confirms_use: bool,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodVerification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    pub initiated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mechanism: Option<String>,
    pub request_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodVerificationSession {
    pub payment_method_id: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_sent_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery: Option<JsonData>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentNextAction {
    #[serde(rename = "type")]
    pub r#type: PaymentNextActionType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm_payment: Option<PaymentNextActionConfirmPayment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execute: Option<JsonData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect: Option<PaymentNextActionRedirect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorize: Option<PaymentNextActionAuthorize>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentNextActionAuthorize {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentNextActionConfirmPayment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<PaymentNextActionConfirmPaymentRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<PaymentNextActionConfirmPaymentAttempt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentNextActionConfirmPaymentAttempt {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentNextActionConfirmPaymentRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_via: Option<PaymentConfirmationChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentNextActionRedirect {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_visit: Option<PaymentNextActionRedirectLatestVisit>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentNextActionRedirectLatestVisit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentPayoutConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_fx: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<PaymentPayoutConfigurationDestination>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentPayoutConfigurationDestination {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub financial_account_id: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payout {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_transactions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub destination_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PayoutError>,
    pub execute_after: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<String>,
    pub id: String,
    pub initiated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiated_by: Option<String>,
    pub max_amount: Amount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduled_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    pub status: PayoutStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded_at: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutError {
    pub cause: String,
    pub message: String,
    pub occurred_at: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutPage {
    pub number: i64,
    pub size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payouts: Option<Vec<Payout>>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutSettingsLookup {
    pub destinations: PayoutDestinations,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fx_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<PayoutSettingsLookupSchedule>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutSettingsLookupSchedule {
    pub aging_spec: PayoutSettingsLookupScheduleAgingSpec,
    pub description: String,
    pub interval: String,
    pub name: String,
    pub schedule_on: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutSettingsLookupScheduleAgingSpec {
    pub abide: String,
    pub label: String,
    pub t_plus: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutSettingsMutation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destinations: Option<PayoutDestinations>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<PayoutSettingsMutationSchedule>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutSettingsMutationSchedule {
    pub description: String,
    pub id: String,
    pub interval: String,
    pub name: String,
    pub schedule_on: String,
    pub spec: PayoutSettingsMutationScheduleSpec,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutSettingsMutationScheduleSpec {
    pub abide: String,
    pub id: String,
    pub label: String,
    pub t_plus: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub currency: Currency,
    pub value: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceActionRequest {
    pub price_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceEmbeddedProduct {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<PriceEmbeddedProductAttributesItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ProductDimensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media: Option<ProductMedia>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipment: Option<ProductShipment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    #[serde(rename = "type")]
    pub r#type: ProductType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_dim: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceEmbeddedProductAttributesItem {
    pub name: String,
    pub value: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PricePage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<CatalogPrice>>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PricePageItem {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    pub active: bool,
    pub nominal: Amount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<PriceEmbeddedProduct>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PricePageRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceParams {
    pub currency: Currency,
    pub value: i64,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: ProductType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<ProductPriceSummary>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipment: Option<ProductShipment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media: Option<ProductMedia>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<ProductAttribute>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ProductDimensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub active: bool,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_dim: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductActionRequest {
    pub product_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductAttribute {
    pub name: String,
    pub value: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductAttributeInput {
    pub name: String,
    pub value: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductDimensions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical: Option<ProductDimensionsPhysical>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digital: Option<ProductDimensionsDigital>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<ProductDimensionsCustom>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductDimensionsCustom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<ProductDimensionDetails>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductDimensionsDigital {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductDimensionsInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical: Option<ProductDimensionsInputPhysical>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digital: Option<ProductDimensionsInputDigital>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<ProductDimensionsInputCustom>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductDimensionsInputCustom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<ProductDimensionDetails>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductDimensionsInputDigital {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductDimensionsInputPhysical {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductDimensionsPhysical {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductLineItemInput {
    #[serde(rename = "type")]
    pub r#type: LineItemType,
    pub product: ProductLineItemInputProduct,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductMedia {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hero_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_page_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brand_logo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infographic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub promo_video: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub demo_video: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gallery: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downloads: Option<Vec<String>>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductMediaInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hero_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_page_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brand_logo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infographic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub promo_video: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub demo_video: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gallery: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downloads: Option<Vec<String>>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductPage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<Product>>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductPriceSummary {
    pub id: String,
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub nominal: Amount,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductShipment {
    #[serde(rename = "type")]
    pub r#type: ProductShipmentType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery: Option<JsonData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download: Option<JsonData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub render: Option<JsonData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<JsonData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<JsonData>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductShipmentInput {
    #[serde(rename = "type")]
    pub r#type: ProductShipmentInputType,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicFileStorage {
    pub encoding: FileStorageEncoding,
    pub stored_size: i64,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity: Option<PurchaseIntentActivity>,
    pub allow_variants: bool,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inactive_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant: Option<PurchaseIntentMerchant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<PurchaseIntentPrice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<PurchaseIntentProduct>,
    pub quantity: PurchaseIntentQuantity,
    pub status: PurchaseIntentStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    pub usage: PurchaseIntentUsage,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant_set: Option<PurchaseIntentVariantSet>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentActivity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recent: Option<Vec<PurchaseIntentActivity>>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentMerchant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentOriginalPrice {
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub nominal: Amount,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentPage {
    pub number: i64,
    pub purchase_intents: Vec<PurchaseIntent>,
    pub size: i64,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentPrice {
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub nominal: Amount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original: Option<PurchaseIntentOriginalPrice>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentProduct {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<PurchaseIntentProductAttributesItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ProductDimensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media: Option<ProductMedia>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipment: Option<ProductShipment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    #[serde(rename = "type")]
    pub r#type: ProductType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_dim: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<ProductPriceSummary>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant_set_id: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentProductAttributesItem {
    pub name: String,
    pub value: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentQuantity {
    pub min: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_use: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<PurchaseIntentUsageOrder>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_use: Option<bool>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentUsageOrder {
    pub created_at: String,
    pub id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentVariant {
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<PurchaseIntentPrice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<PurchaseIntentProduct>,
    pub product_id: String,
    pub variant_values: VariantValues,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentVariantAxis {
    pub key: String,
    pub label: String,
    pub position: i64,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseIntentVariantSet {
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_product_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    pub variant_axes: Vec<PurchaseIntentVariantAxis>,
    pub variants: Vec<PurchaseIntentVariant>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Refund {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<String>,
    pub id: String,
    pub line_items: Vec<RefundLineItem>,
    pub order_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processing_at: Option<String>,
    pub reason: RefundReason,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_details: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    pub status: RefundStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded_at: Option<String>,
    pub total: Amount,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefundLineItem {
    pub id: String,
    pub order_line_item_id: String,
    pub original_amount_paid: Amount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<RefundReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_details: Option<String>,
    pub refund_amount: Amount,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefundPage {
    pub number: i64,
    pub refunds: Vec<Refund>,
    pub size: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefundRequestMetaInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderMessageTemplatePreviewRequest {
    pub message_template: MessageTemplateReferenceInput,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderedEmailMessageTemplate {
    pub subject: String,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(rename = "from")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_: Option<MessageTemplateMailbox>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<MessageTemplateMailbox>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<MessageHeaders>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safety: Option<MessageTemplateSafetyResult>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderedMessageTemplate {
    pub channel: MessageTemplateChannel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sms: Option<RenderedSMSMessageTemplate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<RenderedEmailMessageTemplate>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderedSMSMessageTemplate {
    pub full_message: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestConfirmationRequest {
    pub order_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceSupply {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt_id: Option<String>,
    pub by: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    pub supplied_at: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewUploadRequestAttemptByIDRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<UploadRequestReviewReasonInput>>,
    pub attempt_id: String,
    pub decision: UploadReviewDecision,
    pub id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewUploadRequestAttemptByOrdinalRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<UploadRequestReviewReasonInput>>,
    pub attempt_ordinal: i64,
    pub decision: UploadReviewDecision,
    pub id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevokeFileLinkRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked_by: Option<FileActorInput>,
    pub id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleCancelDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chime_ids: Option<Vec<String>>,
    pub content: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<ChimeEmailMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ScheduleError>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<String>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    pub recipients: Vec<String>,
    pub send_after: String,
    pub sender_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleChimeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_meta: Option<ScheduleChimeRequestRequestMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<ChimeEmailMessageInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_template: Option<MessageTemplateReferenceInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    pub recipients: Vec<serde_json::Value>,
    pub send_after: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleChimeRequestRequestMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleCreationDetail {
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<ChimeEmailMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<String>,
    pub full_message: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
    pub send_after: String,
    pub sender_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chime_ids: Option<Vec<String>>,
    pub content: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<ChimeEmailMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ScheduleError>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<String>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    pub recipients: Vec<String>,
    pub send_after: String,
    pub sender_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fix_code: Option<String>,
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchedulePayoutRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execute_after: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<i64>,
    pub destination_id: String,
    pub reference: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretKey {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub token_type: SecretKeyTokenType,
    pub issued_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub status: SecretKeyStatus,
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_count: Option<i64>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretKeyPage {
    pub number: i64,
    pub size: i64,
    pub count: i64,
    pub total: i64,
    pub has_more: bool,
    pub keys: Vec<SecretKey>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretKeyUsage {
    pub key: SecretKey,
    pub usage: SecretKeyUsagePage,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretKeyUsagePage {
    pub number: i64,
    pub size: i64,
    pub count: i64,
    pub total: i64,
    pub has_more: bool,
    pub rows: Vec<SecretKeyUsageRow>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretKeyUsageRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    pub secret_key_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretKeyUsageRow {
    pub secret_key_id: String,
    pub occurred_at: String,
    pub auth_result: SecretKeyAuthResult,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendChimeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<ChimeEmailMessageInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_template: Option<MessageTemplateReferenceInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_meta: Option<SendChimeRequestRequestMeta>,
    pub recipient: SendChimeRequestRecipient,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendChimeRequestRequestMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPayoutDestinationsRequest {
    pub destinations: PayoutDestinations,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShippingDetailsInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataInput>,
    pub fee: AmountParams,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShippingInput {
    pub address: AddressInput,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShippingLineItemInput {
    #[serde(rename = "type")]
    pub r#type: LineItemType,
    pub shipping: ShippingDetailsInput,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenizeMobileMoneyPaymentMethodRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub customer_id: String,
    #[serde(rename = "type")]
    pub r#type: PaymentMethodType,
    pub mobile_money: TokenizeMobileMoneyPaymentMethodRequestMobileMoney,
    pub owner: PaymentMethodOwnerInput,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenizeMobileMoneyPaymentMethodRequestMobileMoney {
    pub account_number: String,
    pub network: MobileMoneyNetwork,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnarchivePaymentMethodRequest {
    pub payment_method_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateApplicationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_entity_type: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateCustomerRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<CustomerAddressInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataPatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CustomerAddressInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub customer_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateMessageTemplateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<MessageTemplateChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<MessageTemplateVariableInput>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sms: Option<MessageTemplateSMSContentInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<MessageTemplateEmailContentInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
    pub id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateOrderRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clear_payment_method: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<InvoiceSettingsInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finalize: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<UpdateOrderRequestPaymentMethodData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<String>,
    pub order_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateOrderRequestPaymentMethodData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_money: Option<UpdateOrderRequestPaymentMethodDataMobileMoney>,
    #[serde(rename = "type")]
    pub r#type: PaymentMethodType,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateOrderRequestPaymentMethodDataMobileMoney {
    pub network: MobileMoneyNetwork,
    pub account_number: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePaymentMethodRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataPatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<UpdatePaymentMethodRequestOwner>,
    pub payment_method_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePaymentMethodRequestOwner {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdatePaymentMethodRequestOwnerAddress>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePaymentMethodRequestOwnerAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePriceRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    pub price_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ProductType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipment: Option<ProductShipmentInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ProductDimensionsInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_dimension: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media: Option<ProductMediaInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<ProductAttributeInput>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub product_id: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePurchaseIntentRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<UpdatePurchaseIntentRequestQuantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purchase_intent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactivate: Option<bool>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePurchaseIntentRequestQuantity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    pub min: i64,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateSecretKeyRequest {
    pub label: String,
    pub secret_key_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatedProduct {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(rename = "type")]
    pub r#type: ProductType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ProductDimensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<ProductPriceSummary>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_dim: Option<String>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadFulfillment {
    pub upload_request: UploadRequest,
    pub file: FileUploadReceipt,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequest {
    pub id: String,
    pub purpose: String,
    pub status: UploadRequestStatus,
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upload_url: Option<String>,
    pub constraints: UploadRequestConstraints,
    pub display: UploadRequestDisplay,
    pub subject: FileParty,
    pub recipient: FileParty,
    pub resource: FileResource,
    pub requester: UploadRequestActor,
    pub attempts: UploadRequestAttempts,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_error: Option<UploadRequestLatestError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_by: Option<UploadRequestActor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<FileMetadata>,
    pub created_at: String,
    pub updated_at: String,
    pub expires_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploading_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfilled_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<UploadRequestAttempt>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequestActor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequestAttempt {
    pub attempted_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declared_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<UploadRequestLatestError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    pub id: String,
    pub ordinal: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<UploadRequestReview>,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded_at: Option<String>,
    pub upload_request_id: String,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequestAttempts {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<i64>,
    pub attempt_count: i64,
    pub failed_attempt_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_attempted_at: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequestAttemptsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<i64>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequestConstraints {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_types: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequestConstraintsInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_types: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequestDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_text: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequestDisplayInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_text: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequestLatestError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retryable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequestPage {
    pub number: i64,
    pub size: i64,
    pub upload_requests: Vec<UploadRequest>,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequestReview {
    pub created_at: String,
    pub decision: UploadReviewDecision,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<UploadRequestReviewReason>>,
    pub reviewed_at: String,
    #[serde(rename = "type")]
    pub r#type: UploadReviewType,
}

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequestReviewReason {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadRequestReviewReasonInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    pub code: String,
    pub message: String,
}

/// Typed Inttegro request parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerifyOTPRequest {
    pub transaction_id: String,
    pub recipient: String,
    pub token: String,
}
