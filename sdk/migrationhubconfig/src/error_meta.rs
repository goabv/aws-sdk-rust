// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>Exception raised to indicate that authorization of an action was successful, when the <code>DryRun</code> flag is set to true.</p>
    DryRunOperation(crate::error::DryRunOperation),
    /// <p>Exception raised when an internal, configuration, or dependency error is encountered.</p>
    InternalServerError(crate::error::InternalServerError),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInputException(crate::error::InvalidInputException),
    /// <p>Exception raised when a request fails due to temporary unavailability of the service.</p>
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::DryRunOperation(inner) => inner.fmt(f),
            Error::InternalServerError(inner) => inner.fmt(f),
            Error::InvalidInputException(inner) => inner.fmt(f),
            Error::ServiceUnavailableException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateHomeRegionControlError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateHomeRegionControlError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateHomeRegionControlError> for Error {
    fn from(err: crate::error::CreateHomeRegionControlError) -> Self {
        match err.kind {
            crate::error::CreateHomeRegionControlErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::CreateHomeRegionControlErrorKind::DryRunOperation(inner) => {
                Error::DryRunOperation(inner)
            }
            crate::error::CreateHomeRegionControlErrorKind::InternalServerError(inner) => {
                Error::InternalServerError(inner)
            }
            crate::error::CreateHomeRegionControlErrorKind::InvalidInputException(inner) => {
                Error::InvalidInputException(inner)
            }
            crate::error::CreateHomeRegionControlErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::CreateHomeRegionControlErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::CreateHomeRegionControlErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeHomeRegionControlsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeHomeRegionControlsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeHomeRegionControlsError> for Error {
    fn from(err: crate::error::DescribeHomeRegionControlsError) -> Self {
        match err.kind {
            crate::error::DescribeHomeRegionControlsErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::DescribeHomeRegionControlsErrorKind::InternalServerError(inner) => {
                Error::InternalServerError(inner)
            }
            crate::error::DescribeHomeRegionControlsErrorKind::InvalidInputException(inner) => {
                Error::InvalidInputException(inner)
            }
            crate::error::DescribeHomeRegionControlsErrorKind::ServiceUnavailableException(
                inner,
            ) => Error::ServiceUnavailableException(inner),
            crate::error::DescribeHomeRegionControlsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DescribeHomeRegionControlsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetHomeRegionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetHomeRegionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetHomeRegionError> for Error {
    fn from(err: crate::error::GetHomeRegionError) -> Self {
        match err.kind {
            crate::error::GetHomeRegionErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::GetHomeRegionErrorKind::InternalServerError(inner) => {
                Error::InternalServerError(inner)
            }
            crate::error::GetHomeRegionErrorKind::InvalidInputException(inner) => {
                Error::InvalidInputException(inner)
            }
            crate::error::GetHomeRegionErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::GetHomeRegionErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::GetHomeRegionErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
