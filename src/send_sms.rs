use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "exec-crypto-deposit")]
pub struct SendSmsMySbContract {
    #[prost(string, tag = "1")]
    pub inventory_id: String,
    #[prost(string, tag = "2")]
    pub tenant_id: String,
    #[prost(string, tag = "3")]
    pub phone: String,
    #[prost(string, tag = "4")]
    pub message: String,
}
