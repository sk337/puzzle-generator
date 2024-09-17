

#[derive(Debug)]
pub struct Prand {
    seed: u64,
}

impl Prand {
    pub fn new(seed: u64) -> Prand {
        Prand { seed }
    }

    pub fn get_rand_int_with_max(&mut self, max: u64) -> u64 {
        let mut x = self.seed;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.seed = x;
        // println!("{}:{}", max, (x % max)+1);
        (x % max)+1
    }

    // pub fn rand_f64(&mut self) -> f64 {
    //     self.get_rand_int_with_max(0xffffffffffffffff) as f64 / 0xffffffffffffffffu64 as f64
    // }

    // pub fn rand_f32(&mut self) -> f32 {
    //     self.get_rand_int_with_max(0xffffffff) as f32 / 0xffffffffu32 as f32
    // }

    // pub fn gen_range(&mut self, min: u64, max: u64) -> u64 {
    //     self.get_rand_int_with_max(max - min) + min
    // }
}
