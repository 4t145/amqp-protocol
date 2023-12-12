use std::{collections::HashMap, time::Duration};

use amqp_types::{Binary, Symbol, Types, Value};

#[derive(Debug, Clone, Types)]
#[amqp(restrict(source = bool))]
pub enum Role {
    #[amqp(choice = false)]
    Sender,
    #[amqp(choice = true)]
    Receiver,
}

#[derive(Debug, Clone, Types)]
#[amqp(restrict(source = u8))]
pub enum SenderSettleMode {
    #[amqp(choice = 0)]
    Unsettled,
    #[amqp(choice = 1)]
    Settled,
    #[amqp(choice = 2)]
    Mixed,
}

#[derive(Debug, Clone, Types)]
#[amqp(restrict(source = u8))]
pub enum ReceiverSettleMode {
    #[amqp(choice = 0)]
    First,
    #[amqp(choice = 1)]
    Second,
}

#[derive(Debug, Clone, Copy, Default, Types, PartialEq, Eq, Hash)]
pub struct Handle(pub u32);

#[derive(Debug, Clone, Copy, Default, Types, PartialEq, Eq)]
pub struct Seconds(pub u32);

impl From<Seconds> for Duration {
    fn from(val: Seconds) -> Self {
        Duration::from_secs(val.0.into())
    }
}

#[derive(Debug, Clone, Copy, Default, Types, PartialEq, Eq)]
pub struct Milliseconds(pub u32);

impl From<Milliseconds> for Duration {
    fn from(val: Milliseconds) -> Self {
        Duration::from_millis(val.0.into())
    }
}

#[derive(Debug, Clone, Default, Types, PartialEq, Eq)]
#[amqp(restrict(validation = |b: &Binary| b.len() <= 32))]
pub struct DeliveryTag(pub Binary);

pub type DeliveryNumber = SequenceNo;
pub type TransferNumber = SequenceNo;

#[derive(Debug, Clone, Copy, Default, Types)]
// should be RFC-1982
pub struct SequenceNo(pub u32);

#[derive(Debug, Clone, Types)]
pub struct IetfLanguageTag(pub Symbol);
pub type Fields = HashMap<Symbol, Value>;

#[derive(Debug, Clone, Types)]
pub struct Error {
    pub condition: Symbol,
    pub description: Option<String>,
    pub info: Option<Fields>,
}

const fn sym(s: &'static str) -> Symbol {
    Symbol::from_static_str(s)
}

/// Shared error conditions.
#[derive(Debug, Clone, Copy, Types)]
#[amqp(restrict(source = Symbol))]
pub enum AmqpError {
    /// An internal error occurred. Operator intervention may be required to resume normal
    /// operation.
    #[amqp(choice = sym("amqp:internal-error"))]
    InternalError,
    /// A peer attempted to work with a remote entity that does not exist.
    #[amqp(choice = sym("amqp:not-found"))]
    NotFound,
    /// A peer attempted to work with a remote entity to which it has no access due to
    /// security settings.
    #[amqp(choice = sym("amqp:unauthorized-access"))]
    UnauthorizedAccess,
    /// Data could not be decoded.
    #[amqp(choice = sym("amqp:decode-error"))]
    DecodeError,
    /// A peer exceeded its resource allocation.
    #[amqp(choice = sym("amqp:resource-limit-exceeded"))]
    ResourceLimitExceeded,
    /// The peer tried to use a frame in a manner that is inconsistent with the semantics
    /// defined in the specification.
    #[amqp(choice = sym("amqp:not-allowed"))]
    NotAllowed,
    /// An invalid field was passed in a frame body, and the operation could not proceed.
    #[amqp(choice = sym("amqp:invalid-field"))]
    InvalidField,
    /// The peer tried to use functionality that is not implemented in its partner.
    #[amqp(choice = sym("amqp:not-implemented"))]
    NotImplemented,
    /// The client attempted to work with a server entity to which it has no access because
    /// another client is working with it.
    #[amqp(choice = sym("amqp:resource-locked"))]
    ResourceLocked,
    /// The client made a request that was not allowed because some precondition failed.
    #[amqp(choice = sym("amqp:precondition-failed"))]
    PreconditionFailed,
    /// A server entity the client is working with has been deleted.
    #[amqp(choice = sym("amqp:resource-deleted"))]
    ResourceDeleted,
    /// The peer sent a frame that is not permitted in the current state of the Session.
    #[amqp(choice = sym("amqp:illegal-state"))]
    IllegalState,
    /// The peer cannot send a frame because the smallest encoding of the performative
    /// with the currently valid values would be too large to fit within a frame of the agreed
    /// maximum frame size. When transferring a message the message data can be sent in
    /// multiple transfer frames thereby avoiding this error. Similarly when attaching a link
    /// with a large unsettled map the endpoint may make use of the incomplete-unsettled
    /// flag to avoid the need for overly large frames.
    #[amqp(choice = sym("amqp:frame-size-too-small"))]
    FrameSizeTooSmall,
}



/* ==========================================================================
                             CONST VALUES
==========================================================================*/


/// the IANA assigned port number for AMQP.
/// 
/// The standard AMQP port number that has been assigned
/// by IANA for TCP, UDP, and SCTP.
/// 
/// There are currently no UDP or SCTP mappings defined for
/// AMQP. The port number is reserved for future transport
/// mappings to these protocols.
pub const PORT: u16 = 5672;

/// the IANA assigned port number for secure AMQP (amqps).
/// 
/// The standard AMQP port number that has been assigned
/// by IANA for secure TCP using TLS.
/// 
/// Implementations listening on this port should NOT expect
/// a protocol handshake before TLS is negotiated.
pub const SECURE_PORT: u16 = 5671;

/// major protocol version.
pub const MAJOR: u8 = 1;

/// minor protocol version.
pub const MINOR: u8 = 0;

/// protocol revision.
pub const REVISION: u8 = 0;


/// the lower bound for the agreed maximum frame size (in
/// bytes).
/// 
/// During the initial Connection negotiation, the two peers
/// must agree upon a maximum frame size. This constant de-
/// fines the minimum value to which the maximum frame size
/// can be set. By defining this value, the peers can guarantee
/// that they can send frames of up to this size until they have
/// agreed a definitive maximum frame size for that Connection.
pub const MIN_MAX_FRAME_SIZE: u32 = 512;


#[test]
fn test() {
    let x = Handle(1);
    let value = x.as_value();
    let y = Handle::from_value(value).unwrap();
    assert_eq!(x, y);
    let role = Role::Receiver;
    let value = role.as_value();
    dbg!(&value);
    let role2 = Role::from_value(value).unwrap();
    dbg!(role2);
}
