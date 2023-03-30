// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CompleteSnapshot`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`complete_snapshot`](crate::client::Client::complete_snapshot).
            ///
            /// See [`crate::client::fluent_builders::CompleteSnapshot`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CompleteSnapshot {
    _private: ()
}
impl CompleteSnapshot {
    /// Creates a new builder-style object to manufacture [`CompleteSnapshotInput`](crate::input::CompleteSnapshotInput).
    pub fn builder() -> crate::input::complete_snapshot_input::Builder {
        crate::input::complete_snapshot_input::Builder::default()
    }
    /// Creates a new `CompleteSnapshot` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CompleteSnapshot {
                type Output = std::result::Result<crate::output::CompleteSnapshotOutput, crate::error::CompleteSnapshotError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 202 {
                        crate::operation_deser::parse_complete_snapshot_error(response)
                     } else {
                        crate::operation_deser::parse_complete_snapshot_response(response)
                     }
                }
            }
#[allow(unreachable_code, unused_variables)]
#[cfg(test)]
mod complete_snapshot_request_test {
    /// This test case validates case insensitive parsing of `message`
    /// Test ID: LowercaseMessage
    #[tokio::test]
    async fn lowercase_message_response() {
        let expected_output =crate::error::ValidationException::builder()
        .set_message(
            Some(
                "1 validation error detected".to_owned()
            )
        )
        .build()
        ;
        let http_response = http::response::Builder::new()
        .header("content-length", "77")
        .header("content-type", "application/json")
        .header("date", "Wed, 30 Jun 2021 23:42:27 GMT")
        .header("x-amzn-errortype", "ValidationException:http://internal.amazon.com/coral/com.amazon.coral.validate/")
        .header("x-amzn-requestid", "2af8f013-250a-4f6e-88ae-6dd7f6e12807")
        .status(400)
                    .body(aws_smithy_http::body::SdkBody::from("{\n  \"message\": \"1 validation error detected\"\n}\n"))
                    .unwrap();
        let mut op_response = aws_smithy_http::operation::Response::new(http_response);
        use aws_smithy_http::response::ParseHttpResponse;
                    let parser = crate::operation::CompleteSnapshot::new();
                    let parsed = parser.parse_unloaded(&mut op_response);
                    let parsed = parsed.unwrap_or_else(|| {
                        let (http_response, _) = op_response.into_parts();
                        let http_response = http_response.map(|body|bytes::Bytes::copy_from_slice(body.bytes().unwrap()));
                        <crate::operation::CompleteSnapshot as aws_smithy_http::response::ParseHttpResponse>::parse_loaded(&parser, &http_response)
                    });
        let parsed = parsed.expect_err("should be error response");
        if let crate::error::CompleteSnapshotErrorKind::ValidationException(actual_error) = parsed.kind {
            pretty_assertions::assert_eq!(expected_output, actual_error);
        }
        else {
            panic!("wrong variant: Got: {:?}. Expected: {:?}", parsed, expected_output);
        }
    }
    /// This test case validates case insensitive parsing of `message`
    /// Test ID: UppercaseMessage
    #[tokio::test]
    async fn uppercase_message_response() {
        let expected_output =crate::error::ValidationException::builder()
        .set_message(
            Some(
                "Invalid volume size: 99999999999".to_owned()
            )
        )
        .set_reason(
            Some(
                crate::model::ValidationExceptionReason::from("INVALID_VOLUME_SIZE")
            )
        )
        .build()
        ;
        let http_response = http::response::Builder::new()
        .header("content-length", "77")
        .header("content-type", "application/json")
        .header("date", "Wed, 30 Jun 2021 23:42:27 GMT")
        .header("x-amzn-errortype", "ValidationException:http://internal.amazon.com/coral/com.amazon.zeppelindataservice/")
        .header("x-amzn-requestid", "2af8f013-250a-4f6e-88ae-6dd7f6e12807")
        .status(400)
                    .body(aws_smithy_http::body::SdkBody::from("{\"Message\":\"Invalid volume size: 99999999999\",\"Reason\":\"INVALID_VOLUME_SIZE\"}\n"))
                    .unwrap();
        let mut op_response = aws_smithy_http::operation::Response::new(http_response);
        use aws_smithy_http::response::ParseHttpResponse;
                    let parser = crate::operation::CompleteSnapshot::new();
                    let parsed = parser.parse_unloaded(&mut op_response);
                    let parsed = parsed.unwrap_or_else(|| {
                        let (http_response, _) = op_response.into_parts();
                        let http_response = http_response.map(|body|bytes::Bytes::copy_from_slice(body.bytes().unwrap()));
                        <crate::operation::CompleteSnapshot as aws_smithy_http::response::ParseHttpResponse>::parse_loaded(&parser, &http_response)
                    });
        let parsed = parsed.expect_err("should be error response");
        if let crate::error::CompleteSnapshotErrorKind::ValidationException(actual_error) = parsed.kind {
            pretty_assertions::assert_eq!(expected_output, actual_error);
        }
        else {
            panic!("wrong variant: Got: {:?}. Expected: {:?}", parsed, expected_output);
        }
    }
    
    
}

