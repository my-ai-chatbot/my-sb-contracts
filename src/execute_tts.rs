use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "execute-tts")]
pub struct ExecuteTtsMySbContract {
    #[prost(string, tag = "1")]
    pub session_id: String,
    #[prost(string, tag = "2")]
    pub request_id: String,
    #[prost(string, tag = "3")]
    pub conversation_id: String,
    #[prost(string, tag = "4")]
    pub tenant_id: String,
    #[prost(string, tag = "5")]
    pub file_name: String,
    #[prost(string, tag = "6")]
    pub text_lang_id: String,
    #[prost(string, tag = "7")]
    pub session_lang_id: String,
}
