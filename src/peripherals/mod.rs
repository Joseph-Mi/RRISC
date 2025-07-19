// Basic peripheral simulation for STM32G474VET6
pub struct Timer {
    pub counter: u32,
    pub period: u32,
    pub enabled: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            counter: 0,
            period: 1000,
            enabled: false,
        }
    }

    pub fn tick(&mut self) -> bool {
        if self.enabled {
            self.counter += 1;
            if self.counter >= self.period {
                self.counter = 0;
                true // Overflow/match occurred
            } else {
                false
            }
        } else {
            false
        }
    }
}

pub struct Uart {
    pub tx_buffer: [u8; 256],
    pub rx_buffer: [u8; 256],
    pub tx_head: usize,
    pub tx_tail: usize,
    pub rx_head: usize,
    pub rx_tail: usize,
}

impl Uart {
    pub fn new() -> Self {
        Self {
            tx_buffer: [0; 256],
            rx_buffer: [0; 256],
            tx_head: 0,
            tx_tail: 0,
            rx_head: 0,
            rx_tail: 0,
        }
    }

    pub fn send_byte(&mut self, byte: u8) -> bool {
        let next_head = (self.tx_head + 1) % self.tx_buffer.len();
        if next_head != self.tx_tail {
            self.tx_buffer[self.tx_head] = byte;
            self.tx_head = next_head;
            true
        } else {
            false // Buffer full
        }
    }

    pub fn receive_byte(&mut self) -> Option<u8> {
        if self.rx_head != self.rx_tail {
            let byte = self.rx_buffer[self.rx_tail];
            self.rx_tail = (self.rx_tail + 1) % self.rx_buffer.len();
            Some(byte)
        } else {
            None
        }
    }
}

pub struct Gpio {
    pub pins: [bool; 16], // 16 GPIO pins per port
}

impl Gpio {
    pub fn new() -> Self {
        Self { pins: [false; 16] }
    }

    pub fn set_pin(&mut self, pin: u8, state: bool) {
        if (pin as usize) < self.pins.len() {
            self.pins[pin as usize] = state;
        }
    }

    pub fn get_pin(&self, pin: u8) -> bool {
        if (pin as usize) < self.pins.len() {
            self.pins[pin as usize]
        } else {
            false
        }
    }
}