/// Operation shape for `GetSnapshotBlock`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_snapshot_block`](crate::client::Client::get_snapshot_block).
            ///
            /// See [`crate::client::fluent_builders::GetSnapshotBlock`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetSnapshotBlock {
    _private: ()
}
impl GetSnapshotBlock {
    /// Creates a new builder-style object to manufacture [`GetSnapshotBlockInput`](crate::input::GetSnapshotBlockInput).
    pub fn builder() -> crate::input::get_snapshot_block_input::Builder {
        crate::input::get_snapshot_block_input::Builder::default()
    }
    /// Creates a new `GetSnapshotBlock` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseHttpResponse for GetSnapshotBlock {
                type Output = std::result::Result<crate::output::GetSnapshotBlockOutput, crate::error::GetSnapshotBlockError>;
                fn parse_unloaded(&self, response: &mut aws_smithy_http::operation::Response) -> Option<Self::Output> {
                    // This is an error, defer to the non-streaming parser
                    if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
                        return None;
                    }
                    Some(crate::operation_deser::parse_get_snapshot_block(response))
                }
                fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                    // if streaming, we only hit this case if its an error
                    crate::operation_deser::parse_get_snapshot_block_error(response)
                }
            }
#[allow(unreachable_code, unused_variables)]
#[cfg(test)]
mod get_snapshot_block_request_test {
    /// This test case validates case insensitive parsing of `message`
    /// Test ID: LowercaseMessage
    #[tokio::test]
    async fn lowercase_message_response() {
        let expected_output =crate::error::ValidationException::builder()
        .set_message(
            Some(
                "1 validation error detected".to_owned()
            )
        )
        .build()
        ;
        let http_response = http::response::Builder::new()
        .header("content-length", "77")
        .header("content-type", "application/json")
        .header("date", "Wed, 30 Jun 2021 23:42:27 GMT")
        .header("x-amzn-errortype", "ValidationException:http://internal.amazon.com/coral/com.amazon.coral.validate/")
        .header("x-amzn-requestid", "2af8f013-250a-4f6e-88ae-6dd7f6e12807")
        .status(400)
                    .body(aws_smithy_http::body::SdkBody::from("{\n  \"message\": \"1 validation error detected\"\n}\n"))
                    .unwrap();
        let mut op_response = aws_smithy_http::operation::Response::new(http_response);
        use aws_smithy_http::response::ParseHttpResponse;
                    let parser = crate::operation::GetSnapshotBlock::new();
                    let parsed = parser.parse_unloaded(&mut op_response);
                    let parsed = parsed.unwrap_or_else(|| {
                        let (http_response, _) = op_response.into_parts();
                        let http_response = http_response.map(|body|bytes::Bytes::copy_from_slice(body.bytes().unwrap()));
                        <crate::operation::GetSnapshotBlock as aws_smithy_http::response::ParseHttpResponse>::parse_loaded(&parser, &http_response)
                    });
        let parsed = parsed.expect_err("should be error response");
        if let crate::error::GetSnapshotBlockErrorKind::ValidationException(actual_error) = parsed.kind {
            pretty_assertions::assert_eq!(expected_output, actual_error);
        }
        else {
            panic!("wrong variant: Got: {:?}. Expected: {:?}", parsed, expected_output);
        }
    }
    /// This test case validates case insensitive parsing of `message`
    /// Test ID: UppercaseMessage
    #[tokio::test]
    async fn uppercase_message_response() {
        let expected_output =crate::error::ValidationException::builder()
        .set_message(
            Some(
                "Invalid volume size: 99999999999".to_owned()
            )
        )
        .set_reason(
            Some(
                crate::model::ValidationExceptionReason::from("INVALID_VOLUME_SIZE")
            )
        )
        .build()
        ;
        let http_response = http::response::Builder::new()
        .header("content-length", "77")
        .header("content-type", "application/json")
        .header("date", "Wed, 30 Jun 2021 23:42:27 GMT")
        .header("x-amzn-errortype", "ValidationException:http://internal.amazon.com/coral/com.amazon.zeppelindataservice/")
        .header("x-amzn-requestid", "2af8f013-250a-4f6e-88ae-6dd7f6e12807")
        .status(400)
                    .body(aws_smithy_http::body::SdkBody::from("{\"Message\":\"Invalid volume size: 99999999999\",\"Reason\":\"INVALID_VOLUME_SIZE\"}\n"))
                    .unwrap();
        let mut op_response = aws_smithy_http::operation::Response::new(http_response);
        use aws_smithy_http::response::ParseHttpResponse;
                    let parser = crate::operation::GetSnapshotBlock::new();
                    let parsed = parser.parse_unloaded(&mut op_response);
                    let parsed = parsed.unwrap_or_else(|| {
                        let (http_response, _) = op_response.into_parts();
                        let http_response = http_response.map(|body|bytes::Bytes::copy_from_slice(body.bytes().unwrap()));
                        <crate::operation::GetSnapshotBlock as aws_smithy_http::response::ParseHttpResponse>::parse_loaded(&parser, &http_response)
                    });
        let parsed = parsed.expect_err("should be error response");
        if let crate::error::GetSnapshotBlockErrorKind::ValidationException(actual_error) = parsed.kind {
            pretty_assertions::assert_eq!(expected_output, actual_error);
        }
        else {
            panic!("wrong variant: Got: {:?}. Expected: {:?}", parsed, expected_output);
        }
    }
    
    
}

