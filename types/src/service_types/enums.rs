// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
use std::io::{Read, Write};

use crate::encoding::*;
use crate::status_codes::StatusCode;

// All enums assumed to be i32 length in bits when encoded.

/// The possible encodings for a NodeId value.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NodeIdType {
    TwoByte = 0,
    FourByte = 1,
    Numeric = 2,
    String = 3,
    Guid = 4,
    ByteString = 5,
}

impl BinaryEncoder<NodeIdType> for NodeIdType {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::TwoByte),
            1 => Ok(Self::FourByte),
            2 => Ok(Self::Numeric),
            3 => Ok(Self::String),
            4 => Ok(Self::Guid),
            5 => Ok(Self::ByteString),
            v => {
                error!("Invalid value {} for enum NodeIdType", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NamingRuleType {
    Mandatory = 1,
    Optional = 2,
    Constraint = 3,
}

impl BinaryEncoder<NamingRuleType> for NamingRuleType {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            1 => Ok(Self::Mandatory),
            2 => Ok(Self::Optional),
            3 => Ok(Self::Constraint),
            v => {
                error!("Invalid value {} for enum NamingRuleType", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpenFileMode {
    Read = 1,
    Write = 2,
    EraseExisting = 4,
    Append = 8,
}

impl BinaryEncoder<OpenFileMode> for OpenFileMode {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            1 => Ok(Self::Read),
            2 => Ok(Self::Write),
            4 => Ok(Self::EraseExisting),
            8 => Ok(Self::Append),
            v => {
                error!("Invalid value {} for enum OpenFileMode", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TrustListMasks {
    None = 0,
    TrustedCertificates = 1,
    TrustedCrls = 2,
    IssuerCertificates = 4,
    IssuerCrls = 8,
    All = 15,
}

impl BinaryEncoder<TrustListMasks> for TrustListMasks {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::TrustedCertificates),
            2 => Ok(Self::TrustedCrls),
            4 => Ok(Self::IssuerCertificates),
            8 => Ok(Self::IssuerCrls),
            15 => Ok(Self::All),
            v => {
                error!("Invalid value {} for enum TrustListMasks", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}

/// The type of identifier used in a node id.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IdType {
    Numeric = 0,
    String = 1,
    Guid = 2,
    Opaque = 3,
}

impl BinaryEncoder<IdType> for IdType {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::Numeric),
            1 => Ok(Self::String),
            2 => Ok(Self::Guid),
            3 => Ok(Self::Opaque),
            v => {
                error!("Invalid value {} for enum IdType", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}

/// A mask specifying the class of the node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NodeClass {
    Unspecified = 0,
    Object = 1,
    Variable = 2,
    Method = 4,
    ObjectType = 8,
    VariableType = 16,
    ReferenceType = 32,
    DataType = 64,
    View = 128,
}

impl BinaryEncoder<NodeClass> for NodeClass {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::Unspecified),
            1 => Ok(Self::Object),
            2 => Ok(Self::Variable),
            4 => Ok(Self::Method),
            8 => Ok(Self::ObjectType),
            16 => Ok(Self::VariableType),
            32 => Ok(Self::ReferenceType),
            64 => Ok(Self::DataType),
            128 => Ok(Self::View),
            v => {
                error!("Invalid value {} for enum NodeClass", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}

/// The types of applications.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ApplicationType {
    Server = 0,
    Client = 1,
    ClientAndServer = 2,
    DiscoveryServer = 3,
}

impl BinaryEncoder<ApplicationType> for ApplicationType {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::Server),
            1 => Ok(Self::Client),
            2 => Ok(Self::ClientAndServer),
            3 => Ok(Self::DiscoveryServer),
            v => {
                error!("Invalid value {} for enum ApplicationType", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}

/// The type of security to use on a message.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MessageSecurityMode {
    Invalid = 0,
    None = 1,
    Sign = 2,
    SignAndEncrypt = 3,
}

impl BinaryEncoder<MessageSecurityMode> for MessageSecurityMode {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::Invalid),
            1 => Ok(Self::None),
            2 => Ok(Self::Sign),
            3 => Ok(Self::SignAndEncrypt),
            v => {
                error!("Invalid value {} for enum MessageSecurityMode", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}

/// The possible user token types.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UserTokenType {
    Anonymous = 0,
    UserName = 1,
    Certificate = 2,
    IssuedToken = 3,
}

impl BinaryEncoder<UserTokenType> for UserTokenType {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::Anonymous),
            1 => Ok(Self::UserName),
            2 => Ok(Self::Certificate),
            3 => Ok(Self::IssuedToken),
            v => {
                error!("Invalid value {} for enum UserTokenType", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}

/// Indicates whether a token if being created or renewed.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SecurityTokenRequestType {
    Issue = 0,
    Renew = 1,
}

impl BinaryEncoder<SecurityTokenRequestType> for SecurityTokenRequestType {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::Issue),
            1 => Ok(Self::Renew),
            v => {
                error!("Invalid value {} for enum SecurityTokenRequestType", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}

/// The bits used to specify default attributes for a new node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NodeAttributesMask {
    None = 0,
    AccessLevel = 1,
    ArrayDimensions = 2,
    BrowseName = 4,
    ContainsNoLoops = 8,
    DataType = 16,
    Description = 32,
    DisplayName = 64,
    EventNotifier = 128,
    Executable = 256,
    Historizing = 512,
    InverseName = 1024,
    IsAbstract = 2048,
    MinimumSamplingInterval = 4096,
    NodeClass = 8192,
    NodeId = 16384,
    Symmetric = 32768,
    UserAccessLevel = 65536,
    UserExecutable = 131072,
    UserWriteMask = 262144,
    ValueRank = 524288,
    WriteMask = 1048576,
    Value = 2097152,
    All = 4194303,
    BaseNode = 1335396,
    Object = 1335524,
    ObjectTypeOrDataType = 1337444,
    Variable = 4026999,
    VariableType = 3958902,
    Method = 1466724,
    ReferenceType = 1371236,
    View = 1335532,
}

impl BinaryEncoder<NodeAttributesMask> for NodeAttributesMask {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::AccessLevel),
            2 => Ok(Self::ArrayDimensions),
            4 => Ok(Self::BrowseName),
            8 => Ok(Self::ContainsNoLoops),
            16 => Ok(Self::DataType),
            32 => Ok(Self::Description),
            64 => Ok(Self::DisplayName),
            128 => Ok(Self::EventNotifier),
            256 => Ok(Self::Executable),
            512 => Ok(Self::Historizing),
            1024 => Ok(Self::InverseName),
            2048 => Ok(Self::IsAbstract),
            4096 => Ok(Self::MinimumSamplingInterval),
            8192 => Ok(Self::NodeClass),
            16384 => Ok(Self::NodeId),
            32768 => Ok(Self::Symmetric),
            65536 => Ok(Self::UserAccessLevel),
            131072 => Ok(Self::UserExecutable),
            262144 => Ok(Self::UserWriteMask),
            524288 => Ok(Self::ValueRank),
            1048576 => Ok(Self::WriteMask),
            2097152 => Ok(Self::Value),
            4194303 => Ok(Self::All),
            1335396 => Ok(Self::BaseNode),
            1335524 => Ok(Self::Object),
            1337444 => Ok(Self::ObjectTypeOrDataType),
            4026999 => Ok(Self::Variable),
            3958902 => Ok(Self::VariableType),
            1466724 => Ok(Self::Method),
            1371236 => Ok(Self::ReferenceType),
            1335532 => Ok(Self::View),
            v => {
                error!("Invalid value {} for enum NodeAttributesMask", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}

/// Define bits used to indicate which attributes are writable.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AttributeWriteMask {
    None = 0,
    AccessLevel = 1,
    ArrayDimensions = 2,
    BrowseName = 4,
    ContainsNoLoops = 8,
    DataType = 16,
    Description = 32,
    DisplayName = 64,
    EventNotifier = 128,
    Executable = 256,
    Historizing = 512,
    InverseName = 1024,
    IsAbstract = 2048,
    MinimumSamplingInterval = 4096,
    NodeClass = 8192,
    NodeId = 16384,
    Symmetric = 32768,
    UserAccessLevel = 65536,
    UserExecutable = 131072,
    UserWriteMask = 262144,
    ValueRank = 524288,
    WriteMask = 1048576,
    ValueForVariableType = 2097152,
}

impl BinaryEncoder<AttributeWriteMask> for AttributeWriteMask {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::AccessLevel),
            2 => Ok(Self::ArrayDimensions),
            4 => Ok(Self::BrowseName),
            8 => Ok(Self::ContainsNoLoops),
            16 => Ok(Self::DataType),
            32 => Ok(Self::Description),
            64 => Ok(Self::DisplayName),
            128 => Ok(Self::EventNotifier),
            256 => Ok(Self::Executable),
            512 => Ok(Self::Historizing),
            1024 => Ok(Self::InverseName),
            2048 => Ok(Self::IsAbstract),
            4096 => Ok(Self::MinimumSamplingInterval),
            8192 => Ok(Self::NodeClass),
            16384 => Ok(Self::NodeId),
            32768 => Ok(Self::Symmetric),
            65536 => Ok(Self::UserAccessLevel),
            131072 => Ok(Self::UserExecutable),
            262144 => Ok(Self::UserWriteMask),
            524288 => Ok(Self::ValueRank),
            1048576 => Ok(Self::WriteMask),
            2097152 => Ok(Self::ValueForVariableType),
            v => {
                error!("Invalid value {} for enum AttributeWriteMask", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}

/// The directions of the references to return.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BrowseDirection {
    Forward = 0,
    Inverse = 1,
    Both = 2,
    Invalid = 3,
}

impl BinaryEncoder<BrowseDirection> for BrowseDirection {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::Forward),
            1 => Ok(Self::Inverse),
            2 => Ok(Self::Both),
            3 => Ok(Self::Invalid),
            v => {
                error!("Invalid value {} for enum BrowseDirection", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}

/// A bit mask which specifies what should be returned in a browse response.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BrowseResultMask {
    None = 0,
    ReferenceTypeId = 1,
    IsForward = 2,
    NodeClass = 4,
    BrowseName = 8,
    DisplayName = 16,
    TypeDefinition = 32,
    All = 63,
    ReferenceTypeInfo = 3,
    TargetInfo = 60,
}

impl BinaryEncoder<BrowseResultMask> for BrowseResultMask {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::ReferenceTypeId),
            2 => Ok(Self::IsForward),
            4 => Ok(Self::NodeClass),
            8 => Ok(Self::BrowseName),
            16 => Ok(Self::DisplayName),
            32 => Ok(Self::TypeDefinition),
            63 => Ok(Self::All),
            3 => Ok(Self::ReferenceTypeInfo),
            60 => Ok(Self::TargetInfo),
            v => {
                error!("Invalid value {} for enum BrowseResultMask", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub enum FilterOperator {
    Equals = 0,
    IsNull = 1,
    GreaterThan = 2,
    LessThan = 3,
    GreaterThanOrEqual = 4,
    LessThanOrEqual = 5,
    Like = 6,
    Not = 7,
    Between = 8,
    InList = 9,
    And = 10,
    Or = 11,
    Cast = 12,
    InView = 13,
    OfType = 14,
    RelatedTo = 15,
    BitwiseAnd = 16,
    BitwiseOr = 17,
}

impl BinaryEncoder<FilterOperator> for FilterOperator {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::Equals),
            1 => Ok(Self::IsNull),
            2 => Ok(Self::GreaterThan),
            3 => Ok(Self::LessThan),
            4 => Ok(Self::GreaterThanOrEqual),
            5 => Ok(Self::LessThanOrEqual),
            6 => Ok(Self::Like),
            7 => Ok(Self::Not),
            8 => Ok(Self::Between),
            9 => Ok(Self::InList),
            10 => Ok(Self::And),
            11 => Ok(Self::Or),
            12 => Ok(Self::Cast),
            13 => Ok(Self::InView),
            14 => Ok(Self::OfType),
            15 => Ok(Self::RelatedTo),
            16 => Ok(Self::BitwiseAnd),
            17 => Ok(Self::BitwiseOr),
            v => {
                error!("Invalid value {} for enum FilterOperator", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub enum TimestampsToReturn {
    Source = 0,
    Server = 1,
    Both = 2,
    Neither = 3,
    Invalid = 4,
}

impl BinaryEncoder<TimestampsToReturn> for TimestampsToReturn {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::Source),
            1 => Ok(Self::Server),
            2 => Ok(Self::Both),
            3 => Ok(Self::Neither),
            4 => Ok(Self::Invalid),
            v => {
                error!("Invalid value {} for enum TimestampsToReturn", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HistoryUpdateType {
    Insert = 1,
    Replace = 2,
    Update = 3,
    Delete = 4,
}

impl BinaryEncoder<HistoryUpdateType> for HistoryUpdateType {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            1 => Ok(Self::Insert),
            2 => Ok(Self::Replace),
            3 => Ok(Self::Update),
            4 => Ok(Self::Delete),
            v => {
                error!("Invalid value {} for enum HistoryUpdateType", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PerformUpdateType {
    Insert = 1,
    Replace = 2,
    Update = 3,
    Remove = 4,
}

impl BinaryEncoder<PerformUpdateType> for PerformUpdateType {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            1 => Ok(Self::Insert),
            2 => Ok(Self::Replace),
            3 => Ok(Self::Update),
            4 => Ok(Self::Remove),
            v => {
                error!("Invalid value {} for enum PerformUpdateType", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub enum MonitoringMode {
    Disabled = 0,
    Sampling = 1,
    Reporting = 2,
}

impl BinaryEncoder<MonitoringMode> for MonitoringMode {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::Disabled),
            1 => Ok(Self::Sampling),
            2 => Ok(Self::Reporting),
            v => {
                error!("Invalid value {} for enum MonitoringMode", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub enum DataChangeTrigger {
    Status = 0,
    StatusValue = 1,
    StatusValueTimestamp = 2,
}

impl BinaryEncoder<DataChangeTrigger> for DataChangeTrigger {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::Status),
            1 => Ok(Self::StatusValue),
            2 => Ok(Self::StatusValueTimestamp),
            v => {
                error!("Invalid value {} for enum DataChangeTrigger", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DeadbandType {
    None = 0,
    Absolute = 1,
    Percent = 2,
}

impl BinaryEncoder<DeadbandType> for DeadbandType {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Absolute),
            2 => Ok(Self::Percent),
            v => {
                error!("Invalid value {} for enum DeadbandType", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RedundancySupport {
    None = 0,
    Cold = 1,
    Warm = 2,
    Hot = 3,
    Transparent = 4,
    HotAndMirrored = 5,
}

impl BinaryEncoder<RedundancySupport> for RedundancySupport {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Cold),
            2 => Ok(Self::Warm),
            3 => Ok(Self::Hot),
            4 => Ok(Self::Transparent),
            5 => Ok(Self::HotAndMirrored),
            v => {
                error!("Invalid value {} for enum RedundancySupport", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ServerState {
    Running = 0,
    Failed = 1,
    NoConfiguration = 2,
    Suspended = 3,
    Shutdown = 4,
    Test = 5,
    CommunicationFault = 6,
    Unknown = 7,
}

impl BinaryEncoder<ServerState> for ServerState {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::Running),
            1 => Ok(Self::Failed),
            2 => Ok(Self::NoConfiguration),
            3 => Ok(Self::Suspended),
            4 => Ok(Self::Shutdown),
            5 => Ok(Self::Test),
            6 => Ok(Self::CommunicationFault),
            7 => Ok(Self::Unknown),
            v => {
                error!("Invalid value {} for enum ServerState", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ModelChangeStructureVerbMask {
    NodeAdded = 1,
    NodeDeleted = 2,
    ReferenceAdded = 4,
    ReferenceDeleted = 8,
    DataTypeChanged = 16,
}

impl BinaryEncoder<ModelChangeStructureVerbMask> for ModelChangeStructureVerbMask {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            1 => Ok(Self::NodeAdded),
            2 => Ok(Self::NodeDeleted),
            4 => Ok(Self::ReferenceAdded),
            8 => Ok(Self::ReferenceDeleted),
            16 => Ok(Self::DataTypeChanged),
            v => {
                error!("Invalid value {} for enum ModelChangeStructureVerbMask", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AxisScaleEnumeration {
    Linear = 0,
    Log = 1,
    Ln = 2,
}

impl BinaryEncoder<AxisScaleEnumeration> for AxisScaleEnumeration {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::Linear),
            1 => Ok(Self::Log),
            2 => Ok(Self::Ln),
            v => {
                error!("Invalid value {} for enum AxisScaleEnumeration", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ExceptionDeviationFormat {
    AbsoluteValue = 0,
    PercentOfValue = 1,
    PercentOfRange = 2,
    PercentOfEURange = 3,
    Unknown = 4,
}

impl BinaryEncoder<ExceptionDeviationFormat> for ExceptionDeviationFormat {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self as i32)
    }

    fn decode<S: Read>(stream: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
        let value = read_i32(stream)?;
        match value {
            0 => Ok(Self::AbsoluteValue),
            1 => Ok(Self::PercentOfValue),
            2 => Ok(Self::PercentOfRange),
            3 => Ok(Self::PercentOfEURange),
            4 => Ok(Self::Unknown),
            v => {
                error!("Invalid value {} for enum ExceptionDeviationFormat", v);
                Err(StatusCode::BadUnexpectedError)
            }
        }
    }
}
