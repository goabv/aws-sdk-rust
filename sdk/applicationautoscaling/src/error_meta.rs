// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdateException(crate::error::ConcurrentUpdateException),
    /// <p>Failed access to resources caused an exception. This exception is thrown when Application Auto Scaling is unable to retrieve the alarms associated with a scaling policy due to a client error, for example, if the role ARN specified for a scalable target does not have permission to call the CloudWatch <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeAlarms.html">DescribeAlarms</a> on your behalf.</p>
    FailedResourceAccessException(crate::error::FailedResourceAccessException),
    /// <p>The service encountered an internal error.</p>
    InternalServiceException(crate::error::InternalServiceException),
    /// <p>The next token supplied was invalid.</p>
    InvalidNextTokenException(crate::error::InvalidNextTokenException),
    /// <p>A per-account resource limit is exceeded. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-limits.html">Application Auto Scaling service quotas</a>.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>The specified object could not be found. For any operation that depends on the existence of a scalable target, this exception is thrown if the scalable target with the specified service namespace, resource ID, and scalable dimension does not exist. For any operation that deletes or deregisters a resource, this exception is thrown if the resource cannot be found.</p>
    ObjectNotFoundException(crate::error::ObjectNotFoundException),
    /// <p>An exception was thrown for a validation issue. Review the available parameters for the API request.</p>
    ValidationException(crate::error::ValidationException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConcurrentUpdateException(inner) => inner.fmt(f),
            Error::FailedResourceAccessException(inner) => inner.fmt(f),
            Error::InternalServiceException(inner) => inner.fmt(f),
            Error::InvalidNextTokenException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ObjectNotFoundException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteScalingPolicyError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteScalingPolicyError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteScalingPolicyError> for Error {
    fn from(err: crate::error::DeleteScalingPolicyError) -> Self {
        match err.kind {
            crate::error::DeleteScalingPolicyErrorKind::ConcurrentUpdateException(inner) => {
                Error::ConcurrentUpdateException(inner)
            }
            crate::error::DeleteScalingPolicyErrorKind::InternalServiceException(inner) => {
                Error::InternalServiceException(inner)
            }
            crate::error::DeleteScalingPolicyErrorKind::ObjectNotFoundException(inner) => {
                Error::ObjectNotFoundException(inner)
            }
            crate::error::DeleteScalingPolicyErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DeleteScalingPolicyErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteScheduledActionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteScheduledActionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteScheduledActionError> for Error {
    fn from(err: crate::error::DeleteScheduledActionError) -> Self {
        match err.kind {
            crate::error::DeleteScheduledActionErrorKind::ConcurrentUpdateException(inner) => {
                Error::ConcurrentUpdateException(inner)
            }
            crate::error::DeleteScheduledActionErrorKind::InternalServiceException(inner) => {
                Error::InternalServiceException(inner)
            }
            crate::error::DeleteScheduledActionErrorKind::ObjectNotFoundException(inner) => {
                Error::ObjectNotFoundException(inner)
            }
            crate::error::DeleteScheduledActionErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DeleteScheduledActionErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeregisterScalableTargetError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeregisterScalableTargetError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeregisterScalableTargetError> for Error {
    fn from(err: crate::error::DeregisterScalableTargetError) -> Self {
        match err.kind {
            crate::error::DeregisterScalableTargetErrorKind::ConcurrentUpdateException(inner) => {
                Error::ConcurrentUpdateException(inner)
            }
            crate::error::DeregisterScalableTargetErrorKind::InternalServiceException(inner) => {
                Error::InternalServiceException(inner)
            }
            crate::error::DeregisterScalableTargetErrorKind::ObjectNotFoundException(inner) => {
                Error::ObjectNotFoundException(inner)
            }
            crate::error::DeregisterScalableTargetErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DeregisterScalableTargetErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeScalableTargetsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeScalableTargetsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeScalableTargetsError> for Error {
    fn from(err: crate::error::DescribeScalableTargetsError) -> Self {
        match err.kind {
            crate::error::DescribeScalableTargetsErrorKind::ConcurrentUpdateException(inner) => {
                Error::ConcurrentUpdateException(inner)
            }
            crate::error::DescribeScalableTargetsErrorKind::InternalServiceException(inner) => {
                Error::InternalServiceException(inner)
            }
            crate::error::DescribeScalableTargetsErrorKind::InvalidNextTokenException(inner) => {
                Error::InvalidNextTokenException(inner)
            }
            crate::error::DescribeScalableTargetsErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeScalableTargetsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeScalingActivitiesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeScalingActivitiesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeScalingActivitiesError> for Error {
    fn from(err: crate::error::DescribeScalingActivitiesError) -> Self {
        match err.kind {
            crate::error::DescribeScalingActivitiesErrorKind::ConcurrentUpdateException(inner) => {
                Error::ConcurrentUpdateException(inner)
            }
            crate::error::DescribeScalingActivitiesErrorKind::InternalServiceException(inner) => {
                Error::InternalServiceException(inner)
            }
            crate::error::DescribeScalingActivitiesErrorKind::InvalidNextTokenException(inner) => {
                Error::InvalidNextTokenException(inner)
            }
            crate::error::DescribeScalingActivitiesErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeScalingActivitiesErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeScalingPoliciesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeScalingPoliciesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeScalingPoliciesError> for Error {
    fn from(err: crate::error::DescribeScalingPoliciesError) -> Self {
        match err.kind {
            crate::error::DescribeScalingPoliciesErrorKind::ConcurrentUpdateException(inner) => {
                Error::ConcurrentUpdateException(inner)
            }
            crate::error::DescribeScalingPoliciesErrorKind::FailedResourceAccessException(
                inner,
            ) => Error::FailedResourceAccessException(inner),
            crate::error::DescribeScalingPoliciesErrorKind::InternalServiceException(inner) => {
                Error::InternalServiceException(inner)
            }
            crate::error::DescribeScalingPoliciesErrorKind::InvalidNextTokenException(inner) => {
                Error::InvalidNextTokenException(inner)
            }
            crate::error::DescribeScalingPoliciesErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeScalingPoliciesErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeScheduledActionsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeScheduledActionsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeScheduledActionsError> for Error {
    fn from(err: crate::error::DescribeScheduledActionsError) -> Self {
        match err.kind {
            crate::error::DescribeScheduledActionsErrorKind::ConcurrentUpdateException(inner) => {
                Error::ConcurrentUpdateException(inner)
            }
            crate::error::DescribeScheduledActionsErrorKind::InternalServiceException(inner) => {
                Error::InternalServiceException(inner)
            }
            crate::error::DescribeScheduledActionsErrorKind::InvalidNextTokenException(inner) => {
                Error::InvalidNextTokenException(inner)
            }
            crate::error::DescribeScheduledActionsErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeScheduledActionsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutScalingPolicyError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::PutScalingPolicyError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::PutScalingPolicyError> for Error {
    fn from(err: crate::error::PutScalingPolicyError) -> Self {
        match err.kind {
            crate::error::PutScalingPolicyErrorKind::ConcurrentUpdateException(inner) => {
                Error::ConcurrentUpdateException(inner)
            }
            crate::error::PutScalingPolicyErrorKind::FailedResourceAccessException(inner) => {
                Error::FailedResourceAccessException(inner)
            }
            crate::error::PutScalingPolicyErrorKind::InternalServiceException(inner) => {
                Error::InternalServiceException(inner)
            }
            crate::error::PutScalingPolicyErrorKind::LimitExceededException(inner) => {
                Error::LimitExceededException(inner)
            }
            crate::error::PutScalingPolicyErrorKind::ObjectNotFoundException(inner) => {
                Error::ObjectNotFoundException(inner)
            }
            crate::error::PutScalingPolicyErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::PutScalingPolicyErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutScheduledActionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::PutScheduledActionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::PutScheduledActionError> for Error {
    fn from(err: crate::error::PutScheduledActionError) -> Self {
        match err.kind {
            crate::error::PutScheduledActionErrorKind::ConcurrentUpdateException(inner) => {
                Error::ConcurrentUpdateException(inner)
            }
            crate::error::PutScheduledActionErrorKind::InternalServiceException(inner) => {
                Error::InternalServiceException(inner)
            }
            crate::error::PutScheduledActionErrorKind::LimitExceededException(inner) => {
                Error::LimitExceededException(inner)
            }
            crate::error::PutScheduledActionErrorKind::ObjectNotFoundException(inner) => {
                Error::ObjectNotFoundException(inner)
            }
            crate::error::PutScheduledActionErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::PutScheduledActionErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RegisterScalableTargetError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RegisterScalableTargetError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::RegisterScalableTargetError> for Error {
    fn from(err: crate::error::RegisterScalableTargetError) -> Self {
        match err.kind {
            crate::error::RegisterScalableTargetErrorKind::ConcurrentUpdateException(inner) => {
                Error::ConcurrentUpdateException(inner)
            }
            crate::error::RegisterScalableTargetErrorKind::InternalServiceException(inner) => {
                Error::InternalServiceException(inner)
            }
            crate::error::RegisterScalableTargetErrorKind::LimitExceededException(inner) => {
                Error::LimitExceededException(inner)
            }
            crate::error::RegisterScalableTargetErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::RegisterScalableTargetErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
