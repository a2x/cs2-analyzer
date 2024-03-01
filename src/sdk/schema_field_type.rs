use num_enum::TryFromPrimitive;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[repr(u8)]
pub enum SchemaFieldType {
    Unknown = 0,
    Int8 = 6,
    UInt8 = 7,
    Int16 = 8,
    UInt16 = 9,
    Int32 = 10,
    UInt32 = 11,
    Int64 = 12,
    UInt64 = 13,
    Float32 = 14,
    Float64 = 15,
    Bool = 16,
    Vector = 18,
    VectorAligned = 19,
    Vector2D = 20,
    Vector4D = 21,
    QAngle = 22,
    Quaternion = 23,
    QuaternionStorage = 24,
    RadianEuler = 25,
    DegreeEuler = 26,
    Matrix3x4 = 28,
    Matrix3x4a = 29,
    CTransform = 30,
    Color = 32,
    CUtlBinaryBlock = 34,
    CUtlString = 35,
    CUtlSymbol = 36,
    CUtlStringToken = 38,
}

impl SchemaFieldType {
    #[inline]
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Int8 => "int8_t",
            Self::UInt8 => "uint8_t",
            Self::Int16 => "int16_t",
            Self::UInt16 => "uint16_t",
            Self::Int32 => "int32_t",
            Self::UInt32 => "uint32_t",
            Self::Int64 => "int64_t",
            Self::UInt64 => "uint64_t",
            Self::Float32 => "float",
            Self::Float64 => "double",
            Self::Bool => "bool",
            Self::Vector => "Vector",
            Self::VectorAligned => "VectorAligned",
            Self::Vector2D => "Vector2D",
            Self::Vector4D => "Vector4D",
            Self::QAngle => "QAngle",
            Self::Quaternion => "Quaternion",
            Self::QuaternionStorage => "QuaternionStorage",
            Self::RadianEuler => "RadianEuler",
            Self::DegreeEuler => "DegreeEuler",
            Self::Matrix3x4 => "matrix3x4_t",
            Self::Matrix3x4a => "matrix3x4a_t",
            Self::CTransform => "CTransform",
            Self::Color => "Color",
            Self::CUtlBinaryBlock => "CUtlBinaryBlock",
            Self::CUtlString => "CUtlString",
            Self::CUtlSymbol => "CUtlSymbol",
            Self::CUtlStringToken => "CUtlStringToken",
        }
    }
}
