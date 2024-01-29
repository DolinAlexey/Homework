pub const VEC3_LEN: usize = 3;
pub struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
pub struct Pair(pub i32, pub i32);
pub struct SignedCounter(isize);
pub struct UnsignedCounter(usize);

impl SignedCounter {
    pub fn default_signed_counter() -> usize {
        0
    }
    pub fn next_signed(self) -> Self {
        SignedCounter(self.0 + 1)
    }

    pub fn prev_signed(self) -> Self {
        SignedCounter(self.0 - 1)
    }
}

impl UnsignedCounter {
    pub fn default_unsigned_counter() -> usize {
        0
    }
    pub fn next_unsigned(self) -> Self {
        UnsignedCounter(self.0 + 1)
    }
}

impl Vec3 {
    pub fn default_vec3() -> Vec3 {
        Vec3 { x: 0, y: 0, z: 0 }
    }
    pub fn vec3_vector_sum(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn pair_scalar_sum(&self, other: &Vec3) -> i32 {
        self.x + self.y + self.z + other.x + other.y + other.z
    }
}

impl Pair {
    pub fn default_pair() -> Pair {
        Pair(0, 0)
    }
    pub fn pair_vector_sum(&self, other: &Pair) -> Pair {
        Pair(self.0 + other.0, self.1 + other.1)
    }

    pub fn pair_scalar_sum(&self, other: &Pair) -> i32 {
        self.0 + self.1 + other.0 + other.1
    }
}
