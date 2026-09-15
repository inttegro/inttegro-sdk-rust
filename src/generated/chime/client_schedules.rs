//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro schedules.
#[derive(Clone)]
pub struct Schedules {
    client: Client,
}

impl Schedules {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Look up a scheduled Chime
    pub async fn lookup(&self, request: &LookupScheduleRequest) -> Result<ScheduleDetail> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Look up a scheduled Chime with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupScheduleRequest,
        options: RequestOptions,
    ) -> Result<ScheduleDetail> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/schedules/lookup",
                    operation: "schedules.lookup",
                    field: Some("scheduled_chime"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Cancel a scheduled Chime
    pub async fn cancel(&self, request: &CancelScheduleRequest) -> Result<ScheduleCancelDetail> {
        self.cancel_with_options(request, RequestOptions::default())
            .await
    }
    /// Cancel a scheduled Chime with per-request options.
    pub async fn cancel_with_options(
        &self,
        request: &CancelScheduleRequest,
        options: RequestOptions,
    ) -> Result<ScheduleCancelDetail> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/schedules/cancel",
                    operation: "schedules.cancel",
                    field: Some("scheduled_chime"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}
