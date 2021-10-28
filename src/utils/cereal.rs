use rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use strum_macros::Display as StrumDisplay;

#[derive(Eq, Hash, PartialEq, PartialOrd, Ord, Debug, Serialize, Deserialize, Clone, Copy, StrumDisplay)]
pub enum EncodingType {
    Json,
    Cbor,
    Bson,
    MsgPack,
    FlexBuffer,
}

impl EncodingType {
    pub fn all() -> [EncodingType; 5] {
        [EncodingType::Bson, EncodingType::Cbor, EncodingType::FlexBuffer, EncodingType::Json, EncodingType::MsgPack]
    }

    pub fn get_file_ext(&self) -> &'static str {
        match self {
            EncodingType::Json => "json",
            EncodingType::Cbor => "cbor",
            EncodingType::Bson => "bson",
            EncodingType::MsgPack => "msgpack",
            EncodingType::FlexBuffer => "flex",
        }
    }
}

pub const GLOBAL_ENCODING: EncodingType = EncodingType::Json;

pub struct Cereal;

impl Cereal {
    pub fn serialize<TData: Serialize>(data: &TData) -> Result<Vec<u8>, String> {
        match GLOBAL_ENCODING {
            EncodingType::Json => Cereal::serialize_json(data).map_err(|e| e.to_string()),
            EncodingType::Cbor => Cereal::serialize_cbor(data).map_err(|e| e.to_string()),
            EncodingType::MsgPack => Cereal::serialize_msgpack(data).map_err(|e| e.to_string()),
            EncodingType::FlexBuffer => Cereal::serialize_flex(data).map_err(|e| e.to_string()),
            EncodingType::Bson => Cereal::serialize_bson(data).map_err(|e| e.to_string()),
        }
    }

    pub fn deserialize<TOutput: DeserializeOwned>(bytes: &Vec<u8>) -> Result<TOutput, String> {
        match GLOBAL_ENCODING {
            EncodingType::Json => Cereal::deserialize_json(bytes).map_err(|e| e.to_string()),
            EncodingType::Cbor => Cereal::deserialize_cbor(bytes).map_err(|e| e.to_string()),
            EncodingType::MsgPack => Cereal::deserialize_msgpack(bytes).map_err(|e| e.to_string()),
            EncodingType::FlexBuffer => Cereal::deserialize_flex(bytes).map_err(|e| e.to_string()),
            EncodingType::Bson => Cereal::deserialize_bson(bytes).map_err(|e| e.to_string()),
        }
    }

    pub fn serialize_with<TData: Serialize>(encoding: EncodingType, data: &TData) -> Result<Vec<u8>, String> {
        match encoding {
            EncodingType::Json => Cereal::serialize_json(data).map_err(|e| e.to_string()),
            EncodingType::Cbor => Cereal::serialize_cbor(data).map_err(|e| e.to_string()),
            EncodingType::MsgPack => Cereal::serialize_msgpack(data).map_err(|e| e.to_string()),
            EncodingType::FlexBuffer => Cereal::serialize_flex(data).map_err(|e| e.to_string()),
            EncodingType::Bson => Cereal::serialize_bson(data).map_err(|e| e.to_string()),
        }
    }

    pub fn deserialize_with<TOutput: DeserializeOwned>(encoding: EncodingType, bytes: &Vec<u8>) -> Result<TOutput, String> {
        match encoding {
            EncodingType::Json => Cereal::deserialize_json(bytes).map_err(|e| e.to_string()),
            EncodingType::Cbor => Cereal::deserialize_cbor(bytes).map_err(|e| e.to_string()),
            EncodingType::MsgPack => Cereal::deserialize_msgpack(bytes).map_err(|e| e.to_string()),
            EncodingType::FlexBuffer => Cereal::deserialize_flex(bytes).map_err(|e| e.to_string()),
            EncodingType::Bson => Cereal::deserialize_bson(bytes).map_err(|e| e.to_string()),
        }
    }

