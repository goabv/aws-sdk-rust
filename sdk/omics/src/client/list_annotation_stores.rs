// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListAnnotationStores`](crate::operation::list_annotation_stores::builders::ListAnnotationStoresFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_annotation_stores::builders::ListAnnotationStoresFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`ids(impl Into<String>)`](crate::operation::list_annotation_stores::builders::ListAnnotationStoresFluentBuilder::ids) / [`set_ids(Option<Vec::<String>>)`](crate::operation::list_annotation_stores::builders::ListAnnotationStoresFluentBuilder::set_ids):<br>required: **false**<br><p>IDs of stores to list.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_annotation_stores::builders::ListAnnotationStoresFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_annotation_stores::builders::ListAnnotationStoresFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of stores to return in one page of results.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_annotation_stores::builders::ListAnnotationStoresFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_annotation_stores::builders::ListAnnotationStoresFluentBuilder::set_next_token):<br>required: **false**<br><p>Specify the pagination token from a previous request to retrieve the next page of results.</p><br>
    ///   - [`filter(ListAnnotationStoresFilter)`](crate::operation::list_annotation_stores::builders::ListAnnotationStoresFluentBuilder::filter) / [`set_filter(Option<ListAnnotationStoresFilter>)`](crate::operation::list_annotation_stores::builders::ListAnnotationStoresFluentBuilder::set_filter):<br>required: **false**<br><p>A filter to apply to the list.</p><br>
    /// - On success, responds with [`ListAnnotationStoresOutput`](crate::operation::list_annotation_stores::ListAnnotationStoresOutput) with field(s):
    ///   - [`annotation_stores(Option<Vec::<AnnotationStoreItem>>)`](crate::operation::list_annotation_stores::ListAnnotationStoresOutput::annotation_stores): <p>A list of stores.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_annotation_stores::ListAnnotationStoresOutput::next_token): <p>A pagination token that's included if more results are available.</p>
    /// - On failure, responds with [`SdkError<ListAnnotationStoresError>`](crate::operation::list_annotation_stores::ListAnnotationStoresError)
    pub fn list_annotation_stores(&self) -> crate::operation::list_annotation_stores::builders::ListAnnotationStoresFluentBuilder {
        crate::operation::list_annotation_stores::builders::ListAnnotationStoresFluentBuilder::new(self.handle.clone())
    }
}
