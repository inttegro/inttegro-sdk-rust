//! Official server-side Rust SDK for Inttegro.

mod generated;
mod resources;
mod semantic_collections;

pub use generated::*;
pub use resources::*;
pub use semantic_collections::*;

use bytes::Bytes;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, USER_AGENT};
use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::Arc;
use std::time::Instant;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_BASE_URL: &str = "https://api.inttegro.com";

pub type Result<T> = std::result::Result<T, InttegroError>;

#[derive(Debug, thiserror::Error)]
pub enum InttegroError {
    #[error("Inttegro API request failed with status {status} ({code})")]
    Api {
        status: u16,
        code: String,
        error_type: Option<String>,
        fix_code: Option<String>,
        request_id: Option<String>,
        report: Option<Box<ErrorReport>>,
    },
    #[error("Inttegro transport failed: {0}")]
    Transport(#[from] reqwest::Error),
    #[error("Inttegro response could not be decoded: {0}")]
    Decode(#[from] serde_json::Error),
    #[error("invalid Inttegro SDK configuration: {0}")]
    Configuration(String),
}

#[derive(Debug, Clone, Default)]
pub struct RequestOptions {
    pub idempotency_key: Option<String>,
    pub headers: HeaderMap,
}

#[derive(Debug, Clone)]
pub struct FileDownload {
    pub bytes: Bytes,
    pub content_type: Option<String>,
    pub filename: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateFileRequest {
    pub file_name: String,
    pub bytes: Bytes,
    pub purpose: String,
    pub title: Option<String>,
    pub custom_data: Option<CustomData>,
}

#[derive(Debug, Clone)]
pub struct FulfillUploadRequest {
    pub id: String,
    pub token: String,
    pub file_name: String,
    pub bytes: Bytes,
}

#[derive(Debug, Clone)]
pub struct OpenFileLinkRequest {
    pub id: String,
    pub token: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorReportingPolicy {
    Unexpected,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SDKReportContext {
    pub language: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HTTPReportContext {
    pub method: String,
    pub route: Option<String>,
    pub server_address: String,
    pub status_code: Option<u16>,
    pub request_id: Option<String>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct APIErrorReportContext {
    pub r#type: Option<String>,
    pub code: Option<String>,
    pub fix_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TraceReportContext {
    pub trace_id: String,
    pub span_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorReport {
    pub schema_version: u8,
    pub event_id: String,
    pub occurred_at: String,
    pub severity: String,
    pub category: String,
    pub operation: String,
    pub sdk: SDKReportContext,
    pub http: HTTPReportContext,
    pub api_error: Option<APIErrorReportContext>,
    pub trace: Option<TraceReportContext>,
    pub exception_type: String,
    pub fingerprint: String,
}

pub trait ErrorReporter: Send + Sync {
    fn report(&self, report: ErrorReport);
}

#[derive(Debug, Clone)]
pub struct TelemetryEvent<'a> {
    pub name: &'a str,
    pub operation: &'a str,
    pub method: &'a str,
    pub route: &'a str,
    pub status_code: Option<u16>,
    pub duration_ms: u64,
}

pub trait Telemetry: Send + Sync {
    fn record(&self, event: TelemetryEvent<'_>);
}

#[derive(Clone)]
pub struct Client {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    api_key: String,
    base_url: String,
    http: reqwest::Client,
    telemetry: Option<Arc<dyn Telemetry>>,
    error_reporter: Option<Arc<dyn ErrorReporter>>,
    error_reporting_policy: ErrorReportingPolicy,
}

#[derive(Clone, Copy)]
struct RequestSpec<'a> {
    method: &'a str,
    path: &'a str,
    operation: &'a str,
    field: Option<&'a str>,
    authenticated: bool,
}

struct FailureContext<'a> {
    operation: &'a str,
    method: &'a str,
    route: &'a str,
    started: Instant,
    status: Option<u16>,
    request_id: Option<String>,
    code: Option<String>,
    category: &'a str,
    fix_code: Option<String>,
}

struct JsonResponse {
    value: Value,
    status: u16,
    request_id: Option<String>,
    started: Instant,
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::builder(api_key).build()
    }

    pub fn builder(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new(api_key)
    }

    pub fn apps(&self) -> Apps {
        Apps::new(self.clone())
    }
    pub fn balance_transactions(&self) -> BalanceTransactions {
        BalanceTransactions::new(self.clone())
    }
    pub fn balances(&self) -> Balances {
        Balances::new(self.clone())
    }
    pub fn broadcasts(&self) -> Broadcasts {
        Broadcasts::new(self.clone())
    }
    pub fn chimes(&self) -> Chimes {
        Chimes::new(self.clone())
    }
    pub fn customers(&self) -> Customers {
        Customers::new(self.clone())
    }
    pub fn file_links(&self) -> FileLinks {
        FileLinks::new(self.clone())
    }
    pub fn file_references(&self) -> FileReferences {
        FileReferences::new(self.clone())
    }
    pub fn files(&self) -> Files {
        Files::new(self.clone())
    }
    pub fn financial_accounts(&self) -> FinancialAccounts {
        FinancialAccounts::new(self.clone())
    }
    pub fn keys(&self) -> Keys {
        Keys::new(self.clone())
    }
    pub fn message_templates(&self) -> MessageTemplates {
        MessageTemplates::new(self.clone())
    }
    pub fn orders(&self) -> Orders {
        Orders::new(self.clone())
    }
    pub fn otp(&self) -> Otp {
        Otp::new(self.clone())
    }
    pub fn payment_methods(&self) -> PaymentMethods {
        PaymentMethods::new(self.clone())
    }
    pub fn payouts(&self) -> Payouts {
        Payouts::new(self.clone())
    }
    pub fn prices(&self) -> Prices {
        Prices::new(self.clone())
    }
    pub fn products(&self) -> Products {
        Products::new(self.clone())
    }
    pub fn purchase_intents(&self) -> PurchaseIntents {
        PurchaseIntents::new(self.clone())
    }
    pub fn refunds(&self) -> Refunds {
        Refunds::new(self.clone())
    }
    pub fn schedules(&self) -> Schedules {
        Schedules::new(self.clone())
    }
    pub fn specifications(&self) -> Specifications {
        Specifications::new(self.clone())
    }
    pub fn upload_requests(&self) -> UploadRequests {
        UploadRequests::new(self.clone())
    }

    async fn request_resource<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        spec: RequestSpec<'_>,
        body: Option<&B>,
        options: RequestOptions,
    ) -> Result<T> {
        let response = self.request_json(spec, body, options).await?;
        let selected = match spec.field {
            Some(field) => match response.value.get(field).cloned() {
                Some(value) => value,
                None => {
                    let error = serde_json::Error::io(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("missing {field} in Inttegro response"),
                    ));
                    self.finish_failure(FailureContext {
                        operation: spec.operation,
                        method: spec.method,
                        route: spec.path,
                        started: response.started,
                        status: Some(response.status),
                        request_id: response.request_id,
                        code: None,
                        category: "decoding",
                        fix_code: None,
                    });
                    return Err(InttegroError::Decode(error));
                }
            },
            None => response.value,
        };
        match serde_json::from_value(selected) {
            Ok(value) => {
                self.record(
                    "inttegro.response.decoded",
                    spec.operation,
                    spec.method,
                    spec.path,
                    Some(response.status),
                    response.started,
                );
                Ok(value)
            }
            Err(error) => {
                self.finish_failure(FailureContext {
                    operation: spec.operation,
                    method: spec.method,
                    route: spec.path,
                    started: response.started,
                    status: Some(response.status),
                    request_id: response.request_id,
                    code: None,
                    category: "decoding",
                    fix_code: None,
                });
                Err(InttegroError::Decode(error))
            }
        }
    }

    async fn request_json<B: Serialize + ?Sized>(
        &self,
        spec: RequestSpec<'_>,
        body: Option<&B>,
        options: RequestOptions,
    ) -> Result<JsonResponse> {
        let started = Instant::now();
        let url = format!("{}{}", self.inner.base_url, spec.path);
        let mut request = self.inner.http.request(
            spec.method.parse().map_err(|_| {
                InttegroError::Configuration(format!("invalid HTTP method {}", spec.method))
            })?,
            url,
        );
        request = request.header(USER_AGENT, format!("inttegro-rust/{VERSION}"));
        if spec.authenticated {
            request = request.header(AUTHORIZATION, format!("Bearer {}", self.inner.api_key));
        }
        if let Some(key) = options.idempotency_key {
            request = request.header("Idempotency-Key", key);
        }
        request = request.headers(options.headers);
        if let Some(body) = body {
            request = request.json(body);
        }
        self.record(
            "inttegro.request.prepared",
            spec.operation,
            spec.method,
            spec.path,
            None,
            started,
        );
        let response = match request.send().await {
            Ok(response) => response,
            Err(error) => {
                self.finish_failure(FailureContext {
                    operation: spec.operation,
                    method: spec.method,
                    route: spec.path,
                    started,
                    status: None,
                    request_id: None,
                    code: None,
                    category: "transport",
                    fix_code: None,
                });
                return Err(InttegroError::Transport(error));
            }
        };
        let status = response.status();
        let request_id = response
            .headers()
            .get("x-request-id")
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        self.record(
            "inttegro.response.received",
            spec.operation,
            spec.method,
            spec.path,
            Some(status.as_u16()),
            started,
        );
        let bytes = match response.bytes().await {
            Ok(bytes) => bytes,
            Err(error) => {
                self.finish_failure(FailureContext {
                    operation: spec.operation,
                    method: spec.method,
                    route: spec.path,
                    started,
                    status: Some(status.as_u16()),
                    request_id: request_id.clone(),
                    code: None,
                    category: "transport",
                    fix_code: None,
                });
                return Err(InttegroError::Transport(error));
            }
        };
        if !status.is_success() {
            let payload: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
            let error = payload.get("error").unwrap_or(&payload);
            let code = error
                .get("code")
                .and_then(Value::as_str)
                .unwrap_or("api_error")
                .to_owned();
            let error_type = error.get("type").and_then(Value::as_str).map(str::to_owned);
            let fix_code = error
                .get("fix_code")
                .and_then(Value::as_str)
                .map(str::to_owned);
            let report = self.finish_failure(FailureContext {
                operation: spec.operation,
                method: spec.method,
                route: spec.path,
                started,
                status: Some(status.as_u16()),
                request_id: request_id.clone(),
                code: Some(code.clone()),
                category: error_type.as_deref().unwrap_or("api"),
                fix_code: fix_code.clone(),
            });
            return Err(InttegroError::Api {
                status: status.as_u16(),
                code,
                error_type,
                fix_code,
                request_id,
                report: report.map(Box::new),
            });
        }
        let value = match serde_json::from_slice(&bytes) {
            Ok(value) => value,
            Err(error) => {
                self.finish_failure(FailureContext {
                    operation: spec.operation,
                    method: spec.method,
                    route: spec.path,
                    started,
                    status: Some(status.as_u16()),
                    request_id: request_id.clone(),
                    code: None,
                    category: "decoding",
                    fix_code: None,
                });
                return Err(InttegroError::Decode(error));
            }
        };
        Ok(JsonResponse {
            value,
            status: status.as_u16(),
            request_id,
            started,
        })
    }

    async fn download<B: Serialize + ?Sized>(
        &self,
        method: &str,
        path: &str,
        body: Option<&B>,
        options: RequestOptions,
        operation: &str,
        authenticated: bool,
    ) -> Result<FileDownload> {
        let started = Instant::now();
        let url = format!("{}{}", self.inner.base_url, path);
        let mut request = self.inner.http.request(
            method.parse().map_err(|_| {
                InttegroError::Configuration(format!("invalid HTTP method {method}"))
            })?,
            url,
        );
        request = request.header(USER_AGENT, format!("inttegro-rust/{VERSION}"));
        if authenticated {
            request = request.bearer_auth(&self.inner.api_key);
        }
        if let Some(key) = options.idempotency_key {
            request = request.header("Idempotency-Key", key);
        }
        request = request.headers(options.headers);
        if let Some(body) = body {
            request = request.json(body);
        }
        self.record(
            "inttegro.request.prepared",
            operation,
            method,
            path,
            None,
            started,
        );
        let response = match request.send().await {
            Ok(response) => response,
            Err(error) => {
                self.finish_failure(FailureContext {
                    operation,
                    method,
                    route: path,
                    started,
                    status: None,
                    request_id: None,
                    code: None,
                    category: "transport",
                    fix_code: None,
                });
                return Err(InttegroError::Transport(error));
            }
        };
        let status = response.status();
        let request_id = response
            .headers()
            .get("x-request-id")
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        self.record(
            "inttegro.response.received",
            operation,
            method,
            path,
            Some(status.as_u16()),
            started,
        );
        if !status.is_success() {
            let report = self.finish_failure(FailureContext {
                operation,
                method,
                route: path,
                started,
                status: Some(status.as_u16()),
                request_id: request_id.clone(),
                code: Some("download_failed".into()),
                category: "api",
                fix_code: None,
            });
            return Err(InttegroError::Api {
                status: status.as_u16(),
                code: "download_failed".into(),
                error_type: None,
                fix_code: None,
                request_id,
                report: report.map(Box::new),
            });
        }
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned);
        let bytes = match response.bytes().await {
            Ok(bytes) => bytes,
            Err(error) => {
                self.finish_failure(FailureContext {
                    operation,
                    method,
                    route: path,
                    started,
                    status: Some(status.as_u16()),
                    request_id,
                    code: None,
                    category: "transport",
                    fix_code: None,
                });
                return Err(InttegroError::Transport(error));
            }
        };
        Ok(FileDownload {
            bytes,
            content_type,
            filename: None,
        })
    }

    async fn upload_file<T: DeserializeOwned>(
        &self,
        path: &str,
        request: CreateFileRequest,
        options: RequestOptions,
        operation: &str,
        field: Option<&str>,
        authenticated: bool,
    ) -> Result<T> {
        let mut form = reqwest::multipart::Form::new()
            .text("purpose", request.purpose)
            .part(
                "file",
                reqwest::multipart::Part::bytes(request.bytes.to_vec())
                    .file_name(request.file_name),
            );
        if let Some(title) = request.title {
            form = form.text("title", title);
        }
        if let Some(custom_data) = request.custom_data {
            form = form.text("custom_data", serde_json::to_string(&custom_data)?);
        }
        let response = self
            .request_multipart(path, form, options, operation, authenticated, &[])
            .await?;
        let selected = field
            .and_then(|name| response.value.get(name))
            .cloned()
            .unwrap_or(response.value);
        match serde_json::from_value(selected) {
            Ok(value) => {
                self.record(
                    "inttegro.response.decoded",
                    operation,
                    "POST",
                    path,
                    Some(response.status),
                    response.started,
                );
                Ok(value)
            }
            Err(error) => {
                self.finish_failure(FailureContext {
                    operation,
                    method: "POST",
                    route: path,
                    started: response.started,
                    status: Some(response.status),
                    request_id: response.request_id,
                    code: None,
                    category: "decoding",
                    fix_code: None,
                });
                Err(InttegroError::Decode(error))
            }
        }
    }

    async fn fulfill_upload(
        &self,
        path: &str,
        request: FulfillUploadRequest,
        options: RequestOptions,
        operation: &str,
    ) -> Result<UploadFulfillment> {
        let query = [("id", request.id), ("token", request.token)];
        let form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(request.bytes.to_vec()).file_name(request.file_name),
        );
        let response = self
            .request_multipart(path, form, options, operation, false, &query)
            .await?;
        match serde_json::from_value(response.value) {
            Ok(value) => {
                self.record(
                    "inttegro.response.decoded",
                    operation,
                    "POST",
                    path,
                    Some(response.status),
                    response.started,
                );
                Ok(value)
            }
            Err(error) => {
                self.finish_failure(FailureContext {
                    operation,
                    method: "POST",
                    route: path,
                    started: response.started,
                    status: Some(response.status),
                    request_id: response.request_id,
                    code: None,
                    category: "decoding",
                    fix_code: None,
                });
                Err(InttegroError::Decode(error))
            }
        }
    }

    async fn request_multipart(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
        options: RequestOptions,
        operation: &str,
        authenticated: bool,
        query: &[(&str, String)],
    ) -> Result<JsonResponse> {
        let started = Instant::now();
        let url = format!("{}{}", self.inner.base_url, path);
        let idempotency_key = options.idempotency_key;
        let mut request = self
            .inner
            .http
            .post(url)
            .query(query)
            .multipart(form)
            .headers(options.headers);
        request = request.header(USER_AGENT, format!("inttegro-rust/{VERSION}"));
        if authenticated {
            request = request.bearer_auth(&self.inner.api_key);
        }
        if let Some(key) = idempotency_key {
            request = request.header("Idempotency-Key", key);
        }
        self.record(
            "inttegro.request.prepared",
            operation,
            "POST",
            path,
            None,
            started,
        );
        let response = match request.send().await {
            Ok(response) => response,
            Err(error) => {
                self.finish_failure(FailureContext {
                    operation,
                    method: "POST",
                    route: path,
                    started,
                    status: None,
                    request_id: None,
                    code: None,
                    category: "transport",
                    fix_code: None,
                });
                return Err(InttegroError::Transport(error));
            }
        };
        let status = response.status();
        let request_id = response
            .headers()
            .get("x-request-id")
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        self.record(
            "inttegro.response.received",
            operation,
            "POST",
            path,
            Some(status.as_u16()),
            started,
        );
        let bytes = match response.bytes().await {
            Ok(bytes) => bytes,
            Err(error) => {
                self.finish_failure(FailureContext {
                    operation,
                    method: "POST",
                    route: path,
                    started,
                    status: Some(status.as_u16()),
                    request_id: request_id.clone(),
                    code: None,
                    category: "transport",
                    fix_code: None,
                });
                return Err(InttegroError::Transport(error));
            }
        };
        if !status.is_success() {
            let report = self.finish_failure(FailureContext {
                operation,
                method: "POST",
                route: path,
                started,
                status: Some(status.as_u16()),
                request_id: request_id.clone(),
                code: Some("upload_failed".into()),
                category: "api",
                fix_code: None,
            });
            return Err(InttegroError::Api {
                status: status.as_u16(),
                code: "upload_failed".into(),
                error_type: None,
                fix_code: None,
                request_id,
                report: report.map(Box::new),
            });
        }
        let value = match serde_json::from_slice(&bytes) {
            Ok(value) => value,
            Err(error) => {
                self.finish_failure(FailureContext {
                    operation,
                    method: "POST",
                    route: path,
                    started,
                    status: Some(status.as_u16()),
                    request_id: request_id.clone(),
                    code: None,
                    category: "decoding",
                    fix_code: None,
                });
                return Err(InttegroError::Decode(error));
            }
        };
        Ok(JsonResponse {
            value,
            status: status.as_u16(),
            request_id,
            started,
        })
    }

    async fn open_file_link(
        &self,
        path: &str,
        request: OpenFileLinkRequest,
        operation: &str,
    ) -> Result<FileDownload> {
        let url = format!("{}{}", self.inner.base_url, path);
        let started = Instant::now();
        self.record(
            "inttegro.request.prepared",
            operation,
            "GET",
            path,
            None,
            started,
        );
        let response = match self
            .inner
            .http
            .get(url)
            .header(USER_AGENT, format!("inttegro-rust/{VERSION}"))
            .query(&[("id", request.id), ("token", request.token)])
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => {
                self.finish_failure(FailureContext {
                    operation,
                    method: "GET",
                    route: path,
                    started,
                    status: None,
                    request_id: None,
                    code: None,
                    category: "transport",
                    fix_code: None,
                });
                return Err(InttegroError::Transport(error));
            }
        };
        let status = response.status();
        let request_id = response
            .headers()
            .get("x-request-id")
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        self.record(
            "inttegro.response.received",
            operation,
            "GET",
            path,
            Some(status.as_u16()),
            started,
        );
        if !status.is_success() {
            let report = self.finish_failure(FailureContext {
                operation,
                method: "GET",
                route: path,
                started,
                status: Some(status.as_u16()),
                request_id: request_id.clone(),
                code: Some("file_link_failed".into()),
                category: "api",
                fix_code: None,
            });
            return Err(InttegroError::Api {
                status: status.as_u16(),
                code: "file_link_failed".into(),
                error_type: None,
                fix_code: None,
                request_id,
                report: report.map(Box::new),
            });
        }
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned);
        let bytes = match response.bytes().await {
            Ok(bytes) => bytes,
            Err(error) => {
                self.finish_failure(FailureContext {
                    operation,
                    method: "GET",
                    route: path,
                    started,
                    status: Some(status.as_u16()),
                    request_id,
                    code: None,
                    category: "transport",
                    fix_code: None,
                });
                return Err(InttegroError::Transport(error));
            }
        };
        Ok(FileDownload {
            bytes,
            content_type,
            filename: None,
        })
    }

