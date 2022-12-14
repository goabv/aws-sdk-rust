// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The request conflicts with the current state of the resource.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>An unexpected error has occurred.</p>
    InternalFailureException(crate::error::InternalFailureException),
    /// <p>The request is not valid.</p>
    InvalidRequestException(crate::error::InvalidRequestException),
    /// <p>A limit has been exceeded.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The rate exceeds the limit.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalFailureException(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateApplicationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateApplicationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateApplicationError> for Error {
    fn from(err: crate::error::CreateApplicationError) -> Self {
        match err.kind {
            crate::error::CreateApplicationErrorKind::InternalFailureException(inner) => {
                Error::InternalFailureException(inner)
            }
            crate::error::CreateApplicationErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::CreateApplicationErrorKind::LimitExceededException(inner) => {
                Error::LimitExceededException(inner)
            }
            crate::error::CreateApplicationErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::CreateApplicationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteApplicationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteApplicationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteApplicationError> for Error {
    fn from(err: crate::error::DeleteApplicationError) -> Self {
        match err.kind {
            crate::error::DeleteApplicationErrorKind::InternalFailureException(inner) => {
                Error::InternalFailureException(inner)
            }
            crate::error::DeleteApplicationErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::DeleteApplicationErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DeleteApplicationErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DeleteApplicationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeApplicationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeApplicationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeApplicationError> for Error {
    fn from(err: crate::error::DescribeApplicationError) -> Self {
        match err.kind {
            crate::error::DescribeApplicationErrorKind::InternalFailureException(inner) => {
                Error::InternalFailureException(inner)
            }
            crate::error::DescribeApplicationErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::DescribeApplicationErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DescribeApplicationErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DescribeApplicationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListApplicationsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListApplicationsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListApplicationsError> for Error {
    fn from(err: crate::error::ListApplicationsError) -> Self {
        match err.kind {
            crate::error::ListApplicationsErrorKind::InternalFailureException(inner) => {
                Error::InternalFailureException(inner)
            }
            crate::error::ListApplicationsErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::ListApplicationsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListApplicationsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListTagsForResourceError> for Error {
    fn from(err: crate::error::ListTagsForResourceError) -> Self {
        match err.kind {
            crate::error::ListTagsForResourceErrorKind::InternalFailureException(inner) => {
                Error::InternalFailureException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::TagResourceError> for Error {
    fn from(err: crate::error::TagResourceError) -> Self {
        match err.kind {
            crate::error::TagResourceErrorKind::InternalFailureException(inner) => {
                Error::InternalFailureException(inner)
            }
            crate::error::TagResourceErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::TagResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UntagResourceError> for Error {
    fn from(err: crate::error::UntagResourceError) -> Self {
        match err.kind {
            crate::error::UntagResourceErrorKind::InternalFailureException(inner) => {
                Error::InternalFailureException(inner)
            }
            crate::error::UntagResourceErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UntagResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateApplicationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateApplicationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateApplicationError> for Error {
    fn from(err: crate::error::UpdateApplicationError) -> Self {
        match err.kind {
            crate::error::UpdateApplicationErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::UpdateApplicationErrorKind::InternalFailureException(inner) => {
                Error::InternalFailureException(inner)
            }
            crate::error::UpdateApplicationErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::UpdateApplicationErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UpdateApplicationErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::UpdateApplicationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
