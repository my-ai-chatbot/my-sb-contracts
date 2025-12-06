use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "chat-message")]
pub struct ChatMessageSbContract {
    #[prost(string, tag = "1")]
    pub inventory_id: String,
    #[prost(string, tag = "2")]
    pub tenant_id: String,
    #[prost(string, tag = "3")]
    pub session_id: String,
    #[prost(string, tag = "4")]
    pub request_id: String,
    #[prost(string, tag = "5")]
    pub conversation_id: String,
    #[prost(uint64, tag = "6")]
    pub total_messages: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolCallSbContract {
    #[prost(string, tag = "1")]
    pub tool_call: String,
    #[prost(string, tag = "2")]
    pub input_data: String,
    #[prost(string, tag = "3")]
    pub result: String,
}
