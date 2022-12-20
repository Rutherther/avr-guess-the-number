pub struct Eeprom {
    eeprom: avr_device::atmega8::EEPROM,
}

impl Eeprom {
    pub fn new(ep: avr_device::atmega8::EEPROM) -> Eeprom {
        Eeprom {
            eeprom: ep
        }
    }

    fn wait_until_write(&self) {
        while self.eeprom.eecr.read().eewe().bit_is_set() {}
    }

    pub fn read(&self, address: u16, data: &mut [u8]) {
        let mut address = address;
        for i in 0..data.len() {
            let byte = self.read_byte(address);
            data[i] = byte;
            address += 1;
        }
    }

    pub fn write(&self, address: u16, data: &[u8]) {
        let mut address = address;
        for byte in data.iter() {
            self.write_byte(address, *byte);
            address += 1;
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.wait_until_write();

        self.eeprom.eear.write(|w| unsafe {
            w.bits(address)
        });

        self.eeprom.eecr.write(|w| w.eere().set_bit());
        self.eeprom.eedr.read().bits()
    }

    pub fn write_byte(&self, address: u16, data: u8) {
        self.wait_until_write();

        self.eeprom.eear.write(|w| unsafe {
            w.bits(address)
        });
        self.eeprom.eedr.write(|w| unsafe {
            w.bits(data)
        });

        self.eeprom.eecr.write(|w|
            w
                .eemwe().set_bit()
                .eewe().clear_bit());

        self.eeprom.eecr.write(|w|  w.eewe().set_bit());
    }
}
