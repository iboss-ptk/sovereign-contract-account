use prost::EncodeError;

#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct MsgGrantAllowance {
    /// granter is the address of the user granting an allowance of their funds.
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    /// grantee is the address of the user being granted an allowance of another user's funds.
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    /// allowance can be any of basic and filtered fee allowance.
    #[prost(message, optional, tag = "3")]
    pub allowance: ::core::option::Option<Any>,
}
/// BasicAllowance implements Allowance with a one-time grant of tokens
/// that optionally expires. The grantee can use up to SpendLimit to cover fees.
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct BasicAllowance {
    /// spend_limit specifies the maximum amount of tokens that can be spent
    /// by this allowance and will be updated as tokens are spent. If it is
    /// empty, there is no spend limit and any amount of coins can be spent.
    #[prost(message, repeated, tag = "1")]
    pub spend_limit: ::prost::alloc::vec::Vec<Coin>,
    /// expiration specifies an optional time when this allowance expires
    #[prost(message, optional, tag = "2")]
    pub expiration: ::core::option::Option<Timestamp>,
}

/// Coin defines a token with a denomination and an amount.
///
/// NOTE: The amount field is an Int which implements the custom method
/// signatures required by gogoproto.
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct Coin {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct Any {
    /// A URL/resource name that uniquely identifies the type of the serialized
    /// protocol buffer message. This string must contain at least
    /// one "/" character. The last segment of the URL's path must represent
    /// the fully qualified name of the type (as in
    /// `path/google.protobuf.Duration`). The name should be in a canonical form
    /// (e.g., leading "." is not accepted).
    ///
    /// In practice, teams usually precompile into the binary all types that they
    /// expect it to use in the context of Any. However, for URLs which use the
    /// scheme `http`, `https`, or no scheme, one can optionally set up a type
    /// server that maps type URLs to message definitions as follows:
    ///
    /// * If no scheme is provided, `https` is assumed.
    /// * An HTTP GET on the URL must yield a \\[google.protobuf.Type\]\[\\]
    ///   value in binary format, or produce an error.
    /// * Applications are allowed to cache lookup results based on the
    ///   URL, or have them precompiled into a binary to avoid any
    ///   lookup. Therefore, binary compatibility needs to be preserved
    ///   on changes to types. (Use versioned type names to manage
    ///   breaking changes.)
    ///
    /// Note: this functionality is not currently available in the official
    /// protobuf release, and it is not used for type URLs beginning with
    /// type.googleapis.com.
    ///
    /// Schemes other than `http`, `https` (or the empty scheme) might be
    /// used with implementation specific semantics.
    #[prost(string, tag = "1")]
    pub type_url: ::prost::alloc::string::String,
    /// Must be a valid serialized protocol buffer of the above specified type.
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}

#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct Timestamp {
    /// Represents seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    /// Non-negative fractions of a second at nanosecond resolution. Negative
    /// second values with fractions must still have non-negative nanos values
    /// that count forward in time. Must be from 0 to 999,999,999
    /// inclusive.
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}

pub trait MessageExt: prost::Message {
    fn to_bytes(&self) -> Result<Vec<u8>, EncodeError>;
}
impl<M> MessageExt for M
where
    M: prost::Message,
{
    fn to_bytes(&self) -> Result<Vec<u8>, EncodeError> {
        let mut bytes = Vec::new();
        prost::Message::encode(self, &mut bytes)?;
        Ok(bytes)
    }
}
