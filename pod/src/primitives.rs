//! primitive types that can be used in `Pod`s
#[cfg(feature = "borsh")]
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use bytemuck_derive::{Pod, Zeroable};
use ethnum::U256;
#[cfg(feature = "serde-traits")]
use serde::{Deserialize, Serialize};

/// The standard `bool` is not a `Pod`, define a replacement that is
#[cfg_attr(feature = "serde-traits", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-traits", serde(from = "bool", into = "bool"))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct PodBool(pub u8);
impl PodBool {
    pub const fn from_bool(b: bool) -> Self {
        Self(if b { 1 } else { 0 })
    }
}

impl From<bool> for PodBool {
    fn from(b: bool) -> Self {
        Self::from_bool(b)
    }
}

impl From<&bool> for PodBool {
    fn from(b: &bool) -> Self {
        Self(if *b { 1 } else { 0 })
    }
}

impl From<&PodBool> for bool {
    fn from(b: &PodBool) -> Self {
        b.0 != 0
    }
}

impl From<PodBool> for bool {
    fn from(b: PodBool) -> Self {
        b.0 != 0
    }
}

/// Simple macro for implementing conversion functions between Pod* integers and
/// standard integers.
///
/// The standard integer types can cause alignment issues when placed in a `Pod`,
/// so these replacements are usable in all `Pod`s.
#[macro_export]
macro_rules! impl_int_conversion {
    ($P:ty, $I:ty) => {
        impl $P {
            pub const fn from_primitive(n: $I) -> Self {
                Self(n.to_le_bytes())
            }
        }
        impl From<$I> for $P {
            fn from(n: $I) -> Self {
                Self::from_primitive(n)
            }
        }
        impl From<$P> for $I {
            fn from(pod: $P) -> Self {
                Self::from_le_bytes(pod.0)
            }
        }
    };
}

/// `u16` type that can be used in `Pod`s
#[cfg_attr(feature = "serde-traits", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-traits", serde(from = "u16", into = "u16"))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct PodU16(pub [u8; 2]);
impl_int_conversion!(PodU16, u16);

/// `i16` type that can be used in Pods
#[cfg_attr(feature = "serde-traits", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-traits", serde(from = "i16", into = "i16"))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct PodI16(pub [u8; 2]);
impl_int_conversion!(PodI16, i16);

/// `u32` type that can be used in `Pod`s
#[cfg_attr(
    feature = "borsh",
    derive(BorshDeserialize, BorshSerialize, BorshSchema)
)]
#[cfg_attr(feature = "serde-traits", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-traits", serde(from = "u32", into = "u32"))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct PodU32(pub [u8; 4]);
impl_int_conversion!(PodU32, u32);

/// `u64` type that can be used in Pods
#[cfg_attr(
    feature = "borsh",
    derive(BorshDeserialize, BorshSerialize, BorshSchema)
)]
#[cfg_attr(feature = "serde-traits", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-traits", serde(from = "u64", into = "u64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct PodU64(pub [u8; 8]);
impl_int_conversion!(PodU64, u64);

/// `i64` type that can be used in Pods
#[cfg_attr(feature = "serde-traits", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-traits", serde(from = "i64", into = "i64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct PodI64([u8; 8]);
impl_int_conversion!(PodI64, i64);

/// `u128` type that can be used in Pods
#[cfg_attr(
    feature = "borsh",
    derive(BorshDeserialize, BorshSerialize, BorshSchema)
)]
#[cfg_attr(feature = "serde-traits", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-traits", serde(from = "u128", into = "u128"))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct PodU128(pub [u8; 16]);
impl_int_conversion!(PodU128, u128);

/// `U256` type that can be used in Pods
#[cfg_attr(
    feature = "borsh",
    derive(BorshDeserialize, BorshSerialize, BorshSchema)
)]
#[cfg_attr(feature = "serde-traits", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-traits", serde(from = "U256", into = "U256"))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct PodU256(pub [u8; 32]);

impl PodU256 {
    /// Creates a new PodU256 from raw bytes in little-endian format
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Creates a new PodU256 from a u128 value
    pub const fn from_u128(value: u128) -> Self {
        let mut bytes = [0u8; 32];
        let value_bytes = value.to_le_bytes();
        let mut i = 0;
        while i < 16 {
            bytes[i] = value_bytes[i];
            i += 1;
        }
        Self(bytes)
    }

