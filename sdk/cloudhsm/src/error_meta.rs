// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternalException(crate::error::CloudHsmInternalException),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmServiceException(crate::error::CloudHsmServiceException),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequestException(crate::error::InvalidRequestException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CloudHsmInternalException(inner) => inner.fmt(f),
            Error::CloudHsmServiceException(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AddTagsToResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::AddTagsToResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::AddTagsToResourceError> for Error {
    fn from(err: crate::error::AddTagsToResourceError) -> Self {
        match err.kind {
            crate::error::AddTagsToResourceErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::AddTagsToResourceErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::AddTagsToResourceErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::AddTagsToResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateHapgError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateHapgError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateHapgError> for Error {
    fn from(err: crate::error::CreateHapgError) -> Self {
        match err.kind {
            crate::error::CreateHapgErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::CreateHapgErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::CreateHapgErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::CreateHapgErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateHsmError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateHsmError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateHsmError> for Error {
    fn from(err: crate::error::CreateHsmError) -> Self {
        match err.kind {
            crate::error::CreateHsmErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::CreateHsmErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::CreateHsmErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::CreateHsmErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateLunaClientError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateLunaClientError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateLunaClientError> for Error {
    fn from(err: crate::error::CreateLunaClientError) -> Self {
        match err.kind {
            crate::error::CreateLunaClientErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::CreateLunaClientErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::CreateLunaClientErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::CreateLunaClientErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteHapgError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteHapgError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteHapgError> for Error {
    fn from(err: crate::error::DeleteHapgError) -> Self {
        match err.kind {
            crate::error::DeleteHapgErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::DeleteHapgErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::DeleteHapgErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::DeleteHapgErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteHsmError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteHsmError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteHsmError> for Error {
    fn from(err: crate::error::DeleteHsmError) -> Self {
        match err.kind {
            crate::error::DeleteHsmErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::DeleteHsmErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::DeleteHsmErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::DeleteHsmErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteLunaClientError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteLunaClientError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteLunaClientError> for Error {
    fn from(err: crate::error::DeleteLunaClientError) -> Self {
        match err.kind {
            crate::error::DeleteLunaClientErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::DeleteLunaClientErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::DeleteLunaClientErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::DeleteLunaClientErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeHapgError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeHapgError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeHapgError> for Error {
    fn from(err: crate::error::DescribeHapgError) -> Self {
        match err.kind {
            crate::error::DescribeHapgErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::DescribeHapgErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::DescribeHapgErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::DescribeHapgErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeHsmError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeHsmError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeHsmError> for Error {
    fn from(err: crate::error::DescribeHsmError) -> Self {
        match err.kind {
            crate::error::DescribeHsmErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::DescribeHsmErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::DescribeHsmErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::DescribeHsmErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeLunaClientError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeLunaClientError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeLunaClientError> for Error {
    fn from(err: crate::error::DescribeLunaClientError) -> Self {
        match err.kind {
            crate::error::DescribeLunaClientErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::DescribeLunaClientErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::DescribeLunaClientErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::DescribeLunaClientErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetConfigError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetConfigError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetConfigError> for Error {
    fn from(err: crate::error::GetConfigError) -> Self {
        match err.kind {
            crate::error::GetConfigErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::GetConfigErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::GetConfigErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::GetConfigErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListAvailableZonesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListAvailableZonesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListAvailableZonesError> for Error {
    fn from(err: crate::error::ListAvailableZonesError) -> Self {
        match err.kind {
            crate::error::ListAvailableZonesErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::ListAvailableZonesErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::ListAvailableZonesErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::ListAvailableZonesErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListHapgsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListHapgsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListHapgsError> for Error {
    fn from(err: crate::error::ListHapgsError) -> Self {
        match err.kind {
            crate::error::ListHapgsErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::ListHapgsErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::ListHapgsErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::ListHapgsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListHsmsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListHsmsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListHsmsError> for Error {
    fn from(err: crate::error::ListHsmsError) -> Self {
        match err.kind {
            crate::error::ListHsmsErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::ListHsmsErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::ListHsmsErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::ListHsmsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListLunaClientsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListLunaClientsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListLunaClientsError> for Error {
    fn from(err: crate::error::ListLunaClientsError) -> Self {
        match err.kind {
            crate::error::ListLunaClientsErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::ListLunaClientsErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::ListLunaClientsErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::ListLunaClientsErrorKind::Unhandled(inner) => {
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
            crate::error::ListTagsForResourceErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ModifyHapgError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ModifyHapgError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ModifyHapgError> for Error {
    fn from(err: crate::error::ModifyHapgError) -> Self {
        match err.kind {
            crate::error::ModifyHapgErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::ModifyHapgErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::ModifyHapgErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::ModifyHapgErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ModifyHsmError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ModifyHsmError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ModifyHsmError> for Error {
    fn from(err: crate::error::ModifyHsmError) -> Self {
        match err.kind {
            crate::error::ModifyHsmErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::ModifyHsmErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::ModifyHsmErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::ModifyHsmErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ModifyLunaClientError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ModifyLunaClientError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ModifyLunaClientError> for Error {
    fn from(err: crate::error::ModifyLunaClientError) -> Self {
        match err.kind {
            crate::error::ModifyLunaClientErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::ModifyLunaClientErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RemoveTagsFromResourceError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RemoveTagsFromResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::RemoveTagsFromResourceError> for Error {
    fn from(err: crate::error::RemoveTagsFromResourceError) -> Self {
        match err.kind {
            crate::error::RemoveTagsFromResourceErrorKind::CloudHsmInternalException(inner) => {
                Error::CloudHsmInternalException(inner)
            }
            crate::error::RemoveTagsFromResourceErrorKind::CloudHsmServiceException(inner) => {
                Error::CloudHsmServiceException(inner)
            }
            crate::error::RemoveTagsFromResourceErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::RemoveTagsFromResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
