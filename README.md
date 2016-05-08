# rust-dcc
A DCC protocol implementation in Rust.

DCC is defined by the National Model Railroad Association (NMRA) in [http://www.nmra.org/index-nmra-standards-and-recommended-practices Specifications S-9.1 and S-9.2]. It is a packet-based system, transmitted on a bus (typically the two rails of the model railway's track) by reversing the polarity of the power supply. This can be easily implemented using an H-Bridge DC motor controller IC (e.g. an L293).

The software is divided into various layers.

The top layer is the control layer. Commands (e.g. "Train 3 accelerate to 50% speed") are supplied to the control layer, which then schedules messages to be sent out on the bus.

The next layer is the message layer. This receives messages from the layer above, and emits packets as octet arrays to the layer below.

The bottom layer is the packet layer. This receives packets as octet arrays from the layer above, and emits a binary sequence using a given timer object.

The timer object allows the bottom layer to schedule output transitions with precise timing. It exists as a separate entity to allow for unit testing in a non-realtime environment.

