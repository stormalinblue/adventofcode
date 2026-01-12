#[derive(Copy, Clone)]
struct MD5State {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

type Block<'a> = &'a [u32; 16];

impl MD5State {
    pub fn init() -> Self {
        MD5State {
            a: 0x67452301,
            b: 0xefcdab89,
            c: 0x98badcfe,
            d: 0x10325476,
        }
    }

    fn f(&self) -> u32 {
        (self.b & self.c) | (!self.b & self.c)
    }

    fn g(&self) -> u32 {
        (self.b & self.d) | (self.c & !self.d)
    }

    fn h(&self) -> u32 {
        self.b ^ self.c ^ self.d
    }

    fn i(&self) -> u32 {
        self.c ^ (self.b | !self.d)
    }

    fn consume_block<'a>(&self, block: Block<'a>) -> Self {
        let mut int = self.clone();
        for i in 0u32..64 {
            let (f, g): (u32, u32) = match i >> 4 {
                0 => (int.f(), i),
                1 => (int.g(), (5 * i + 1) % 16),
                2 => (int.h(), (3 * i + 5) % 16),
                3 => (int.i(), (7 * i) % 16),
                _ => unreachable!(),
            };

            let new_f = f + int.a + K[i as usize] + block[g as usize];
            int = Self {
                a: int.d,
                d: int.c,
                c: int.b,
                b: int.b + new_f.rotate_left(S[i as usize]),
            }
        }
        Self {
            a: self.a + int.a,
            b: self.b + int.b,
            c: self.c + int.c,
            d: self.d + int.d,
        }
    }
}

#[repr(align(4))]
struct AlignedBytes {
    bytes: [u8; 64],
}

const _: () = assert!(align_of::<AlignedBytes>() == align_of::<u32>());

struct MD5Hasher {
    state: MD5State,
    last_bytes: AlignedBytes,
    offset: usize,
    len_bytes: usize,
}

fn is_aligned(bytes: &[u8]) -> bool {
    (bytes.as_ptr() as usize) % align_of::<u32>() == 0
}

#[cfg(target_endian = "little")]
impl MD5Hasher {
    fn update(&mut self, bytes: &[u8]) -> () {
        let mut input_offset: usize = 0;

        let prev_offset = self.len_bytes % 64;

        if self.len_bytes % 64 + bytes.len() >= 64 {}

        if is_aligned(&bytes[input_offset..]) {
            while input_offset + 64 <= bytes.len() {
                self.state = unsafe {
                    let (_, block, _) = &bytes[input_offset..(input_offset + 64)].align_to::<u32>();
                    let new_state = self.state.consume_block((*block).try_into().unwrap());
                    new_state
                };

                input_offset += 64;
            }
        } else {
            while input_offset + 64 <= bytes.len() {
                self.last_bytes
                    .bytes
                    .copy_from_slice(&bytes[input_offset..(input_offset + 64)]);

                self.state = unsafe {
                    let (_, block, _) = &self.last_bytes.bytes.align_to::<u32>();
                    let new_state = self.state.consume_block((*block).try_into().unwrap());
                    new_state
                };
            }
        }

        self.len_bytes += bytes.len();
    }

    fn digest(&self) -> [u8; 16] {
        transmute
    }
}