/// Operation shape for `ListChangedBlocks`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_changed_blocks`](crate::client::Client::list_changed_blocks).
            ///
            /// See [`crate::client::fluent_builders::ListChangedBlocks`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListChangedBlocks {
    _private: ()
}
impl ListChangedBlocks {
    /// Creates a new builder-style object to manufacture [`ListChangedBlocksInput`](crate::input::ListChangedBlocksInput).
    pub fn builder() -> crate::input::list_changed_blocks_input::Builder {
        crate::input::list_changed_blocks_input::Builder::default()
    }
    /// Creates a new `ListChangedBlocks` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListChangedBlocks {
                type Output = std::result::Result<crate::output::ListChangedBlocksOutput, crate::error::ListChangedBlocksError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_changed_blocks_error(response)
                     } else {
                        crate::operation_deser::parse_list_changed_blocks_response(response)
                     }
                }
            }
#[allow(unreachable_code, unused_variables)]
#[cfg(test)]
mod list_changed_blocks_request_test {
    /// This test case validates case insensitive parsing of `message`
    /// Test ID: LowercaseMessage
    #[tokio::test]
    async fn lowercase_message_response() {
        let expected_output =crate::error::ValidationException::builder()
        .set_message(
            Some(
                "1 validation error detected".to_owned()
            )
        )
        .build()
        ;
        let http_response = http::response::Builder::new()
        .header("content-length", "77")
        .header("content-type", "application/json")
        .header("date", "Wed, 30 Jun 2021 23:42:27 GMT")
        .header("x-amzn-errortype", "ValidationException:http://internal.amazon.com/coral/com.amazon.coral.validate/")
        .header("x-amzn-requestid", "2af8f013-250a-4f6e-88ae-6dd7f6e12807")
        .status(400)
                    .body(aws_smithy_http::body::SdkBody::from("{\n  \"message\": \"1 validation error detected\"\n}\n"))
                    .unwrap();
        let mut op_response = aws_smithy_http::operation::Response::new(http_response);
        use aws_smithy_http::response::ParseHttpResponse;
                    let parser = crate::operation::ListChangedBlocks::new();
                    let parsed = parser.parse_unloaded(&mut op_response);
                    let parsed = parsed.unwrap_or_else(|| {
                        let (http_response, _) = op_response.into_parts();
                        let http_response = http_response.map(|body|bytes::Bytes::copy_from_slice(body.bytes().unwrap()));
                        <crate::operation::ListChangedBlocks as aws_smithy_http::response::ParseHttpResponse>::parse_loaded(&parser, &http_response)
                    });
        let parsed = parsed.expect_err("should be error response");
        if let crate::error::ListChangedBlocksErrorKind::ValidationException(actual_error) = parsed.kind {
            pretty_assertions::assert_eq!(expected_output, actual_error);
        }
        else {
            panic!("wrong variant: Got: {:?}. Expected: {:?}", parsed, expected_output);
        }
    }
    /// This test case validates case insensitive parsing of `message`
    /// Test ID: UppercaseMessage
    #[tokio::test]
    async fn uppercase_message_response() {
        let expected_output =crate::error::ValidationException::builder()
        .set_message(
            Some(
                "Invalid volume size: 99999999999".to_owned()
            )
        )
        .set_reason(
            Some(
                crate::model::ValidationExceptionReason::from("INVALID_VOLUME_SIZE")
            )
        )
        .build()
        ;
        let http_response = http::response::Builder::new()
        .header("content-length", "77")
        .header("content-type", "application/json")
        .header("date", "Wed, 30 Jun 2021 23:42:27 GMT")
        .header("x-amzn-errortype", "ValidationException:http://internal.amazon.com/coral/com.amazon.zeppelindataservice/")
        .header("x-amzn-requestid", "2af8f013-250a-4f6e-88ae-6dd7f6e12807")
        .status(400)
                    .body(aws_smithy_http::body::SdkBody::from("{\"Message\":\"Invalid volume size: 99999999999\",\"Reason\":\"INVALID_VOLUME_SIZE\"}\n"))
                    .unwrap();
        let mut op_response = aws_smithy_http::operation::Response::new(http_response);
        use aws_smithy_http::response::ParseHttpResponse;
                    let parser = crate::operation::ListChangedBlocks::new();
                    let parsed = parser.parse_unloaded(&mut op_response);
                    let parsed = parsed.unwrap_or_else(|| {
                        let (http_response, _) = op_response.into_parts();
                        let http_response = http_response.map(|body|bytes::Bytes::copy_from_slice(body.bytes().unwrap()));
                        <crate::operation::ListChangedBlocks as aws_smithy_http::response::ParseHttpResponse>::parse_loaded(&parser, &http_response)
                    });
        let parsed = parsed.expect_err("should be error response");
        if let crate::error::ListChangedBlocksErrorKind::ValidationException(actual_error) = parsed.kind {
            pretty_assertions::assert_eq!(expected_output, actual_error);
        }
        else {
            panic!("wrong variant: Got: {:?}. Expected: {:?}", parsed, expected_output);
        }
    }
    
    
}

