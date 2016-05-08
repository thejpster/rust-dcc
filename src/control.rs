//! This is the Control layer for rust-dcc. Commands (e.g. "Train 3 accelerate
//! to 50% speed") are supplied to the control layer, which then schedules
//! messages to be sent out on the bus.

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use std::collections::BTreeMap;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

/// The Controller object receives command and emits messages to the supplied
/// message handler. When strobed, it will send a stopping packet if required, else
/// it will send the next moving packet in the sequence. Moving packets are sent
/// repeatedly, in case the DCC decoder browns-out and forgets its current state.
pub struct Controller<'a> {
    /// Contains all of the currently moving Locomotives and their speeds, ordered
    /// by address.
    moving: BTreeMap<Address, Move>,
    /// Contains all of the vehicles we need to stop, and how many times
    /// the stop command should be sent, ordered by address. We don't send
    /// these continuously, and they take priority over moving packets.
    stopping: BTreeMap<Address, Retries>,
    /// The last address we commanded.
    last_address: Option<Address>,
    /// We give our Controller a name, for debug
    name: &'a str
}

/// Commands are operations, sent to a specific Locomotive
pub struct Command<'a> {
    device: &'a Locomotive,
    operation: Operation
}

/// Represents a decoder on the bus
pub struct Locomotive<'a> {
    name: &'a str,
    address: Address
}

type Address = u8;

#[derive(Copy, Clone)]
pub enum Direction {
    Forwards,
    Backwards
}

/// Operations are the things we can make a Locomotive do.
#[derive(Copy, Clone)]
pub enum Operation {
    Stop,
    EmergencyStop,
    Move(Direction, Speed),
}

#[derive(Copy, Clone)]
pub struct Speed(u8);

type Retries = u16;

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

impl<'a> Controller<'a> {
    pub fn new(name: &'a str) -> Controller<'a> {
        Controller {
            name: name
        }
    }

    pub fn name(&self) -> &'a str {
        return self.name
    }

    pub fn submit(&mut self, command: &Command, retries: Retries) {

    }

    /// Send the next command which needs sending
    pub fn strobe(&mut self) {

    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_name() {
        let name = "Test Controller";
        let c = Controller::new(name);
        if c.name != name {
            panic!("Name mismatch");
        }
    }
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