    fn record(
        &self,
        name: &str,
        operation: &str,
        method: &str,
        route: &str,
        status_code: Option<u16>,
        started: Instant,
    ) {
        let Some(telemetry) = &self.inner.telemetry else {
            return;
        };
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            telemetry.record(TelemetryEvent {
                name,
                operation,
                method,
                route,
                status_code,
                duration_ms: started.elapsed().as_millis() as u64,
            })
        }));
    }

    fn finish_failure(&self, context: FailureContext<'_>) -> Option<ErrorReport> {
        self.record(
            "inttegro.request.failed",
            context.operation,
            context.method,
            context.route,
            context.status,
            context.started,
        );
        let reporter = self.inner.error_reporter.as_ref()?;
        if self.inner.error_reporting_policy == ErrorReportingPolicy::Unexpected
            && context.status.is_some_and(|value| value < 500)
            && context.category != "transport"
            && context.category != "decoding"
            && context.category != "unknown_error"
        {
            return None;
        }
        let api_error = context.status.map(|_| APIErrorReportContext {
            r#type: Some(context.category.into()),
            code: context.code,
            fix_code: context.fix_code,
        });
        let report = ErrorReport {
            schema_version: 1,
            event_id: uuid::Uuid::now_v7().to_string(),
            occurred_at: time::OffsetDateTime::now_utc()
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_default(),
            severity: "error".into(),
            category: context.category.into(),
            operation: context.operation.into(),
            sdk: SDKReportContext {
                language: "rust".into(),
                version: VERSION.into(),
            },
            http: HTTPReportContext {
                method: context.method.to_uppercase(),
                route: Some(context.route.into()),
                server_address: self.inner.base_url.clone(),
                status_code: context.status,
                request_id: context.request_id,
                duration_ms: context.started.elapsed().as_millis() as u64,
            },
            api_error,
            trace: None,
            exception_type: if context.status.is_some() {
                "InttegroApiError".into()
            } else {
                "reqwest::Error".into()
            },
            fingerprint: format!(
                "inttegro:rust:{}:{}:{}",
                context.operation,
                context.category,
                context
                    .status
                    .map_or_else(|| "none".into(), |value| value.to_string())
            ),
        };
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            reporter.report(report.clone())
        }));
        Some(report)
    }
}

