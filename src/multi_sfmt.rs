use std::{
    ops::Not,
    simd::{u32x8, u64x8, Simd},
};

pub struct MultiSFMT {
    idx: usize,
    state: [u32x8; 624],
}

impl Default for MultiSFMT {
    fn default() -> MultiSFMT {
        MultiSFMT {
            idx: 0,
            state: [Simd::splat(0); 624],
        }
    }
}

impl MultiSFMT {
    pub fn init(&mut self, seed: u32x8) {
        self.idx = 624;
        self.state[0] = seed;
        for i in 1..624 {
            self.state[i] = Simd::splat(0x6c078965)
                * (self.state[i - 1] ^ (self.state[i - 1] >> Simd::splat(30)))
                + Simd::splat(i as u32);
        }

        let mut inner = seed & Simd::splat(1);
        inner ^= self.state[3] & Simd::splat(0x13c9e684);
        inner ^= inner >> Simd::splat(16);
        inner ^= inner >> Simd::splat(8);
        inner ^= inner >> Simd::splat(4);
        inner ^= inner >> Simd::splat(2);
        inner ^= inner >> Simd::splat(1);

        self.state[0] ^= Simd::not(inner) & Simd::splat(1);
    }

    pub fn advance(&mut self, advances: usize) {
        self.idx += advances * 2;
        while self.idx > 624 {
            self.shuffle();
            self.idx -= 624;
        }
    }

    pub fn next_needle(&mut self) -> u64x8 {
        if self.idx == 624 {
            self.shuffle();
            self.idx = 0;
        }

        let low = self.state[self.idx];
        let high = self.state[self.idx + 1];
        self.idx += 2;

        Simd::from_array([
            ((high[0] as u64) << 32) | (low[0] as u64),
            ((high[1] as u64) << 32) | (low[1] as u64),
            ((high[2] as u64) << 32) | (low[2] as u64),
            ((high[3] as u64) << 32) | (low[3] as u64),
            ((high[4] as u64) << 32) | (low[4] as u64),
            ((high[5] as u64) << 32) | (low[5] as u64),
            ((high[6] as u64) << 32) | (low[6] as u64),
            ((high[7] as u64) << 32) | (low[7] as u64),
        ]) % Simd::splat(17)
    }

    fn shuffle(&mut self) {
        let mut a = 0;
        let mut b = 488;
        let mut c = 616;
        let mut d = 620;

        while a < 136 {
            self.state[a + 3] ^= (self.state[a + 3] << Simd::splat(8))
                ^ (self.state[a + 2] >> Simd::splat(24))
                ^ (self.state[c + 3] >> Simd::splat(8))
                ^ ((self.state[b + 3] >> Simd::splat(11)) & Simd::splat(0xbffffff6))
                ^ (self.state[d + 3] << Simd::splat(18));
            self.state[a + 2] ^= (self.state[a + 2] << Simd::splat(8))
                ^ (self.state[a + 1] >> Simd::splat(24))
                ^ (self.state[c + 3] << Simd::splat(24))
                ^ (self.state[c + 2] >> Simd::splat(8))
                ^ ((self.state[b + 2] >> Simd::splat(11)) & Simd::splat(0xbffaffff))
                ^ (self.state[d + 2] << Simd::splat(18));
            self.state[a + 1] ^= (self.state[a + 1] << Simd::splat(8))
                ^ (self.state[a] >> Simd::splat(24))
                ^ (self.state[c + 2] << Simd::splat(24))
                ^ (self.state[c + 1] >> Simd::splat(8))
                ^ ((self.state[b + 1] >> Simd::splat(11)) & Simd::splat(0xddfecb7f))
                ^ (self.state[d + 1] << Simd::splat(18));
            self.state[a] ^= (self.state[a] << Simd::splat(8))
                ^ (self.state[c + 1] << Simd::splat(24))
                ^ (self.state[c] >> Simd::splat(8))
                ^ ((self.state[b] >> Simd::splat(11)) & Simd::splat(0xdfffffef))
                ^ (self.state[d] << Simd::splat(18));

            c = d;
            d = a;
            a += 4;
            b += 4;
        }

        b = 0;
        while a < 624 {
            self.state[a + 3] ^= (self.state[a + 3] << Simd::splat(8))
                ^ (self.state[a + 2] >> Simd::splat(24))
                ^ (self.state[c + 3] >> Simd::splat(8))
                ^ ((self.state[b + 3] >> Simd::splat(11)) & Simd::splat(0xbffffff6))
                ^ (self.state[d + 3] << Simd::splat(18));
            self.state[a + 2] ^= (self.state[a + 2] << Simd::splat(8))
                ^ (self.state[a + 1] >> Simd::splat(24))
                ^ (self.state[c + 3] << Simd::splat(24))
                ^ (self.state[c + 2] >> Simd::splat(8))
                ^ ((self.state[b + 2] >> Simd::splat(11)) & Simd::splat(0xbffaffff))
                ^ (self.state[d + 2] << Simd::splat(18));
            self.state[a + 1] ^= (self.state[a + 1] << Simd::splat(8))
                ^ (self.state[a] >> Simd::splat(24))
                ^ (self.state[c + 2] << Simd::splat(24))
                ^ (self.state[c + 1] >> Simd::splat(8))
                ^ ((self.state[b + 1] >> Simd::splat(11)) & Simd::splat(0xddfecb7f))
                ^ (self.state[d + 1] << Simd::splat(18));
            self.state[a] ^= (self.state[a] << Simd::splat(8))
                ^ (self.state[c + 1] << Simd::splat(24))
                ^ (self.state[c] >> Simd::splat(8))
                ^ ((self.state[b] >> Simd::splat(11)) & Simd::splat(0xdfffffef))
                ^ (self.state[d] << Simd::splat(18));

            c = d;
            d = a;
            a += 4;
            b += 4;
        }
    }
}
