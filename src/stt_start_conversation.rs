use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "stt-start-conversation")]
pub struct SttStartConversationMySbContract {
    #[prost(string, tag = "1")]
    pub session_id: String,
    #[prost(string, tag = "2")]
    pub lang_id: String,
    #[prost(string, tag = "3")]
    pub request_id: String,
    #[prost(string, tag = "4")]
    pub conversation_id: String,
    #[prost(string, tag = "5")]
    pub file_storage_name: String,
    #[prost(int64, tag = "6")]
    pub request_telemetry_id: i64,
}
