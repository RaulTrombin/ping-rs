#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use cortex_m_rt::entry;
use stm32f4xx_hal::{
    pac,
    prelude::*,
    serial::{config::Config, Serial, Event, Tx},
    gpio::{Alternate, gpioa::{PA2, PA3}},
};
use bluerobotics_ping_core::{
    decoder::{Decoder, DecoderResult},
    message::ProtocolMessage,
};

// Device configuration
const DEVICE_ID: u8 = 1;

// Allocator setup for no_std
use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

const HEAP_SIZE: usize = 1024;
static mut HEAP_MEM: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

struct RingBuffer {
    buffer: [u8; 256],
    write_pos: usize,
    read_pos: usize,
}

impl RingBuffer {
    const fn new() -> Self {
        Self {
            buffer: [0; 256],
            write_pos: 0,
            read_pos: 0,
        }
    }

    fn push(&mut self, byte: u8) {
        self.buffer[self.write_pos] = byte;
        self.write_pos = (self.write_pos + 1) & 255;
    }

    fn pop(&mut self) -> Option<u8> {
        if self.read_pos != self.write_pos {
            let byte = self.buffer[self.read_pos];
            self.read_pos = (self.read_pos + 1) & 255;
            Some(byte)
        } else {
            None
        }
    }
}

static mut RING_BUFFER: RingBuffer = RingBuffer::new();

type TxPin = PA2<Alternate<7>>;
type RxPin = PA3<Alternate<7>>;
type UartTx = Tx<pac::USART2>;

struct SimpleDevice {
    tx: UartTx,
}

impl SimpleDevice {
    fn new(tx: UartTx) -> Self {
        Self { tx }
    }

    fn handle_message(&mut self, message: &ProtocolMessage) {
        // Echo back any received message with our device ID
        let mut response = ProtocolMessage::new();
        response.message_id = message.message_id;
        response.src_device_id = DEVICE_ID;
        response.dst_device_id = message.src_device_id;
        response.payload = message.payload.clone();
        response.payload_length = message.payload_length;
        response.update_checksum();

        self.send_message(response);
    }

    fn send_message(&mut self, message: ProtocolMessage) {
        let data = message.serialized();
        for byte in data {
            let _ = nb::block!(self.tx.write(byte));
        }
    }
}

#[entry]
fn main() -> ! {
    unsafe {
        HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE);
    }

    let dp = pac::Peripherals::take().unwrap();
    let _cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

    let gpioa = dp.GPIOA.split();
    let tx_pin: TxPin = gpioa.pa2.into_alternate();
    let rx_pin: RxPin = gpioa.pa3.into_alternate();

    let mut serial = Serial::new(
        dp.USART2,
        (tx_pin, rx_pin),
        Config::default().baudrate(115200.bps()),
        &clocks,
    ).unwrap();

    serial.listen(Event::Rxne);

    let (tx, mut rx) = serial.split();
    let mut device = SimpleDevice::new(tx);
    let mut decoder = Decoder::new();

    loop {
        if let Ok(byte) = nb::block!(rx.read()) {
            unsafe { RING_BUFFER.push(byte) };
        }

        while let Some(byte) = unsafe { RING_BUFFER.pop() } {
            match decoder.parse_byte(byte) {
                DecoderResult::Success(message) => {
                    device.handle_message(&message);
                }
                DecoderResult::Error(_) => {
                    decoder = Decoder::new();
                }
                DecoderResult::InProgress => {}
            }
        }
    }
}