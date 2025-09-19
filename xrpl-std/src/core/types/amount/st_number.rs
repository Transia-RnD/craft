use crate::core::types::amount::opaque_float::OpaqueFloat;

/// Represents an STNumber value (mantissa * 10^exponent)
#[derive(Debug, Clone, Copy)]
pub struct STNumber {
    pub mantissa: i64,
    pub exponent: i32,
}

impl STNumber {
    /// Create from 12-byte serialized format (BIG-ENDIAN)
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 12 {
            return None;
        }
        
        let mantissa_bytes: [u8; 8] = bytes[0..8].try_into().ok()?;
        let exponent_bytes: [u8; 4] = bytes[8..12].try_into().ok()?;
        
        Some(STNumber {
            mantissa: i64::from_be_bytes(mantissa_bytes),
            exponent: i32::from_be_bytes(exponent_bytes),
        })
    }
    
    /// Convert to 12-byte serialized format (BIG-ENDIAN)
    pub fn to_bytes(&self) -> [u8; 12] {
        let mut bytes = [0u8; 12];
        bytes[0..8].copy_from_slice(&self.mantissa.to_be_bytes());
        bytes[8..12].copy_from_slice(&self.exponent.to_be_bytes());
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stnumber_roundtrip() {
        let st = STNumber {
            mantissa: 123456789,
            exponent: -6,
        };
        
        let bytes = st.to_bytes();
        assert_eq!(bytes.len(), 12);
        
        let st2 = STNumber::from_bytes(&bytes).unwrap();
        assert_eq!(st.mantissa, st2.mantissa);
        assert_eq!(st.exponent, st2.exponent);
    }
    
    #[test]
    fn test_opaque_from_stnumber_bytes() {
        let st = STNumber {
            mantissa: 1000000,
            exponent: -3,
        };
        
        let bytes = st.to_bytes();
        let opaque = OpaqueFloat::from_stnumber_bytes(&bytes).unwrap();
        
        // The opaque float should be created (actual conversion logic would need host functions)
        assert_eq!(opaque.0.len(), 8);
    }
}