/// Operation shape for `ListSnapshotBlocks`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_snapshot_blocks`](crate::client::Client::list_snapshot_blocks).
            ///
            /// See [`crate::client::fluent_builders::ListSnapshotBlocks`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListSnapshotBlocks {
    _private: ()
}
impl ListSnapshotBlocks {
    /// Creates a new builder-style object to manufacture [`ListSnapshotBlocksInput`](crate::input::ListSnapshotBlocksInput).
    pub fn builder() -> crate::input::list_snapshot_blocks_input::Builder {
        crate::input::list_snapshot_blocks_input::Builder::default()
    }
    /// Creates a new `ListSnapshotBlocks` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListSnapshotBlocks {
                type Output = std::result::Result<crate::output::ListSnapshotBlocksOutput, crate::error::ListSnapshotBlocksError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_snapshot_blocks_error(response)
                     } else {
                        crate::operation_deser::parse_list_snapshot_blocks_response(response)
                     }
                }
            }
#[allow(unreachable_code, unused_variables)]
#[cfg(test)]
mod list_snapshot_blocks_request_test {
    /// This test case validates case insensitive parsing of `message`
    /// Test ID: LowercaseMessage
    #[tokio::test]
    async fn lowercase_message_response() {
        let expected_output =crate::error::ValidationException::builder()
        .set_message(
            Some(
                "1 validation error detected".to_owned()
            )
        )
        .build()
        ;
        let http_response = http::response::Builder::new()
        .header("content-length", "77")
        .header("content-type", "application/json")
        .header("date", "Wed, 30 Jun 2021 23:42:27 GMT")
        .header("x-amzn-errortype", "ValidationException:http://internal.amazon.com/coral/com.amazon.coral.validate/")
        .header("x-amzn-requestid", "2af8f013-250a-4f6e-88ae-6dd7f6e12807")
        .status(400)
                    .body(aws_smithy_http::body::SdkBody::from("{\n  \"message\": \"1 validation error detected\"\n}\n"))
                    .unwrap();
        let mut op_response = aws_smithy_http::operation::Response::new(http_response);
        use aws_smithy_http::response::ParseHttpResponse;
                    let parser = crate::operation::ListSnapshotBlocks::new();
                    let parsed = parser.parse_unloaded(&mut op_response);
                    let parsed = parsed.unwrap_or_else(|| {
                        let (http_response, _) = op_response.into_parts();
                        let http_response = http_response.map(|body|bytes::Bytes::copy_from_slice(body.bytes().unwrap()));
                        <crate::operation::ListSnapshotBlocks as aws_smithy_http::response::ParseHttpResponse>::parse_loaded(&parser, &http_response)
                    });
        let parsed = parsed.expect_err("should be error response");
        if let crate::error::ListSnapshotBlocksErrorKind::ValidationException(actual_error) = parsed.kind {
            pretty_assertions::assert_eq!(expected_output, actual_error);
        }
        else {
            panic!("wrong variant: Got: {:?}. Expected: {:?}", parsed, expected_output);
        }
    }
    /// This test case validates case insensitive parsing of `message`
    /// Test ID: UppercaseMessage
    #[tokio::test]
    async fn uppercase_message_response() {
        let expected_output =crate::error::ValidationException::builder()
        .set_message(
            Some(
                "Invalid volume size: 99999999999".to_owned()
            )
        )
        .set_reason(
            Some(
                crate::model::ValidationExceptionReason::from("INVALID_VOLUME_SIZE")
            )
        )
        .build()
        ;
        let http_response = http::response::Builder::new()
        .header("content-length", "77")
        .header("content-type", "application/json")
        .header("date", "Wed, 30 Jun 2021 23:42:27 GMT")
        .header("x-amzn-errortype", "ValidationException:http://internal.amazon.com/coral/com.amazon.zeppelindataservice/")
        .header("x-amzn-requestid", "2af8f013-250a-4f6e-88ae-6dd7f6e12807")
        .status(400)
                    .body(aws_smithy_http::body::SdkBody::from("{\"Message\":\"Invalid volume size: 99999999999\",\"Reason\":\"INVALID_VOLUME_SIZE\"}\n"))
                    .unwrap();
        let mut op_response = aws_smithy_http::operation::Response::new(http_response);
        use aws_smithy_http::response::ParseHttpResponse;
                    let parser = crate::operation::ListSnapshotBlocks::new();
                    let parsed = parser.parse_unloaded(&mut op_response);
                    let parsed = parsed.unwrap_or_else(|| {
                        let (http_response, _) = op_response.into_parts();
                        let http_response = http_response.map(|body|bytes::Bytes::copy_from_slice(body.bytes().unwrap()));
                        <crate::operation::ListSnapshotBlocks as aws_smithy_http::response::ParseHttpResponse>::parse_loaded(&parser, &http_response)
                    });
        let parsed = parsed.expect_err("should be error response");
        if let crate::error::ListSnapshotBlocksErrorKind::ValidationException(actual_error) = parsed.kind {
            pretty_assertions::assert_eq!(expected_output, actual_error);
        }
        else {
            panic!("wrong variant: Got: {:?}. Expected: {:?}", parsed, expected_output);
        }
    }
    
    
}