    /// Creates a new PodU256 from a u64 value
    pub const fn from_u64(value: u64) -> Self {
        let mut bytes = [0u8; 32];
        let value_bytes = value.to_le_bytes();
        let mut i = 0;
        while i < 8 {
            bytes[i] = value_bytes[i];
            i += 1;
        }
        Self(bytes)
    }
}

impl From<U256> for PodU256 {
    fn from(n: U256) -> Self {
        Self(n.to_le_bytes())
    }
}

impl From<PodU256> for U256 {
    fn from(pod: PodU256) -> Self {
        Self::from_le_bytes(pod.0)
    }
}

impl From<u64> for PodU256 {
    fn from(n: u64) -> Self {
        Self::from_u64(n)
    }
}

impl From<PodU256> for u64 {
    fn from(pod: PodU256) -> Self {
        // Convert PodU256 to U256 first, then to u64
        let u256_value = U256::from(pod);
        u256_value.try_into().unwrap_or(0) // If value is too large, return 0
    }
}

impl From<PodU64> for U256 {
    fn from(pod: PodU64) -> Self {
        let value: u64 = pod.into();
        value.into()
    }
}

impl From<U256> for PodU64 {
    fn from(n: U256) -> Self {
        if n > U256::from(u64::MAX) {
            panic!(
                "attempted to convert U256 value {} which is larger than u64::MAX",
                n
            );
        }
        Self::from(n.as_u64())
    }
}

impl From<PodU64> for PodU256 {
    fn from(pod: PodU64) -> Self {
        let value: u64 = pod.into();
        value.into()
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::bytemuck::pod_from_bytes};

    #[test]
    fn test_pod_bool() {
        assert!(pod_from_bytes::<PodBool>(&[]).is_err());
        assert!(pod_from_bytes::<PodBool>(&[0, 0]).is_err());

        for i in 0..=u8::MAX {
            assert_eq!(i != 0, bool::from(pod_from_bytes::<PodBool>(&[i]).unwrap()));
        }
    }

    #[cfg(feature = "serde-traits")]
    #[test]
    fn test_pod_bool_serde() {
        let pod_false: PodBool = false.into();
        let pod_true: PodBool = true.into();

        let serialized_false = serde_json::to_string(&pod_false).unwrap();
        let serialized_true = serde_json::to_string(&pod_true).unwrap();
        assert_eq!(&serialized_false, "false");
        assert_eq!(&serialized_true, "true");

        let deserialized_false = serde_json::from_str::<PodBool>(&serialized_false).unwrap();
        let deserialized_true = serde_json::from_str::<PodBool>(&serialized_true).unwrap();
        assert_eq!(pod_false, deserialized_false);
        assert_eq!(pod_true, deserialized_true);
    }

    #[test]
    fn test_pod_u16() {
        assert!(pod_from_bytes::<PodU16>(&[]).is_err());
        assert_eq!(1u16, u16::from(*pod_from_bytes::<PodU16>(&[1, 0]).unwrap()));
    }

    #[cfg(feature = "serde-traits")]
    #[test]
    fn test_pod_u16_serde() {
        let pod_u16: PodU16 = u16::MAX.into();

        let serialized = serde_json::to_string(&pod_u16).unwrap();
        assert_eq!(&serialized, "65535");

        let deserialized = serde_json::from_str::<PodU16>(&serialized).unwrap();
        assert_eq!(pod_u16, deserialized);
    }

    #[test]
    fn test_pod_i16() {
        assert!(pod_from_bytes::<PodI16>(&[]).is_err());
        assert_eq!(
            -1i16,
            i16::from(*pod_from_bytes::<PodI16>(&[255, 255]).unwrap())
        );
    }

    #[cfg(feature = "serde-traits")]
    #[test]
    fn test_pod_i16_serde() {
        let pod_i16: PodI16 = i16::MAX.into();

        println!("pod_i16 {:?}", pod_i16);

        let serialized = serde_json::to_string(&pod_i16).unwrap();
        assert_eq!(&serialized, "32767");

        let deserialized = serde_json::from_str::<PodI16>(&serialized).unwrap();
        assert_eq!(pod_i16, deserialized);
    }

