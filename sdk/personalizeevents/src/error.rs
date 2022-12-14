// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Could not find the specified resource.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ResourceNotFoundException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ResourceNotFoundException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ResourceNotFoundException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceNotFoundException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for ResourceNotFoundException {}
/// See [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
pub mod resource_not_found_exception {

    /// A builder for [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
        pub fn build(self) -> crate::error::ResourceNotFoundException {
            crate::error::ResourceNotFoundException {
                message: self.message,
            }
        }
    }
}
impl ResourceNotFoundException {
    /// Creates a new builder-style object to manufacture [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
    pub fn builder() -> crate::error::resource_not_found_exception::Builder {
        crate::error::resource_not_found_exception::Builder::default()
    }
}

/// <p>The specified resource is in use.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ResourceInUseException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ResourceInUseException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ResourceInUseException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ResourceInUseException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ResourceInUseException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceInUseException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for ResourceInUseException {}
/// See [`ResourceInUseException`](crate::error::ResourceInUseException).
pub mod resource_in_use_exception {

    /// A builder for [`ResourceInUseException`](crate::error::ResourceInUseException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ResourceInUseException`](crate::error::ResourceInUseException).
        pub fn build(self) -> crate::error::ResourceInUseException {
            crate::error::ResourceInUseException {
                message: self.message,
            }
        }
    }
}
impl ResourceInUseException {
    /// Creates a new builder-style object to manufacture [`ResourceInUseException`](crate::error::ResourceInUseException).
    pub fn builder() -> crate::error::resource_in_use_exception::Builder {
        crate::error::resource_in_use_exception::Builder::default()
    }
}

/// <p>Provide a valid value for the field or parameter.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvalidInputException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InvalidInputException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvalidInputException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InvalidInputException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidInputException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidInputException")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for InvalidInputException {}
/// See [`InvalidInputException`](crate::error::InvalidInputException).
pub mod invalid_input_exception {

    /// A builder for [`InvalidInputException`](crate::error::InvalidInputException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InvalidInputException`](crate::error::InvalidInputException).
        pub fn build(self) -> crate::error::InvalidInputException {
            crate::error::InvalidInputException {
                message: self.message,
            }
        }
    }
}
impl InvalidInputException {
    /// Creates a new builder-style object to manufacture [`InvalidInputException`](crate::error::InvalidInputException).
    pub fn builder() -> crate::error::invalid_input_exception::Builder {
        crate::error::invalid_input_exception::Builder::default()
    }
}

/// Error type for the `PutEvents` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct PutEventsError {
    /// Kind of error that occurred.
    pub kind: PutEventsErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for PutEventsError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: PutEventsErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `PutEvents` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum PutEventsErrorKind {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInputException(crate::error::InvalidInputException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for PutEventsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            PutEventsErrorKind::InvalidInputException(_inner) => _inner.fmt(f),
            PutEventsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for PutEventsError {
    fn code(&self) -> Option<&str> {
        PutEventsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl PutEventsError {
    /// Creates a new `PutEventsError`.
    pub fn new(kind: PutEventsErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `PutEventsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: PutEventsErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
            meta: Default::default(),
        }
    }

    /// Creates the `PutEventsError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: PutEventsErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `PutEventsErrorKind::InvalidInputException`.
    pub fn is_invalid_input_exception(&self) -> bool {
        matches!(&self.kind, PutEventsErrorKind::InvalidInputException(_))
    }
}
impl std::error::Error for PutEventsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            PutEventsErrorKind::InvalidInputException(_inner) => Some(_inner),
            PutEventsErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `PutItems` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct PutItemsError {
    /// Kind of error that occurred.
    pub kind: PutItemsErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for PutItemsError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: PutItemsErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `PutItems` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum PutItemsErrorKind {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInputException(crate::error::InvalidInputException),
    /// <p>The specified resource is in use.</p>
    ResourceInUseException(crate::error::ResourceInUseException),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for PutItemsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            PutItemsErrorKind::InvalidInputException(_inner) => _inner.fmt(f),
            PutItemsErrorKind::ResourceInUseException(_inner) => _inner.fmt(f),
            PutItemsErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            PutItemsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for PutItemsError {
    fn code(&self) -> Option<&str> {
        PutItemsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl PutItemsError {
    /// Creates a new `PutItemsError`.
    pub fn new(kind: PutItemsErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `PutItemsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: PutItemsErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
            meta: Default::default(),
        }
    }

    /// Creates the `PutItemsError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: PutItemsErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `PutItemsErrorKind::InvalidInputException`.
    pub fn is_invalid_input_exception(&self) -> bool {
        matches!(&self.kind, PutItemsErrorKind::InvalidInputException(_))
    }
    /// Returns `true` if the error kind is `PutItemsErrorKind::ResourceInUseException`.
    pub fn is_resource_in_use_exception(&self) -> bool {
        matches!(&self.kind, PutItemsErrorKind::ResourceInUseException(_))
    }
    /// Returns `true` if the error kind is `PutItemsErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(&self.kind, PutItemsErrorKind::ResourceNotFoundException(_))
    }
}
impl std::error::Error for PutItemsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            PutItemsErrorKind::InvalidInputException(_inner) => Some(_inner),
            PutItemsErrorKind::ResourceInUseException(_inner) => Some(_inner),
            PutItemsErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            PutItemsErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `PutUsers` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct PutUsersError {
    /// Kind of error that occurred.
    pub kind: PutUsersErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for PutUsersError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: PutUsersErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `PutUsers` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum PutUsersErrorKind {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInputException(crate::error::InvalidInputException),
    /// <p>The specified resource is in use.</p>
    ResourceInUseException(crate::error::ResourceInUseException),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for PutUsersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            PutUsersErrorKind::InvalidInputException(_inner) => _inner.fmt(f),
            PutUsersErrorKind::ResourceInUseException(_inner) => _inner.fmt(f),
            PutUsersErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            PutUsersErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for PutUsersError {
    fn code(&self) -> Option<&str> {
        PutUsersError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl PutUsersError {
    /// Creates a new `PutUsersError`.
    pub fn new(kind: PutUsersErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `PutUsersError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: PutUsersErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
            meta: Default::default(),
        }
    }

    /// Creates the `PutUsersError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: PutUsersErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `PutUsersErrorKind::InvalidInputException`.
    pub fn is_invalid_input_exception(&self) -> bool {
        matches!(&self.kind, PutUsersErrorKind::InvalidInputException(_))
    }
    /// Returns `true` if the error kind is `PutUsersErrorKind::ResourceInUseException`.
    pub fn is_resource_in_use_exception(&self) -> bool {
        matches!(&self.kind, PutUsersErrorKind::ResourceInUseException(_))
    }
    /// Returns `true` if the error kind is `PutUsersErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(&self.kind, PutUsersErrorKind::ResourceNotFoundException(_))
    }
}
impl std::error::Error for PutUsersError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            PutUsersErrorKind::InvalidInputException(_inner) => Some(_inner),
            PutUsersErrorKind::ResourceInUseException(_inner) => Some(_inner),
            PutUsersErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            PutUsersErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

///
/// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code)
///
/// Call [`Error::source`](std::error::Error::source) for more details about the underlying cause.
///
#[derive(Debug)]
pub struct Unhandled {
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
}
impl Unhandled {
    pub(crate) fn new(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self { source }
    }
}
impl std::fmt::Display for Unhandled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "unhandled error")
    }
}
impl std::error::Error for Unhandled {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref() as _)
    }
}
