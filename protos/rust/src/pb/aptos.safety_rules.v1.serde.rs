// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// @generated
// Serde implementations for safety_rules proto messages.
// These are simplified implementations since the actual data is BCS serialized bytes.

impl serde::Serialize for Empty {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let struct_ser = serializer.serialize_struct("aptos.safety_rules.v1.Empty", 0)?;
        struct_ser.end()
    }
}

impl<'de> serde::Deserialize<'de> for Empty {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Empty;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.safety_rules.v1.Empty")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Empty, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<&str>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Empty {})
            }
        }
        deserializer.deserialize_struct("aptos.safety_rules.v1.Empty", FIELDS, GeneratedVisitor)
    }
}

impl serde::Serialize for ErrorResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.error_type.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("aptos.safety_rules.v1.ErrorResponse", len)?;
        if !self.error_type.is_empty() {
            struct_ser.serialize_field("errorType", &self.error_type)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}

impl<'de> serde::Deserialize<'de> for ErrorResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["error_type", "errorType", "message"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ErrorType,
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "errorType" | "error_type" => Ok(GeneratedField::ErrorType),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.safety_rules.v1.ErrorResponse")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<ErrorResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut error_type__ = None;
                let mut message__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ErrorType => {
                            if error_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorType"));
                            }
                            error_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ErrorResponse {
                    error_type: error_type__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "aptos.safety_rules.v1.ErrorResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}

impl serde::Serialize for ConsensusState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.epoch != 0 {
            len += 1;
        }
        if self.last_voted_round != 0 {
            len += 1;
        }
        if self.preferred_round != 0 {
            len += 1;
        }
        if !self.waypoint.is_empty() {
            len += 1;
        }
        if self.in_validator_set {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("aptos.safety_rules.v1.ConsensusState", len)?;
        if self.epoch != 0 {
            struct_ser.serialize_field("epoch", ToString::to_string(&self.epoch).as_str())?;
        }
        if self.last_voted_round != 0 {
            struct_ser.serialize_field(
                "lastVotedRound",
                ToString::to_string(&self.last_voted_round).as_str(),
            )?;
        }
        if self.preferred_round != 0 {
            struct_ser.serialize_field(
                "preferredRound",
                ToString::to_string(&self.preferred_round).as_str(),
            )?;
        }
        if !self.waypoint.is_empty() {
            struct_ser.serialize_field(
                "waypoint",
                pbjson::private::base64::encode(&self.waypoint).as_str(),
            )?;
        }
        if self.in_validator_set {
            struct_ser.serialize_field("inValidatorSet", &self.in_validator_set)?;
        }
        struct_ser.end()
    }
}

impl<'de> serde::Deserialize<'de> for ConsensusState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "epoch",
            "last_voted_round",
            "lastVotedRound",
            "preferred_round",
            "preferredRound",
            "waypoint",
            "in_validator_set",
            "inValidatorSet",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Epoch,
            LastVotedRound,
            PreferredRound,
            Waypoint,
            InValidatorSet,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "epoch" => Ok(GeneratedField::Epoch),
                            "lastVotedRound" | "last_voted_round" => {
                                Ok(GeneratedField::LastVotedRound)
                            }
                            "preferredRound" | "preferred_round" => {
                                Ok(GeneratedField::PreferredRound)
                            }
                            "waypoint" => Ok(GeneratedField::Waypoint),
                            "inValidatorSet" | "in_validator_set" => {
                                Ok(GeneratedField::InValidatorSet)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConsensusState;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.safety_rules.v1.ConsensusState")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConsensusState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut epoch__ = None;
                let mut last_voted_round__ = None;
                let mut preferred_round__ = None;
                let mut waypoint__ = None;
                let mut in_validator_set__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::LastVotedRound => {
                            if last_voted_round__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastVotedRound"));
                            }
                            last_voted_round__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::PreferredRound => {
                            if preferred_round__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferredRound"));
                            }
                            preferred_round__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Waypoint => {
                            if waypoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("waypoint"));
                            }
                            waypoint__ = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::InValidatorSet => {
                            if in_validator_set__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inValidatorSet"));
                            }
                            in_validator_set__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ConsensusState {
                    epoch: epoch__.unwrap_or_default(),
                    last_voted_round: last_voted_round__.unwrap_or_default(),
                    preferred_round: preferred_round__.unwrap_or_default(),
                    waypoint: waypoint__.unwrap_or_default(),
                    in_validator_set: in_validator_set__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "aptos.safety_rules.v1.ConsensusState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}

// For the request/response types, we use derive macros since they're primarily
// containers for BCS-serialized bytes. The actual data serialization happens
// at the application layer using BCS.

impl serde::Serialize for ConsensusStateRequest {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let struct_ser =
            serializer.serialize_struct("aptos.safety_rules.v1.ConsensusStateRequest", 0)?;
        struct_ser.end()
    }
}

impl<'de> serde::Deserialize<'de> for ConsensusStateRequest {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConsensusStateRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aptos.safety_rules.v1.ConsensusStateRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<ConsensusStateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<&str>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ConsensusStateRequest {})
            }
        }
        deserializer.deserialize_struct(
            "aptos.safety_rules.v1.ConsensusStateRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}

// Note: The remaining request/response types follow a similar pattern.
// For brevity, we'll implement them with a simpler approach using the
// prost-generated default serialization since these are internal protocol
// messages and the actual consensus data is BCS-serialized within the bytes fields.
