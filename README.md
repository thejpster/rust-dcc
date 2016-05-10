# rust-dcc
A DCC protocol implementation in Rust.

DCC is defined by the National Model Railroad Association (NMRA) in [http://www.nmra.org/index-nmra-standards-and-recommended-practices Specifications S-9.1 and S-9.2]. It is a packet-based system, transmitted on a bus (typically the two rails of the model railway's track) by reversing the polarity of the power supply. This can be easily implemented using an H-Bridge DC motor controller IC (e.g. an L293).

To aid compatibility with existing desktop software, rust-dcc will implement the Arduino DCC++ protocol set.

* `<t REGISTER CAB SPEED DIRECTION>`: sets the throttle for a mobile engine decoder using 128-step speeds
* `<f CAB BYTE1 [BYTE2]>`: controls mobile engine decoder functions F0-F28
* `<a ADDRESS SUBADDRESS ACTIVATE>`: controls stationary accessory decoders
* `<T ID THROW>`: controls turnouts connected to stationary accessory decoders
* `<w CAB CV VALUE>`: writes a configuration variable byte to an engine decoder on the main ops track
* `<b CAB CV BIT VALUE>`: sets/clear a configuration variable bit in an engine decoder on the main operations track
* `<W CV VALUE CALLBACKNUM CALLBACKSUB>`: writes a configuration variable byte to an engine decoder on the programming track
* `<B CV BIT VALUE CALLBACKNUM CALLBACKSUB>`: sets/clear a configuration variable bit in an engine decoder on the programming track
* `<R CV CALLBACKNUM CALLBACKSUB>`: reads a configuration variable byte from an engine decoder on the programming track
* `<1>`: turns on track power
* `<0>`: turns off track power
* `<c>`: reads current draw from main operations track
* `<s>`: returns status messages, including power state, turnout states, and sketch version

see https://github.com/DccPlusPlus/BaseStation for more details.

The software is divided into various layers in two stacks - the PC stack and the DCC stack.

At the bottom of the PC stack is the UART.

Next up on the PC stack is the command parser. Bytes come in from below and Commands go up.

At the top is the Application. This receives Commands from the command parser and sends Messages to the packet layer.

Heading down the DCC stack, below the Application is the packet layer. This receives Messages and emits packets containing arrays of bits into specified registers.

Next is the Bitstream layer, which encodes the packets as a bitstream with the correct timing. This makes use of a Timing object. It buffers a series of packets which are sent out in sequence.

The timer object allows the bottom layer to schedule output transitions with precise timing. It exists as a separate entity to allow for unit testing in a non-realtime environment.

Messages pass through the stack like this:

              +--------------------------+
              |      Application         |
              |          ^>>>v           |
    Commands  +------------+-------------+ Packets
              | Command  ^ | v Packet    |
       bytes  +------------+-------------+ bits
              | UART     ^ | v Bitstream |
              +------------+-------------+
        RS232                              DCC
