#![allow(dead_code,unused)]

// AES's constants
static SBOX: [u8; 256] = [
    0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,
    0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,
    0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15,
    0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75,
    0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84,
    0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF,
    0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8,
    0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5, 0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2,
    0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73,
    0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB,
    0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79,
    0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08,
    0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A,
    0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E,
    0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
    0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16,
];

static INV_SBOX: [u8; 256] = [
    0x52, 0x09, 0x6A, 0xD5, 0x30, 0x36, 0xA5, 0x38, 0xBF, 0x40, 0xA3, 0x9E, 0x81, 0xF3, 0xD7, 0xFB,
    0x7C, 0xE3, 0x39, 0x82, 0x9B, 0x2F, 0xFF, 0x87, 0x34, 0x8E, 0x43, 0x44, 0xC4, 0xDE, 0xE9, 0xCB,
    0x54, 0x7B, 0x94, 0x32, 0xA6, 0xC2, 0x23, 0x3D, 0xEE, 0x4C, 0x95, 0x0B, 0x42, 0xFA, 0xC3, 0x4E,
    0x08, 0x2E, 0xA1, 0x66, 0x28, 0xD9, 0x24, 0xB2, 0x76, 0x5B, 0xA2, 0x49, 0x6D, 0x8B, 0xD1, 0x25,
    0x72, 0xF8, 0xF6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xD4, 0xA4, 0x5C, 0xCC, 0x5D, 0x65, 0xB6, 0x92,
    0x6C, 0x70, 0x48, 0x50, 0xFD, 0xED, 0xB9, 0xDA, 0x5E, 0x15, 0x46, 0x57, 0xA7, 0x8D, 0x9D, 0x84,
    0x90, 0xD8, 0xAB, 0x00, 0x8C, 0xBC, 0xD3, 0x0A, 0xF7, 0xE4, 0x58, 0x05, 0xB8, 0xB3, 0x45, 0x06,
    0xD0, 0x2C, 0x1E, 0x8F, 0xCA, 0x3F, 0x0F, 0x02, 0xC1, 0xAF, 0xBD, 0x03, 0x01, 0x13, 0x8A, 0x6B,
    0x3A, 0x91, 0x11, 0x41, 0x4F, 0x67, 0xDC, 0xEA, 0x97, 0xF2, 0xCF, 0xCE, 0xF0, 0xB4, 0xE6, 0x73,
    0x96, 0xAC, 0x74, 0x22, 0xE7, 0xAD, 0x35, 0x85, 0xE2, 0xF9, 0x37, 0xE8, 0x1C, 0x75, 0xDF, 0x6E,
    0x47, 0xF1, 0x1A, 0x71, 0x1D, 0x29, 0xC5, 0x89, 0x6F, 0xB7, 0x62, 0x0E, 0xAA, 0x18, 0xBE, 0x1B,
    0xFC, 0x56, 0x3E, 0x4B, 0xC6, 0xD2, 0x79, 0x20, 0x9A, 0xDB, 0xC0, 0xFE, 0x78, 0xCD, 0x5A, 0xF4,
    0x1F, 0xDD, 0xA8, 0x33, 0x88, 0x07, 0xC7, 0x31, 0xB1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xEC, 0x5F,
    0x60, 0x51, 0x7F, 0xA9, 0x19, 0xB5, 0x4A, 0x0D, 0x2D, 0xE5, 0x7A, 0x9F, 0x93, 0xC9, 0x9C, 0xEF,
    0xA0, 0xE0, 0x3B, 0x4D, 0xAE, 0x2A, 0xF5, 0xB0, 0xC8, 0xEB, 0xBB, 0x3C, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2B, 0x04, 0x7E, 0xBA, 0x77, 0xD6, 0x26, 0xE1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0C, 0x7D,
];

static RCON: [u8; 32] = [
    0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36, 0x6C, 0xD8, 0xAB, 0x4D, 0x9A,
    0x2F, 0x5E, 0xBC, 0x63, 0xC6, 0x97, 0x35, 0x6A, 0xD4, 0xB3, 0x7D, 0xFA, 0xEF, 0xC5, 0x91, 0x39,
];

// helper utils
fn bytes2matrix(text: &Vec<u8>) -> Vec<[u8; 4]> {
    let mut matrix: Vec<[u8; 4]> = Vec::new();
    let n_cols = text.len() / 4;

    for i in 0..n_cols {
        let mut col = [0; 4];
        for j in 0..4 {
            col[j] = text[4*i + j];
        }
        matrix.push(col);
    }
    matrix
}

fn matrix2bytes(matrix: &Vec<[u8; 4]>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    for col in matrix.iter() {
        for cell in col.iter() {
            bytes.push(*cell);
        }
    }
    bytes
}

// key-expansion utils
fn rotword(word: &[u8; 4]) -> [u8; 4] {
    [word[1], word[2], word[3], word[0]]
}

fn subword(word: &[u8; 4]) -> [u8; 4] {
    [
        SBOX[word[0] as usize],
        SBOX[word[1] as usize],
        SBOX[word[2] as usize],
        SBOX[word[3] as usize],
    ]
}

fn xorword(word1: &[u8; 4], word2: &[u8; 4]) -> [u8; 4] {
    [
        word1[0] ^ word2[0],
        word1[1] ^ word2[1],
        word1[2] ^ word2[2],
        word1[3] ^ word2[3],
    ]
}

// AES's core algorithm

pub struct AES {
    key       : Vec<u8>      ,
    state     : Vec<[u8; 4]> ,
    n_rounds  : usize        ,
    round_keys: Vec<[u8; 4]> ,
}

