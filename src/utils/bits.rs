pub fn bytes_to_bits(data: &[u8]) -> Vec<u8> {
    data.to_vec().iter().map(|x: &u8| u8_to_bit(*x)).flatten().collect()
}

pub fn u8_to_bit(n: u8) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    let mut temp = n;

    for _ in 0..8 {
        res.push(temp & 1);
        temp >>= 1;
    }
    res.reverse();
    res
}


pub fn vec8_to_str(v: &Vec<u8>) -> String {
    let mut res: String = String::new();
    let mut character: u8 = 0;

    for byte in v.chunks(8) {
        for bit in byte {
            character <<= 1;
            character |= bit;
        }
        res.push(character as char);
        character = 0;
    }

    res
}