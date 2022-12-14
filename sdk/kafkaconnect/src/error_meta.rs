// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>HTTP Status Code 400: Bad request due to incorrect input. Correct your request and then retry it.</p>
    BadRequestException(crate::error::BadRequestException),
    /// <p>HTTP Status Code 409: Conflict. A resource with this name already exists. Retry your request with another name.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>HTTP Status Code 403: Access forbidden. Correct your credentials and then retry your request.</p>
    ForbiddenException(crate::error::ForbiddenException),
    /// <p>HTTP Status Code 500: Unexpected internal server error. Retrying your request might resolve the issue.</p>
    InternalServerErrorException(crate::error::InternalServerErrorException),
    /// <p>HTTP Status Code 404: Resource not found due to incorrect input. Correct your request and then retry it.</p>
    NotFoundException(crate::error::NotFoundException),
    /// <p>HTTP Status Code 503: Service Unavailable. Retrying your request in some time might resolve the issue.</p>
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    /// <p>HTTP Status Code 429: Limit exceeded. Resource limit reached.</p>
    TooManyRequestsException(crate::error::TooManyRequestsException),
    /// <p>HTTP Status Code 401: Unauthorized request. The provided credentials couldn't be validated.</p>
    UnauthorizedException(crate::error::UnauthorizedException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadRequestException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::ForbiddenException(inner) => inner.fmt(f),
            Error::InternalServerErrorException(inner) => inner.fmt(f),
            Error::NotFoundException(inner) => inner.fmt(f),
            Error::ServiceUnavailableException(inner) => inner.fmt(f),
            Error::TooManyRequestsException(inner) => inner.fmt(f),
            Error::UnauthorizedException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateConnectorError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateConnectorError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateConnectorError> for Error {
    fn from(err: crate::error::CreateConnectorError) -> Self {
        match err.kind {
            crate::error::CreateConnectorErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::CreateConnectorErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::CreateConnectorErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::CreateConnectorErrorKind::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::error::CreateConnectorErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::CreateConnectorErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::CreateConnectorErrorKind::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::error::CreateConnectorErrorKind::UnauthorizedException(inner) => {
                Error::UnauthorizedException(inner)
            }
            crate::error::CreateConnectorErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateCustomPluginError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateCustomPluginError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateCustomPluginError> for Error {
    fn from(err: crate::error::CreateCustomPluginError) -> Self {
        match err.kind {
            crate::error::CreateCustomPluginErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::CreateCustomPluginErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::CreateCustomPluginErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::CreateCustomPluginErrorKind::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::error::CreateCustomPluginErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::CreateCustomPluginErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::CreateCustomPluginErrorKind::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::error::CreateCustomPluginErrorKind::UnauthorizedException(inner) => {
                Error::UnauthorizedException(inner)
            }
            crate::error::CreateCustomPluginErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateWorkerConfigurationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateWorkerConfigurationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateWorkerConfigurationError> for Error {
    fn from(err: crate::error::CreateWorkerConfigurationError) -> Self {
        match err.kind {
            crate::error::CreateWorkerConfigurationErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::CreateWorkerConfigurationErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::CreateWorkerConfigurationErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::CreateWorkerConfigurationErrorKind::InternalServerErrorException(
                inner,
            ) => Error::InternalServerErrorException(inner),
            crate::error::CreateWorkerConfigurationErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::CreateWorkerConfigurationErrorKind::ServiceUnavailableException(
                inner,
            ) => Error::ServiceUnavailableException(inner),
            crate::error::CreateWorkerConfigurationErrorKind::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::error::CreateWorkerConfigurationErrorKind::UnauthorizedException(inner) => {
                Error::UnauthorizedException(inner)
            }
            crate::error::CreateWorkerConfigurationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteConnectorError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteConnectorError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteConnectorError> for Error {
    fn from(err: crate::error::DeleteConnectorError) -> Self {
        match err.kind {
            crate::error::DeleteConnectorErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::DeleteConnectorErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::DeleteConnectorErrorKind::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::error::DeleteConnectorErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::DeleteConnectorErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::DeleteConnectorErrorKind::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::error::DeleteConnectorErrorKind::UnauthorizedException(inner) => {
                Error::UnauthorizedException(inner)
            }
            crate::error::DeleteConnectorErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteCustomPluginError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteCustomPluginError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteCustomPluginError> for Error {
    fn from(err: crate::error::DeleteCustomPluginError) -> Self {
        match err.kind {
            crate::error::DeleteCustomPluginErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::DeleteCustomPluginErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::DeleteCustomPluginErrorKind::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::error::DeleteCustomPluginErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::DeleteCustomPluginErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::DeleteCustomPluginErrorKind::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::error::DeleteCustomPluginErrorKind::UnauthorizedException(inner) => {
                Error::UnauthorizedException(inner)
            }
            crate::error::DeleteCustomPluginErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeConnectorError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeConnectorError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeConnectorError> for Error {
    fn from(err: crate::error::DescribeConnectorError) -> Self {
        match err.kind {
            crate::error::DescribeConnectorErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::DescribeConnectorErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::DescribeConnectorErrorKind::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::error::DescribeConnectorErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::DescribeConnectorErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::DescribeConnectorErrorKind::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::error::DescribeConnectorErrorKind::UnauthorizedException(inner) => {
                Error::UnauthorizedException(inner)
            }
            crate::error::DescribeConnectorErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeCustomPluginError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeCustomPluginError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeCustomPluginError> for Error {
    fn from(err: crate::error::DescribeCustomPluginError) -> Self {
        match err.kind {
            crate::error::DescribeCustomPluginErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::DescribeCustomPluginErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::DescribeCustomPluginErrorKind::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::error::DescribeCustomPluginErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::DescribeCustomPluginErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::DescribeCustomPluginErrorKind::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::error::DescribeCustomPluginErrorKind::UnauthorizedException(inner) => {
                Error::UnauthorizedException(inner)
            }
            crate::error::DescribeCustomPluginErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeWorkerConfigurationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeWorkerConfigurationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeWorkerConfigurationError> for Error {
    fn from(err: crate::error::DescribeWorkerConfigurationError) -> Self {
        match err.kind {
            crate::error::DescribeWorkerConfigurationErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::DescribeWorkerConfigurationErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::DescribeWorkerConfigurationErrorKind::InternalServerErrorException(
                inner,
            ) => Error::InternalServerErrorException(inner),
            crate::error::DescribeWorkerConfigurationErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::DescribeWorkerConfigurationErrorKind::ServiceUnavailableException(
                inner,
            ) => Error::ServiceUnavailableException(inner),
            crate::error::DescribeWorkerConfigurationErrorKind::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::error::DescribeWorkerConfigurationErrorKind::UnauthorizedException(inner) => {
                Error::UnauthorizedException(inner)
            }
            crate::error::DescribeWorkerConfigurationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListConnectorsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListConnectorsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListConnectorsError> for Error {
    fn from(err: crate::error::ListConnectorsError) -> Self {
        match err.kind {
            crate::error::ListConnectorsErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::ListConnectorsErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::ListConnectorsErrorKind::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::error::ListConnectorsErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::ListConnectorsErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::ListConnectorsErrorKind::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::error::ListConnectorsErrorKind::UnauthorizedException(inner) => {
                Error::UnauthorizedException(inner)
            }
            crate::error::ListConnectorsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListCustomPluginsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListCustomPluginsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListCustomPluginsError> for Error {
    fn from(err: crate::error::ListCustomPluginsError) -> Self {
        match err.kind {
            crate::error::ListCustomPluginsErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::ListCustomPluginsErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::ListCustomPluginsErrorKind::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::error::ListCustomPluginsErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::ListCustomPluginsErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::ListCustomPluginsErrorKind::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::error::ListCustomPluginsErrorKind::UnauthorizedException(inner) => {
                Error::UnauthorizedException(inner)
            }
            crate::error::ListCustomPluginsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListWorkerConfigurationsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListWorkerConfigurationsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListWorkerConfigurationsError> for Error {
    fn from(err: crate::error::ListWorkerConfigurationsError) -> Self {
        match err.kind {
            crate::error::ListWorkerConfigurationsErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::ListWorkerConfigurationsErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::ListWorkerConfigurationsErrorKind::InternalServerErrorException(
                inner,
            ) => Error::InternalServerErrorException(inner),
            crate::error::ListWorkerConfigurationsErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::ListWorkerConfigurationsErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::ListWorkerConfigurationsErrorKind::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::error::ListWorkerConfigurationsErrorKind::UnauthorizedException(inner) => {
                Error::UnauthorizedException(inner)
            }
            crate::error::ListWorkerConfigurationsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateConnectorError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateConnectorError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateConnectorError> for Error {
    fn from(err: crate::error::UpdateConnectorError) -> Self {
        match err.kind {
            crate::error::UpdateConnectorErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::UpdateConnectorErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::UpdateConnectorErrorKind::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::error::UpdateConnectorErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::UpdateConnectorErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::UpdateConnectorErrorKind::TooManyRequestsException(inner) => {
                Error::TooManyRequestsException(inner)
            }
            crate::error::UpdateConnectorErrorKind::UnauthorizedException(inner) => {
                Error::UnauthorizedException(inner)
            }
            crate::error::UpdateConnectorErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
