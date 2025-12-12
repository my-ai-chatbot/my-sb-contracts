use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "stt-transcribed-text")]
pub struct SttTranscribedTextMySbContract {
    #[prost(string, tag = "1")]
    pub session_id: String,
    #[prost(string, tag = "2")]
    pub text: String,
    #[prost(string, tag = "3")]
    pub chat_lang_id: String,
    #[prost(string, tag = "4")]
    pub stt_type: String,
    #[prost(string, tag = "5")]
    pub file: String,
    #[prost(string, tag = "6")]
    pub request_id: String,
    #[prost(string, tag = "7")]
    pub conversation_id: String,
    #[prost(string, tag = "8")]
    pub text_lang_id: String,
    #[prost(int64, tag = "9")]
    pub silence_micros: i64,
    #[prost(int64, tag = "10")]
    pub stt_duration: i64,
    #[prost(int64, tag = "11")]
    pub request_telemetry_id: i64,
}
