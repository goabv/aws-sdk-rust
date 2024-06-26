// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateOrganization`](crate::operation::create_organization::builders::CreateOrganizationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_id(impl Into<String>)`](crate::operation::create_organization::builders::CreateOrganizationFluentBuilder::directory_id) / [`set_directory_id(Option<String>)`](crate::operation::create_organization::builders::CreateOrganizationFluentBuilder::set_directory_id):<br>required: **false**<br><p>The AWS Directory Service directory ID.</p><br>
    ///   - [`alias(impl Into<String>)`](crate::operation::create_organization::builders::CreateOrganizationFluentBuilder::alias) / [`set_alias(Option<String>)`](crate::operation::create_organization::builders::CreateOrganizationFluentBuilder::set_alias):<br>required: **true**<br><p>The organization alias.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_organization::builders::CreateOrganizationFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_organization::builders::CreateOrganizationFluentBuilder::set_client_token):<br>required: **false**<br><p>The idempotency token associated with the request.</p><br>
    ///   - [`domains(Domain)`](crate::operation::create_organization::builders::CreateOrganizationFluentBuilder::domains) / [`set_domains(Option<Vec::<Domain>>)`](crate::operation::create_organization::builders::CreateOrganizationFluentBuilder::set_domains):<br>required: **false**<br><p>The email domains to associate with the organization.</p><br>
    ///   - [`kms_key_arn(impl Into<String>)`](crate::operation::create_organization::builders::CreateOrganizationFluentBuilder::kms_key_arn) / [`set_kms_key_arn(Option<String>)`](crate::operation::create_organization::builders::CreateOrganizationFluentBuilder::set_kms_key_arn):<br>required: **false**<br><p>The Amazon Resource Name (ARN) of a customer managed key from AWS KMS.</p><br>
    ///   - [`enable_interoperability(bool)`](crate::operation::create_organization::builders::CreateOrganizationFluentBuilder::enable_interoperability) / [`set_enable_interoperability(Option<bool>)`](crate::operation::create_organization::builders::CreateOrganizationFluentBuilder::set_enable_interoperability):<br>required: **false**<br><p>When <code>true</code>, allows organization interoperability between WorkMail and Microsoft Exchange. If <code>true</code>, you must include a AD Connector directory ID in the request.</p><br>
    /// - On success, responds with [`CreateOrganizationOutput`](crate::operation::create_organization::CreateOrganizationOutput) with field(s):
    ///   - [`organization_id(Option<String>)`](crate::operation::create_organization::CreateOrganizationOutput::organization_id): <p>The organization ID.</p>
    /// - On failure, responds with [`SdkError<CreateOrganizationError>`](crate::operation::create_organization::CreateOrganizationError)
    pub fn create_organization(&self) -> crate::operation::create_organization::builders::CreateOrganizationFluentBuilder {
        crate::operation::create_organization::builders::CreateOrganizationFluentBuilder::new(self.handle.clone())
    }
}
