use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "stt-end-conversation")]
pub struct SessionConnectDisconnect {
    #[prost(string, tag = "1")]
    pub session_id: String,
    #[prost(string, tag = "2")]
    pub ip: String,
    #[prost(string, tag = "3")]
    pub country: String,
    #[prost(string, tag = "4")]
    pub origin: String,
}
