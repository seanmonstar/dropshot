// Copyright 2023 Oxide Computer Company

//! Raw request extractor

use crate::api_description::{ApiEndpointBodyContentType, ExtensionMode};
use crate::error::HttpError;
use crate::server::ServerContext;
use crate::{ExclusiveExtractor, ExtractorMetadata, RequestContext};
use async_trait::async_trait;
use hyper::body::Body;
use std::fmt::Debug;

/// `RawRequest` is an extractor providing access to the raw underlying
/// [`hyper::Request`].
#[derive(Debug)]
pub struct RawRequest<B> {
    request: hyper::Request<B>,
}

impl<B> RawRequest<B> {
    pub fn into_inner(self) -> hyper::Request<B> {
        self.request
    }
}

#[async_trait]
impl<B> ExclusiveExtractor for RawRequest<B> {
    async fn from_request<Context: ServerContext, ReqBody: Body>(
        _rqctx: &RequestContext<Context>,
        request: hyper::Request<ReqBody>,
    ) -> Result<RawRequest<ReqBody>, HttpError> {
        Ok(RawRequest { request })
    }

    fn metadata(
        _content_type: ApiEndpointBodyContentType,
    ) -> ExtractorMetadata {
        ExtractorMetadata {
            parameters: vec![],
            extension_mode: ExtensionMode::None,
        }
    }
}