impl AES {
    pub fn new(master_key: &[u8]) -> Self {
        let n_rounds = match master_key.len() {
            16 => 10,
            24 => 12,
            32 => 14,
            _ => panic!("Invalid key length"),
        };
        
        let mut aes = AES {
            key       : master_key.to_vec()  ,
            state     : Vec::with_capacity(4),
            n_rounds  : n_rounds             ,
            round_keys: Vec::new()           ,
        };

        aes.expand_key(&master_key.to_vec());
        aes
    }
    
    fn expand_key(&mut self, master_key: &Vec<u8>) {
        // init master_key to round_keys
        for row in bytes2matrix(&master_key).iter() {
            self.round_keys.push(*row);
        }

        let iter_size = master_key.len() / 4;
        let mut i = 1usize;

        // expand round_keys
        while self.round_keys.len() < 4 * (self.n_rounds + 1) {
            // get previous word
            let mut word: [u8; 4] = self.round_keys.last().unwrap().clone();

            // core key's schedule once every "row"
            if self.round_keys.len() % iter_size == 0 {
                // rotate word
                word = rotword(&word);
                // substitute bytes
                word = subword(&word);
                // xor word with RCON
                word = xorword(&word, &[RCON[i], 0, 0, 0]);

                i += 1;
            } else if master_key.len() == 32 && self.round_keys.len() % iter_size == 4 {
                // substitute bytes in the fourth iteration when using a 256-bits key.
                word = subword(&word);
                
            }
            // xor current word with previous word
            word = xorword(&word, &self.round_keys[self.round_keys.len() - iter_size]);
            self.round_keys.push(word);
        }
    }

    pub fn encrypt_block(&mut self, block: &Vec<u8>) -> Vec<u8> {
        assert!(block.len() == 16, "Block size must be 16 bytes");

        self.state = bytes2matrix(&block);
        self.add_round_key(0);

        for round in 1..self.n_rounds {
            self.sub_bytes();
            self.shift_rows();
            self.mix_columns();
            self.add_round_key(round);
        }

        self.sub_bytes();
        self.shift_rows();
        self.add_round_key(self.n_rounds);

        matrix2bytes(&self.state)
    }

    pub fn decrypt_block(&mut self, block: &Vec<u8>) -> Vec<u8> {
        self.state = bytes2matrix(&block);
        self.add_round_key(self.n_rounds);

        for round in (1..self.n_rounds).rev() {
            self.inv_shift_rows();
            self.inv_sub_bytes();
            self.add_round_key(round);
            self.inv_mix_columns();
        }

        self.inv_shift_rows();
        self.inv_sub_bytes();
        self.add_round_key(0);

        matrix2bytes(&self.state)
    }

    fn add_round_key(&mut self, round: usize) {
        for i in 0..4 {
            for j in 0..4 {
                self.state[i][j] ^= self.round_keys[round * 4 + i][j];
            }
        }
    }

    fn sub_bytes(&mut self) {
        for i in 0..4 {
            for j in 0..4 {
                self.state[i][j] = SBOX[self.state[i][j] as usize];
            }
        }
    }

    fn inv_sub_bytes(&mut self) {
        for i in 0..4 {
            for j in 0..4 {
                self.state[i][j] = INV_SBOX[self.state[i][j] as usize];
            }
        }
    }

    fn shift_rows(&mut self) {
        self.state = vec![
            [self.state[0][0], self.state[1][1], self.state[2][2], self.state[3][3]],
            [self.state[1][0], self.state[2][1], self.state[3][2], self.state[0][3]],
            [self.state[2][0], self.state[3][1], self.state[0][2], self.state[1][3]],
            [self.state[3][0], self.state[0][1], self.state[1][2], self.state[2][3]],
        ]
    }

    fn inv_shift_rows(&mut self) {
        self.state = vec![
            [self.state[0][0], self.state[3][1], self.state[2][2], self.state[1][3]],
            [self.state[1][0], self.state[0][1], self.state[3][2], self.state[2][3]],
            [self.state[2][0], self.state[1][1], self.state[0][2], self.state[3][3]],
            [self.state[3][0], self.state[2][1], self.state[1][2], self.state[0][3]],
        ]
    }


    fn mix_columns(&mut self) {
        // learned from https://web.archive.org/web/20100626212235/http://cs.ucsb.edu/~koc/cs178/projects/JT/aes.c
        let xtime = |a: u8| -> u8 {if a & 0x80 != 0 {((a << 1) ^ 0x1B) & 0xFF} else {a << 1}};
        
        // mix single column
        for i in 0..4 {
            // see Sec 4.1.2 in The Design of Rijndael
            let t = self.state[i][0] ^ self.state[i][1] ^ self.state[i][2] ^ self.state[i][3];
            let u = self.state[i][0];

            self.state[i] = [
                self.state[i][0] ^ t ^ xtime(self.state[i][0] ^ self.state[i][1]),
                self.state[i][1] ^ t ^ xtime(self.state[i][1] ^ self.state[i][2]),
                self.state[i][2] ^ t ^ xtime(self.state[i][2] ^ self.state[i][3]),
                self.state[i][3] ^ t ^ xtime(self.state[i][3] ^ u),
            ]
        }
    }

    fn inv_mix_columns(&mut self) {
        let xtime = |a: u8| -> u8 {if a & 0x80 != 0 {((a << 1) ^ 0x1B) & 0xFF} else {a << 1}};

        // see Sec 4.1.3 in The Design of Rijndael
        for i in 0..4 {
            let u = xtime(xtime(self.state[i][0] ^ self.state[i][2]));
            let v = xtime(xtime(self.state[i][1] ^ self.state[i][3]));

            self.state[i][0] ^= u;
            self.state[i][1] ^= v;
            self.state[i][2] ^= u;
            self.state[i][3] ^= v;
        }

        self.mix_columns();
    }
}