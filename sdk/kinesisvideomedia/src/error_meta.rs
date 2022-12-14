// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceededException(crate::error::ClientLimitExceededException),
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client connections.</p>
    ConnectionLimitExceededException(crate::error::ConnectionLimitExceededException),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgumentException(crate::error::InvalidArgumentException),
    /// <p> Status Code: 400, Caller used wrong endpoint to write data to a stream. On receiving such an exception, the user must call <code>GetDataEndpoint</code> with <code>AccessMode</code> set to "READ" and use the endpoint Kinesis Video returns in the next <code>GetMedia</code> call. </p>
    InvalidEndpointException(crate::error::InvalidEndpointException),
    /// <p>Status Code: 403, The caller is not authorized to perform an operation on the given stream, or the token has expired.</p>
    NotAuthorizedException(crate::error::NotAuthorizedException),
    /// <p>Status Code: 404, The stream with the given name does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ClientLimitExceededException(inner) => inner.fmt(f),
            Error::ConnectionLimitExceededException(inner) => inner.fmt(f),
            Error::InvalidArgumentException(inner) => inner.fmt(f),
            Error::InvalidEndpointException(inner) => inner.fmt(f),
            Error::NotAuthorizedException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetMediaError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetMediaError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetMediaError> for Error {
    fn from(err: crate::error::GetMediaError) -> Self {
        match err.kind {
            crate::error::GetMediaErrorKind::ClientLimitExceededException(inner) => {
                Error::ClientLimitExceededException(inner)
            }
            crate::error::GetMediaErrorKind::ConnectionLimitExceededException(inner) => {
                Error::ConnectionLimitExceededException(inner)
            }
            crate::error::GetMediaErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::GetMediaErrorKind::InvalidEndpointException(inner) => {
                Error::InvalidEndpointException(inner)
            }
            crate::error::GetMediaErrorKind::NotAuthorizedException(inner) => {
                Error::NotAuthorizedException(inner)
            }
            crate::error::GetMediaErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::GetMediaErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
