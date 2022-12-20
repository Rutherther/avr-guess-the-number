pub struct Rng {
    a: u8,
    b: u8,
    c: u8,
    x: u8
}

impl Rng {
    pub fn init(s1: u8, s2: u8, s3: u8) -> Rng {
        let mut rng = Rng {
            a: 0,
            b: 0,
            c: 0,
            x: 0
        };

        rng.a ^= s1;
        rng.b ^= s2;
        rng.c ^= s3;
        rng.randomize();

        rng
    }

    fn randomize(&mut self) -> u8 {
        self.x += 1;
        self.a = self.a^self.c^self.x;
        self.b = self.b+self.a;
        self.c = self.c + (self.b >> 1)^self.a;

        self.c
    }

    #[inline]
    pub fn take_u8(&mut self) -> u8 {
        self.randomize()
    }

    pub fn take_u16(&mut self) -> u16 {
        let first: u16 = self.randomize().into();
        let second: u16 = self.randomize().into();

        first << 8 | second
    }
}
