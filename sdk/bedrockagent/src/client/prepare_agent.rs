// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PrepareAgent`](crate::operation::prepare_agent::builders::PrepareAgentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`agent_id(impl Into<String>)`](crate::operation::prepare_agent::builders::PrepareAgentFluentBuilder::agent_id) / [`set_agent_id(Option<String>)`](crate::operation::prepare_agent::builders::PrepareAgentFluentBuilder::set_agent_id):<br>required: **true**<br><p>The unique identifier of the agent for which to create a <code>DRAFT</code> version.</p><br>
    /// - On success, responds with [`PrepareAgentOutput`](crate::operation::prepare_agent::PrepareAgentOutput) with field(s):
    ///   - [`agent_id(String)`](crate::operation::prepare_agent::PrepareAgentOutput::agent_id): <p>The unique identifier of the agent for which the <code>DRAFT</code> version was created.</p>
    ///   - [`agent_status(AgentStatus)`](crate::operation::prepare_agent::PrepareAgentOutput::agent_status): <p>The status of the <code>DRAFT</code> version and whether it is ready for use.</p>
    ///   - [`agent_version(String)`](crate::operation::prepare_agent::PrepareAgentOutput::agent_version): <p>The version of the agent.</p>
    ///   - [`prepared_at(DateTime)`](crate::operation::prepare_agent::PrepareAgentOutput::prepared_at): <p>The time at which the <code>DRAFT</code> version of the agent was last prepared.</p>
    /// - On failure, responds with [`SdkError<PrepareAgentError>`](crate::operation::prepare_agent::PrepareAgentError)
    pub fn prepare_agent(&self) -> crate::operation::prepare_agent::builders::PrepareAgentFluentBuilder {
        crate::operation::prepare_agent::builders::PrepareAgentFluentBuilder::new(self.handle.clone())
    }
}