    pub fn serialize_json<TData: Serialize>(data: &TData) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(data)
    }

    pub fn deserialize_json<TOutput: DeserializeOwned>(
        bytes: &Vec<u8>,
    ) -> Result<TOutput, serde_json::Error> {
        serde_json::from_slice(bytes)
    }

    pub fn serialize_msgpack<TData: Serialize>(
        data: &TData,
    ) -> Result<Vec<u8>, rmps::encode::Error> {
        rmps::to_vec(data)
    }

    pub fn deserialize_msgpack<TOutput: DeserializeOwned>(
        bytes: &Vec<u8>,
    ) -> Result<TOutput, rmps::decode::Error> {
        rmps::from_read_ref(bytes)
    }

    pub fn serialize_flex<TData: Serialize>(
        data: &TData,
    ) -> Result<Vec<u8>, flexbuffers::SerializationError> {
        flexbuffers::to_vec(data)
    }

    pub fn deserialize_flex<TOutput: DeserializeOwned>(
        bytes: &Vec<u8>,
    ) -> Result<TOutput, flexbuffers::DeserializationError> {
        flexbuffers::from_slice(bytes)
    }

    pub fn serialize_bson<TData: Serialize>(
        data: &TData,
    ) -> Result<Vec<u8>, bson::ser::Error> {
        bson::to_vec(data)
    }

    pub fn deserialize_bson<TOutput: DeserializeOwned>(
        bytes: &Vec<u8>,
    ) -> Result<TOutput, bson::de::Error> {
        bson::from_slice(bytes)
    }

    pub fn serialize_cbor<TData: Serialize>(
        data: &TData,
    ) -> Result<Vec<u8>, serde_cbor::Error> {
        serde_cbor::to_vec(data)
    }

    pub fn deserialize_cbor<TOutput: DeserializeOwned>(
        bytes: &Vec<u8>,
    ) -> Result<TOutput, serde_cbor::Error> {
        serde_cbor::from_slice(bytes)
    }

    // pub fn deserialize_with<'Caller, TOutput: Deserialize<'Caller>>(encoding: EncodingType, bytes: &'Caller Vec<u8>) -> Result<TOutput, String> {
    //     match encoding {
    //         EncodingType::Json => Cereal::deserialize_json(bytes).map_err(|e| e.to_string()),
    //         EncodingType::Cbor => Cereal::deserialize_cbor(bytes).map_err(|e| e.to_string()),
    //         EncodingType::MsgPack => Cereal::deserialize_msgpack(bytes).map_err(|e| e.to_string()),
    //         EncodingType::FlexBuffer => Cereal::deserialize_flex(bytes).map_err(|e| e.to_string()),
    //         EncodingType::Bson => Cereal::deserialize_bson(bytes).map_err(|e| e.to_string()),
    //     }
    // }

    // pub fn serialize_json<TData: Serialize>(data: &TData) -> Result<Vec<u8>, serde_json::Error> {
    //     serde_json::to_vec(data)
    // }

    // pub fn deserialize_json<'Caller, TOutput: Deserialize<'Caller>>(
    //     bytes: &'Caller Vec<u8>,
    // ) -> Result<TOutput, serde_json::Error> {
    //     serde_json::from_slice(bytes)
    // }

    // pub fn serialize_msgpack<TData: Serialize>(
    //     data: &TData,
    // ) -> Result<Vec<u8>, rmps::encode::Error> {
    //     rmps::to_vec(data)
    // }

    // pub fn deserialize_msgpack<'Caller, TOutput: Deserialize<'Caller>>(
    //     bytes: &'Caller Vec<u8>,
    // ) -> Result<TOutput, rmps::decode::Error> {
    //     rmps::from_read_ref(bytes)
    // }

    // pub fn serialize_flex<TData: Serialize>(
    //     data: &TData,
    // ) -> Result<Vec<u8>, flexbuffers::SerializationError> {
    //     flexbuffers::to_vec(data)
    // }

    // pub fn deserialize_flex<'Caller, TOutput: Deserialize<'Caller>>(
    //     bytes: &'Caller Vec<u8>,
    // ) -> Result<TOutput, flexbuffers::DeserializationError> {
    //     flexbuffers::from_slice(bytes)
    // }

    // pub fn serialize_bson<TData: Serialize>(
    //     data: &TData,
    // ) -> Result<Vec<u8>, bson::ser::Error> {
    //     bson::to_vec(data)
    // }

    // pub fn deserialize_bson<'Caller, TOutput: Deserialize<'Caller>>(
    //     bytes: &'Caller Vec<u8>,
    // ) -> Result<TOutput, bson::de::Error> {
    //     bson::from_slice(bytes)
    // }

    // pub fn serialize_cbor<TData: Serialize>(
    //     data: &TData,
    // ) -> Result<Vec<u8>, serde_cbor::Error> {
    //     serde_cbor::to_vec(data)
    // }

    // pub fn deserialize_cbor<'Caller, TOutput: Deserialize<'Caller>>(
    //     bytes: &'Caller Vec<u8>,
    // ) -> Result<TOutput, serde_cbor::Error> {
    //     serde_cbor::from_slice(bytes)
    // }
}
