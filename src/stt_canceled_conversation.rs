use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "stt-canceled-conversation")]
pub struct SttEndConversationMySbContract {
    #[prost(string, tag = "1")]
    pub session_id: String,
    #[prost(string, tag = "2")]
    pub request_id: String,
    #[prost(string, tag = "3")]
    pub conversation_id: String,
}