    #[test]
    fn test_pod_u64() {
        assert!(pod_from_bytes::<PodU64>(&[]).is_err());
        assert_eq!(
            1u64,
            u64::from(*pod_from_bytes::<PodU64>(&[1, 0, 0, 0, 0, 0, 0, 0]).unwrap())
        );
    }

    #[cfg(feature = "serde-traits")]
    #[test]
    fn test_pod_u64_serde() {
        let pod_u64: PodU64 = u64::MAX.into();

        let serialized = serde_json::to_string(&pod_u64).unwrap();
        assert_eq!(&serialized, "18446744073709551615");

        let deserialized = serde_json::from_str::<PodU64>(&serialized).unwrap();
        assert_eq!(pod_u64, deserialized);
    }

    #[test]
    fn test_pod_i64() {
        assert!(pod_from_bytes::<PodI64>(&[]).is_err());
        assert_eq!(
            -1i64,
            i64::from(
                *pod_from_bytes::<PodI64>(&[255, 255, 255, 255, 255, 255, 255, 255]).unwrap()
            )
        );
    }

    #[cfg(feature = "serde-traits")]
    #[test]
    fn test_pod_i64_serde() {
        let pod_i64: PodI64 = i64::MAX.into();

        let serialized = serde_json::to_string(&pod_i64).unwrap();
        assert_eq!(&serialized, "9223372036854775807");

        let deserialized = serde_json::from_str::<PodI64>(&serialized).unwrap();
        assert_eq!(pod_i64, deserialized);
    }

    #[test]
    fn test_pod_u128() {
        assert!(pod_from_bytes::<PodU128>(&[]).is_err());
        assert_eq!(
            1u128,
            u128::from(
                *pod_from_bytes::<PodU128>(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
                    .unwrap()
            )
        );
    }

    #[cfg(feature = "serde-traits")]
    #[test]
    fn test_pod_u128_serde() {
        let pod_u128: PodU128 = u128::MAX.into();

        let serialized = serde_json::to_string(&pod_u128).unwrap();
        assert_eq!(&serialized, "340282366920938463463374607431768211455");

        let deserialized = serde_json::from_str::<PodU128>(&serialized).unwrap();
        assert_eq!(pod_u128, deserialized);
    }

    #[test]
    fn test_pod_u256() {
        assert!(pod_from_bytes::<PodU256>(&[]).is_err());
        assert_eq!(
            U256::from(1u64),
            U256::from(
                *pod_from_bytes::<PodU256>(&[
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0
                ])
                .unwrap()
            )
        );
    }

    #[cfg(feature = "serde-traits")]
    #[test]
    fn test_pod_u256_serde() {
        let pod_u256: PodU256 = u256::MAX.into();

        let serialized = serde_json::to_string(&pod_u256).unwrap();
        assert_eq!(&serialized, "340282366920938463463374607431768211455");

        let deserialized = serde_json::from_str::<PodU256>(&serialized).unwrap();
        assert_eq!(pod_u256, deserialized);
    }

    #[test]
    fn test_u256_pod_conversions() {
        // Test u64 -> U256 -> PodU64 conversion
        let original_u64 = 12345u64;
        let u256_value = U256::from(original_u64);
        let pod_u64: PodU64 = u256_value.into();
        assert_eq!(u64::from(pod_u64), original_u64);

        // Test max u64 conversion
        let max_u64 = u64::MAX;
        let u256_max = U256::from(max_u64);
        let pod_u64_max: PodU64 = u256_max.into();
        assert_eq!(u64::from(pod_u64_max), max_u64);

        // Test U256 -> PodU256 -> u64 conversion
        let small_u256 = U256::from(54321u64);
        let pod_u256: PodU256 = small_u256.into();
        let converted_u64: u64 = pod_u256.into();
        assert_eq!(converted_u64, 54321u64);

        // Test PodU64 -> U256 conversion
        let pod_u64 = PodU64::from(9999u64);
        let converted_u256: U256 = pod_u64.into();
        assert_eq!(converted_u256, U256::from(9999u64));
    }

    #[test]
    #[should_panic(expected = "attempted to convert U256 value")]
    fn test_u256_pod_overflow() {
        let large_u256 = U256::from(u64::MAX) + U256::from(1u64);
        let _should_panic: PodU64 = large_u256.into();
    }
}
