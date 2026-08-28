pub fn i32_to_null_terminated_bytes(val: i32) -> [u8; 12] {
    let mut buf = [0u8; 12];

    let mut temp = [0u8; 11];
    let mut len = 0;

    let mut n = val.unsigned_abs();

    if n == 0 {
        temp[0] = b'0';
        len = 1;
    } else {
        while n > 0 {
            temp[len] = b'0' + (n % 10) as u8;
            n /= 10;
            len += 1;
        }
    }

    for i in 0..len {
        buf[i] = temp[len - 1 - i];
    }

    buf
}
