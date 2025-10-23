use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "make-appointment")]
pub struct MakeAppointmentSbContract {
    #[prost(string, tag = "1")]
    pub inventory_id: String,
    #[prost(string, tag = "2")]
    pub tenant_id: String,
    #[prost(string, tag = "3")]
    pub session_id: String,
    #[prost(string, tag = "4")]
    pub event_id: String,
    #[prost(string, tag = "5")]
    pub phone: String,
    #[prost(string, tag = "6")]
    pub email: String,
    #[prost(string, tag = "7")]
    pub date: String,
    #[prost(string, tag = "8")]
    pub time: String,
    #[prost(string, tag = "9")]
    pub time_zone: String,
    #[prost(string, tag = "10")]
    pub language: String,
    #[prost(string, tag = "11")]
    pub full_name: String,
}
