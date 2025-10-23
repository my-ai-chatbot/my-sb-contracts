use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "make-appointment")]
pub struct ChatMessageSbContract {
    #[prost(string, tag = "1")]
    pub inventory_id: String,
    #[prost(string, tag = "2")]
    pub tenant_id: String,
    #[prost(string, tag = "3")]
    pub session_id: String,
    #[prost(string, tag = "4")]
    pub chat_id: String,
    #[prost(bool, tag = "5")]
    pub audio: bool,
    #[prost(bool, tag = "6")]
    pub client_to_server: bool,
    #[prost(string, tag = "7")]
    pub message: String,
    #[prost(message, tag = "8")]
    pub tool_call: Option<ToolCallSbContract>,
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
