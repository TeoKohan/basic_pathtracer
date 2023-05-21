use std::num::Wrapping;

pub struct PCGR {
    pub state: Wrapping<u64>,
    pub inc: Wrapping<u64>,
}

impl PCGR {
    fn next(&mut self) -> u64 {
        let oldstate: Wrapping<u64> = self.state;
        // Advance internal state
        self.state = oldstate * Wrapping(6364136223846793005_u64) + Wrapping(self.inc.0 | 1);
        // Calculate output function (XSH RR), uses old state for max ILP
        let xorshifted : Wrapping<u64> = Wrapping(((oldstate.0 >> 18_u16) ^ oldstate.0) >> 27_u16);
        let rot: Wrapping<u64> = Wrapping(oldstate.0 >> 59_u16);
        (xorshifted.0 >> rot.0) | (xorshifted.0 << ((-(rot.0 as i128)) & 31))
    }

    pub fn random(&mut self) -> f32 {
        self.next() as f32 / std::u64::MAX as f32      
    }
}
