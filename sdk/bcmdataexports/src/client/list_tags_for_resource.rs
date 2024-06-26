// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTagsForResource`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::set_resource_arn):<br>required: **true**<br><p>The unique identifier for the resource.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of objects that are returned for the request.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::set_next_token):<br>required: **false**<br><p>The token to retrieve the next set of results.</p><br>
    /// - On success, responds with [`ListTagsForResourceOutput`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput) with field(s):
    ///   - [`resource_tags(Option<Vec::<ResourceTag>>)`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput::resource_tags): <p>An optional list of tags to associate with the specified export. Each tag consists of a key and a value, and each key must be unique for the resource.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput::next_token): <p>The token to retrieve the next set of results.</p>
    /// - On failure, responds with [`SdkError<ListTagsForResourceError>`](crate::operation::list_tags_for_resource::ListTagsForResourceError)
    pub fn list_tags_for_resource(&self) -> crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder {
        crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::new(self.handle.clone())
    }
}
