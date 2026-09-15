//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro message templates.
#[derive(Clone)]
pub struct MessageTemplates {
    client: Client,
}

impl MessageTemplates {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a message template
    pub async fn create(&self, request: &CreateMessageTemplateRequest) -> Result<MessageTemplate> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }
    /// Create a message template with per-request options.
    pub async fn create_with_options(
        &self,
        request: &CreateMessageTemplateRequest,
        options: RequestOptions,
    ) -> Result<MessageTemplate> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/message_templates/create",
                    operation: "message_templates.create",
                    field: Some("message_template"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Update a message template
    pub async fn update(&self, request: &UpdateMessageTemplateRequest) -> Result<MessageTemplate> {
        self.update_with_options(request, RequestOptions::default())
            .await
    }
    /// Update a message template with per-request options.
    pub async fn update_with_options(
        &self,
        request: &UpdateMessageTemplateRequest,
        options: RequestOptions,
    ) -> Result<MessageTemplate> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/message_templates/update",
                    operation: "message_templates.update",
                    field: Some("message_template"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Publish a message template
    pub async fn publish(&self, request: &MessageTemplateIDRequest) -> Result<MessageTemplate> {
        self.publish_with_options(request, RequestOptions::default())
            .await
    }
    /// Publish a message template with per-request options.
    pub async fn publish_with_options(
        &self,
        request: &MessageTemplateIDRequest,
        options: RequestOptions,
    ) -> Result<MessageTemplate> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/message_templates/publish",
                    operation: "message_templates.publish",
                    field: Some("message_template"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Archive a message template
    pub async fn archive(&self, request: &MessageTemplateIDRequest) -> Result<MessageTemplate> {
        self.archive_with_options(request, RequestOptions::default())
            .await
    }
    /// Archive a message template with per-request options.
    pub async fn archive_with_options(
        &self,
        request: &MessageTemplateIDRequest,
        options: RequestOptions,
    ) -> Result<MessageTemplate> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/message_templates/archive",
                    operation: "message_templates.archive",
                    field: Some("message_template"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Look up a message template
    pub async fn lookup(&self, request: &MessageTemplateIDRequest) -> Result<MessageTemplate> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Look up a message template with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &MessageTemplateIDRequest,
        options: RequestOptions,
    ) -> Result<MessageTemplate> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/message_templates/lookup",
                    operation: "message_templates.lookup",
                    field: Some("message_template"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Page message templates
    pub async fn page(
        &self,
        request: &PageMessageTemplatesRequest,
    ) -> Result<MessageTemplatesPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page message templates with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PageMessageTemplatesRequest,
        options: RequestOptions,
    ) -> Result<MessageTemplatesPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/message_templates/page",
                    operation: "message_templates.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Render a message template preview
    pub async fn render_preview(
        &self,
        request: &RenderMessageTemplatePreviewRequest,
    ) -> Result<MessageTemplatePreview> {
        self.render_preview_with_options(request, RequestOptions::default())
            .await
    }
    /// Render a message template preview with per-request options.
    pub async fn render_preview_with_options(
        &self,
        request: &RenderMessageTemplatePreviewRequest,
        options: RequestOptions,
    ) -> Result<MessageTemplatePreview> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/message_templates/render_preview",
                    operation: "message_templates.render_preview",
                    field: None,
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}
