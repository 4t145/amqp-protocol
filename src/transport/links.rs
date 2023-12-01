//!
//! 
//! A Link provides a unidirectional transport for Messages between a Source and a Target. The primary
//! responsibility of a Source or Target (a Terminus) is to maintain a record of the status of each active
//! delivery attempt until such a time as it is safe to forget. These are referred to as unsettled deliveries.
//! When a Terminus forgets the state associated with a delivery-tag, it is considered settled. Each delivery
//! attempt is assigned a unique delivery-tag at the Source. The status of an active delivery attempt is
//! known as the Delivery State of the delivery.
//! 
//! 


pub struct Links {
    source: String,
    target: String,
    name: String,
}