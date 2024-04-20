use num_enum::TryFromPrimitive;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

// TODO: Look into this properly.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[repr(u8)]
pub enum SchemaFieldType {
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
