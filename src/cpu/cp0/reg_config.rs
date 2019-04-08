#[derive(Debug, Default)]
pub struct RegConfig {
    // EP
    data_transfer_pattern: DataTransferPattern,

    // BE
    endianness: Endianness,

    // CU
    cu: bool,
    kseg0_cache_enabled: bool,
}

impl RegConfig {
    pub fn power_on_reset(&mut self) {
        self.data_transfer_pattern = DataTransferPattern::Normal;
        self.endianness = Endianness::Big;
    }
}

impl From<u32> for RegConfig {
    fn from(data: u32) -> Self {
        RegConfig {
            data_transfer_pattern: data.into(),
            endianness: data.into(),
            cu: (data & (1 << 3)) != 0,
            kseg0_cache_enabled: data & 0b111 != 0b010,
        }
    }
}

#[derive(Debug)]
enum DataTransferPattern {
    Normal, // D
    DxxDxx,
}

impl Default for DataTransferPattern {
    fn default() -> Self {
        DataTransferPattern::Normal
    }
}

impl From<u32> for DataTransferPattern {
    fn from(data: u32) -> Self {
        match (data >> 24) & 0b1111 {
            0 => DataTransferPattern::Normal,
            6 => DataTransferPattern::DxxDxx,
            _ => panic!("Invalid data transfer pattern (EP): {:#x}", data),
        }
    }
}

#[derive(Debug)]
enum Endianness {
    Little,
    Big,
}

impl Default for Endianness {
    fn default() -> Self {
        Endianness::Big
    }
}

impl From<u32> for Endianness {
    fn from(value: u32) -> Self {
        match (value >> 15) & 0b1 {
            0 => Endianness::Little,
            1 => Endianness::Big,
            _ => unreachable!(),
        }
    }
}
