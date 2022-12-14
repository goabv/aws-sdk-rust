// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalServiceException(crate::error::InternalServiceException),
    /// <p>The request is not valid.</p>
    InvalidRequestException(crate::error::InvalidRequestException),
    /// <p>The specified resource was not found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The throttling limit has been exceeded.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::InternalServiceException(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::ListRealtimeContactAnalysisSegmentsError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::ListRealtimeContactAnalysisSegmentsError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListRealtimeContactAnalysisSegmentsError> for Error {
    fn from(err: crate::error::ListRealtimeContactAnalysisSegmentsError) -> Self {
        match err.kind {
            crate::error::ListRealtimeContactAnalysisSegmentsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListRealtimeContactAnalysisSegmentsErrorKind::InternalServiceException(inner) => Error::InternalServiceException(inner),
            crate::error::ListRealtimeContactAnalysisSegmentsErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::ListRealtimeContactAnalysisSegmentsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ListRealtimeContactAnalysisSegmentsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::ListRealtimeContactAnalysisSegmentsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl std::error::Error for Error {}
