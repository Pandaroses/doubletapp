use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

#[wasm_bindgen]
pub struct SplitMix {
    seed: u64,
}

#[wasm_bindgen]
impl SplitMix {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: Option<u64>) -> Self {
        SplitMix {
            seed: seed.unwrap_or_else(|| 0),
        }
    }

    pub fn next(&mut self) -> u64 {
        self.seed = self.seed.wrapping_add(0x9e3779b97f4a7c15);
        let mut z: u64 = self.seed;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}

#[wasm_bindgen]
pub struct Xoshiro256plus {
    seed: [u64; 4],
}

#[wasm_bindgen]
impl Xoshiro256plus {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: Option<u64>) -> Self {
        let mut rng = SplitMix::new(seed);
        Xoshiro256plus {
            seed: [rng.next(), rng.next(), rng.next(), rng.next()],
        }
    }

    pub fn next(&mut self) -> f64 {
        let result = self.seed[0].wrapping_add(self.seed[3]);
        let t = self.seed[1] << 17;

        self.seed[2] ^= self.seed[0];
        self.seed[3] ^= self.seed[1];
        self.seed[1] ^= self.seed[2];
        self.seed[0] ^= self.seed[3];

        self.seed[2] ^= t;
        self.seed[3] = Self::rol64(self.seed[3], 45);

        (result >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
    pub fn get_seed(&self) -> String {
        format!("{:?}", self.seed)
    }
    fn rol64(x: u64, k: i32) -> u64 {
        (x << k) | (x >> (64 - k))
    }
}