/// Operation shape for `PutSnapshotBlock`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_snapshot_block`](crate::client::Client::put_snapshot_block).
            ///
            /// See [`crate::client::fluent_builders::PutSnapshotBlock`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutSnapshotBlock {
    _private: ()
}
impl PutSnapshotBlock {
    /// Creates a new builder-style object to manufacture [`PutSnapshotBlockInput`](crate::input::PutSnapshotBlockInput).
    pub fn builder() -> crate::input::put_snapshot_block_input::Builder {
        crate::input::put_snapshot_block_input::Builder::default()
    }
    /// Creates a new `PutSnapshotBlock` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutSnapshotBlock {
                type Output = std::result::Result<crate::output::PutSnapshotBlockOutput, crate::error::PutSnapshotBlockError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 201 {
                        crate::operation_deser::parse_put_snapshot_block_error(response)
                     } else {
                        crate::operation_deser::parse_put_snapshot_block_response(response)
                     }
                }
            }
#[allow(unreachable_code, unused_variables)]
#[cfg(test)]
mod put_snapshot_block_request_test {
    /// This test case validates case insensitive parsing of `message`
    /// Test ID: LowercaseMessage
    #[tokio::test]
    async fn lowercase_message_response() {
        let expected_output =crate::error::ValidationException::builder()
        .set_message(
            Some(
                "1 validation error detected".to_owned()
            )
        )
        .build()
        ;
        let http_response = http::response::Builder::new()
        .header("content-length", "77")
        .header("content-type", "application/json")
        .header("date", "Wed, 30 Jun 2021 23:42:27 GMT")
        .header("x-amzn-errortype", "ValidationException:http://internal.amazon.com/coral/com.amazon.coral.validate/")
        .header("x-amzn-requestid", "2af8f013-250a-4f6e-88ae-6dd7f6e12807")
        .status(400)
                    .body(aws_smithy_http::body::SdkBody::from("{\n  \"message\": \"1 validation error detected\"\n}\n"))
                    .unwrap();
        let mut op_response = aws_smithy_http::operation::Response::new(http_response);
        use aws_smithy_http::response::ParseHttpResponse;
                    let parser = crate::operation::PutSnapshotBlock::new();
                    let parsed = parser.parse_unloaded(&mut op_response);
                    let parsed = parsed.unwrap_or_else(|| {
                        let (http_response, _) = op_response.into_parts();
                        let http_response = http_response.map(|body|bytes::Bytes::copy_from_slice(body.bytes().unwrap()));
                        <crate::operation::PutSnapshotBlock as aws_smithy_http::response::ParseHttpResponse>::parse_loaded(&parser, &http_response)
                    });
        let parsed = parsed.expect_err("should be error response");
        if let crate::error::PutSnapshotBlockErrorKind::ValidationException(actual_error) = parsed.kind {
            pretty_assertions::assert_eq!(expected_output, actual_error);
        }
        else {
            panic!("wrong variant: Got: {:?}. Expected: {:?}", parsed, expected_output);
        }
    }
    /// This test case validates case insensitive parsing of `message`
    /// Test ID: UppercaseMessage
    #[tokio::test]
    async fn uppercase_message_response() {
        let expected_output =crate::error::ValidationException::builder()
        .set_message(
            Some(
                "Invalid volume size: 99999999999".to_owned()
            )
        )
        .set_reason(
            Some(
                crate::model::ValidationExceptionReason::from("INVALID_VOLUME_SIZE")
            )
        )
        .build()
        ;
        let http_response = http::response::Builder::new()
        .header("content-length", "77")
        .header("content-type", "application/json")
        .header("date", "Wed, 30 Jun 2021 23:42:27 GMT")
        .header("x-amzn-errortype", "ValidationException:http://internal.amazon.com/coral/com.amazon.zeppelindataservice/")
        .header("x-amzn-requestid", "2af8f013-250a-4f6e-88ae-6dd7f6e12807")
        .status(400)
                    .body(aws_smithy_http::body::SdkBody::from("{\"Message\":\"Invalid volume size: 99999999999\",\"Reason\":\"INVALID_VOLUME_SIZE\"}\n"))
                    .unwrap();
        let mut op_response = aws_smithy_http::operation::Response::new(http_response);
        use aws_smithy_http::response::ParseHttpResponse;
                    let parser = crate::operation::PutSnapshotBlock::new();
                    let parsed = parser.parse_unloaded(&mut op_response);
                    let parsed = parsed.unwrap_or_else(|| {
                        let (http_response, _) = op_response.into_parts();
                        let http_response = http_response.map(|body|bytes::Bytes::copy_from_slice(body.bytes().unwrap()));
                        <crate::operation::PutSnapshotBlock as aws_smithy_http::response::ParseHttpResponse>::parse_loaded(&parser, &http_response)
                    });
        let parsed = parsed.expect_err("should be error response");
        if let crate::error::PutSnapshotBlockErrorKind::ValidationException(actual_error) = parsed.kind {
            pretty_assertions::assert_eq!(expected_output, actual_error);
        }
        else {
            panic!("wrong variant: Got: {:?}. Expected: {:?}", parsed, expected_output);
        }
    }
    
    
}

