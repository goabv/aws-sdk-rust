// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetEventConfigurationByResourceTypes`](crate::operation::get_event_configuration_by_resource_types::builders::GetEventConfigurationByResourceTypesFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::get_event_configuration_by_resource_types::builders::GetEventConfigurationByResourceTypesFluentBuilder::send) it.
    /// - On success, responds with [`GetEventConfigurationByResourceTypesOutput`](crate::operation::get_event_configuration_by_resource_types::GetEventConfigurationByResourceTypesOutput) with field(s):
    ///   - [`device_registration_state(Option<DeviceRegistrationStateResourceTypeEventConfiguration>)`](crate::operation::get_event_configuration_by_resource_types::GetEventConfigurationByResourceTypesOutput::device_registration_state): <p>Resource type event configuration for the device registration state event.</p>
    ///   - [`proximity(Option<ProximityResourceTypeEventConfiguration>)`](crate::operation::get_event_configuration_by_resource_types::GetEventConfigurationByResourceTypesOutput::proximity): <p>Resource type event configuration for the proximity event.</p>
    ///   - [`join(Option<JoinResourceTypeEventConfiguration>)`](crate::operation::get_event_configuration_by_resource_types::GetEventConfigurationByResourceTypesOutput::join): <p>Resource type event configuration for the join event.</p>
    ///   - [`connection_status(Option<ConnectionStatusResourceTypeEventConfiguration>)`](crate::operation::get_event_configuration_by_resource_types::GetEventConfigurationByResourceTypesOutput::connection_status): <p>Resource type event configuration for the connection status event.</p>
    ///   - [`message_delivery_status(Option<MessageDeliveryStatusResourceTypeEventConfiguration>)`](crate::operation::get_event_configuration_by_resource_types::GetEventConfigurationByResourceTypesOutput::message_delivery_status): <p>Resource type event configuration object for the message delivery status event.</p>
    /// - On failure, responds with [`SdkError<GetEventConfigurationByResourceTypesError>`](crate::operation::get_event_configuration_by_resource_types::GetEventConfigurationByResourceTypesError)
    pub fn get_event_configuration_by_resource_types(
        &self,
    ) -> crate::operation::get_event_configuration_by_resource_types::builders::GetEventConfigurationByResourceTypesFluentBuilder {
        crate::operation::get_event_configuration_by_resource_types::builders::GetEventConfigurationByResourceTypesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
