//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro chimes.
#[derive(Clone)]
pub struct Chimes {
    client: Client,
}

impl Chimes {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Send a Chime
    pub async fn send(&self, request: &SendChimeRequest) -> Result<Chime> {
        self.send_with_options(request, RequestOptions::default())
            .await
    }
    /// Send a Chime with per-request options.
    pub async fn send_with_options(
        &self,
        request: &SendChimeRequest,
        options: RequestOptions,
    ) -> Result<Chime> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/chimes/send",
                    operation: "chimes.send",
                    field: Some("chime"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Look up a Chime
    pub async fn lookup(&self, request: &LookupChimeRequest) -> Result<Chime> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Look up a Chime with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupChimeRequest,
        options: RequestOptions,
    ) -> Result<Chime> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/chimes/lookup",
                    operation: "chimes.lookup",
                    field: Some("chime"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Page through Chimes
    pub async fn page(&self, request: &PageChimesRequest) -> Result<ChimePage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page through Chimes with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PageChimesRequest,
        options: RequestOptions,
    ) -> Result<ChimePage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/chimes/page",
                    operation: "chimes.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Schedule Chimes
    pub async fn schedule(&self, request: &ScheduleChimeRequest) -> Result<ScheduleCreationDetail> {
        self.schedule_with_options(request, RequestOptions::default())
            .await
    }
    /// Schedule Chimes with per-request options.
    pub async fn schedule_with_options(
        &self,
        request: &ScheduleChimeRequest,
        options: RequestOptions,
    ) -> Result<ScheduleCreationDetail> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/chimes/schedule",
                    operation: "chimes.schedule",
                    field: Some("scheduled_chime"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Broadcast Chimes
    pub async fn broadcast(&self, request: &BroadcastRequest) -> Result<BroadcastCreationDetail> {
        self.broadcast_with_options(request, RequestOptions::default())
            .await
    }
    /// Broadcast Chimes with per-request options.
    pub async fn broadcast_with_options(
        &self,
        request: &BroadcastRequest,
        options: RequestOptions,
    ) -> Result<BroadcastCreationDetail> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/chimes/broadcast",
                    operation: "chimes.broadcast",
                    field: Some("broadcast"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}
