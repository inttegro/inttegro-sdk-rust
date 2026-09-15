//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro otp.
#[derive(Clone)]
pub struct Otp {
    client: Client,
}

impl Otp {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Initiate OTP transaction
    pub async fn initiate(&self, request: &InitiateOTPRequest) -> Result<OTPTransaction> {
        self.initiate_with_options(request, RequestOptions::default())
            .await
    }
    /// Initiate OTP transaction with per-request options.
    pub async fn initiate_with_options(
        &self,
        request: &InitiateOTPRequest,
        options: RequestOptions,
    ) -> Result<OTPTransaction> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/otp/initiate",
                    operation: "otp.initiate",
                    field: Some("transaction"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Verify OTP token
    pub async fn verify(&self, request: &VerifyOTPRequest) -> Result<OTPVerification> {
        self.verify_with_options(request, RequestOptions::default())
            .await
    }
    /// Verify OTP token with per-request options.
    pub async fn verify_with_options(
        &self,
        request: &VerifyOTPRequest,
        options: RequestOptions,
    ) -> Result<OTPVerification> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/otp/verify",
                    operation: "otp.verify",
                    field: None,
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Lookup OTP transaction
    pub async fn lookup(&self, request: &LookupOTPRequest) -> Result<OTPTransaction> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Lookup OTP transaction with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupOTPRequest,
        options: RequestOptions,
    ) -> Result<OTPTransaction> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/otp/lookup",
                    operation: "otp.lookup",
                    field: Some("transaction"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}