pub struct ClientBuilder {
    api_key: String,
    base_url: String,
    http: Option<reqwest::Client>,
    telemetry: Option<Arc<dyn Telemetry>>,
    error_reporter: Option<Arc<dyn ErrorReporter>>,
    error_reporting_policy: ErrorReportingPolicy,
}

impl ClientBuilder {
    fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.into(),
            http: None,
            telemetry: None,
            error_reporter: None,
            error_reporting_policy: ErrorReportingPolicy::Unexpected,
        }
    }
    pub fn base_url(mut self, value: impl Into<String>) -> Self {
        self.base_url = value.into().trim_end_matches('/').to_owned();
        self
    }
    pub fn http_client(mut self, value: reqwest::Client) -> Self {
        self.http = Some(value);
        self
    }
    pub fn telemetry(mut self, value: Arc<dyn Telemetry>) -> Self {
        self.telemetry = Some(value);
        self
    }
    pub fn error_reporter(mut self, value: Arc<dyn ErrorReporter>) -> Self {
        self.error_reporter = Some(value);
        self
    }
    pub fn error_reporting_policy(mut self, value: ErrorReportingPolicy) -> Self {
        self.error_reporting_policy = value;
        self
    }
    pub fn build(self) -> Result<Client> {
        if self.api_key.trim().is_empty() {
            return Err(InttegroError::Configuration(
                "api key cannot be empty".into(),
            ));
        }
        let http = self.http.unwrap_or_default();
        Ok(Client {
            inner: Arc::new(ClientInner {
                api_key: self.api_key,
                base_url: self.base_url,
                http,
                telemetry: self.telemetry,
                error_reporter: self.error_reporter,
                error_reporting_policy: self.error_reporting_policy,
            }),
        })
    }
}