/// Operation shape for `StartSnapshot`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`start_snapshot`](crate::client::Client::start_snapshot).
            ///
            /// See [`crate::client::fluent_builders::StartSnapshot`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartSnapshot {
    _private: ()
}
impl StartSnapshot {
    /// Creates a new builder-style object to manufacture [`StartSnapshotInput`](crate::input::StartSnapshotInput).
    pub fn builder() -> crate::input::start_snapshot_input::Builder {
        crate::input::start_snapshot_input::Builder::default()
    }
    /// Creates a new `StartSnapshot` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartSnapshot {
                type Output = std::result::Result<crate::output::StartSnapshotOutput, crate::error::StartSnapshotError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 201 {
                        crate::operation_deser::parse_start_snapshot_error(response)
                     } else {
                        crate::operation_deser::parse_start_snapshot_response(response)
                     }
                }
            }
#[allow(unreachable_code, unused_variables)]
#[cfg(test)]
mod start_snapshot_request_test {
    /// This test case validates case insensitive parsing of `message`
    /// Test ID: LowercaseMessage
    #[tokio::test]
    async fn lowercase_message_response() {
        let expected_output =crate::error::ValidationException::builder()
        .set_message(
            Some(
                "1 validation error detected".to_owned()
            )
        )
        .build()
        ;
        let http_response = http::response::Builder::new()
        .header("content-length", "77")
        .header("content-type", "application/json")
        .header("date", "Wed, 30 Jun 2021 23:42:27 GMT")
        .header("x-amzn-errortype", "ValidationException:http://internal.amazon.com/coral/com.amazon.coral.validate/")
        .header("x-amzn-requestid", "2af8f013-250a-4f6e-88ae-6dd7f6e12807")
        .status(400)
                    .body(aws_smithy_http::body::SdkBody::from("{\n  \"message\": \"1 validation error detected\"\n}\n"))
                    .unwrap();
        let mut op_response = aws_smithy_http::operation::Response::new(http_response);
        use aws_smithy_http::response::ParseHttpResponse;
                    let parser = crate::operation::StartSnapshot::new();
                    let parsed = parser.parse_unloaded(&mut op_response);
                    let parsed = parsed.unwrap_or_else(|| {
                        let (http_response, _) = op_response.into_parts();
                        let http_response = http_response.map(|body|bytes::Bytes::copy_from_slice(body.bytes().unwrap()));
                        <crate::operation::StartSnapshot as aws_smithy_http::response::ParseHttpResponse>::parse_loaded(&parser, &http_response)
                    });
        let parsed = parsed.expect_err("should be error response");
        if let crate::error::StartSnapshotErrorKind::ValidationException(actual_error) = parsed.kind {
            pretty_assertions::assert_eq!(expected_output, actual_error);
        }
        else {
            panic!("wrong variant: Got: {:?}. Expected: {:?}", parsed, expected_output);
        }
    }
    /// This test case validates case insensitive parsing of `message`
    /// Test ID: UppercaseMessage
    #[tokio::test]
    async fn uppercase_message_response() {
        let expected_output =crate::error::ValidationException::builder()
        .set_message(
            Some(
                "Invalid volume size: 99999999999".to_owned()
            )
        )
        .set_reason(
            Some(
                crate::model::ValidationExceptionReason::from("INVALID_VOLUME_SIZE")
            )
        )
        .build()
        ;
        let http_response = http::response::Builder::new()
        .header("content-length", "77")
        .header("content-type", "application/json")
        .header("date", "Wed, 30 Jun 2021 23:42:27 GMT")
        .header("x-amzn-errortype", "ValidationException:http://internal.amazon.com/coral/com.amazon.zeppelindataservice/")
        .header("x-amzn-requestid", "2af8f013-250a-4f6e-88ae-6dd7f6e12807")
        .status(400)
                    .body(aws_smithy_http::body::SdkBody::from("{\"Message\":\"Invalid volume size: 99999999999\",\"Reason\":\"INVALID_VOLUME_SIZE\"}\n"))
                    .unwrap();
        let mut op_response = aws_smithy_http::operation::Response::new(http_response);
        use aws_smithy_http::response::ParseHttpResponse;
                    let parser = crate::operation::StartSnapshot::new();
                    let parsed = parser.parse_unloaded(&mut op_response);
                    let parsed = parsed.unwrap_or_else(|| {
                        let (http_response, _) = op_response.into_parts();
                        let http_response = http_response.map(|body|bytes::Bytes::copy_from_slice(body.bytes().unwrap()));
                        <crate::operation::StartSnapshot as aws_smithy_http::response::ParseHttpResponse>::parse_loaded(&parser, &http_response)
                    });
        let parsed = parsed.expect_err("should be error response");
        if let crate::error::StartSnapshotErrorKind::ValidationException(actual_error) = parsed.kind {
            pretty_assertions::assert_eq!(expected_output, actual_error);
        }
        else {
            panic!("wrong variant: Got: {:?}. Expected: {:?}", parsed, expected_output);
        }
    }
    
    
}

/// Operation customization and supporting types
pub mod customize;

