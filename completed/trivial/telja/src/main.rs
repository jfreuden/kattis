fn read_i32() -> i32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<i32>().unwrap()
}

fn main() {
    use std::io::Write;

    let value = read_i32();

    let mut charholder = Vec::<u8>::with_capacity(1024 * 1024 * 64);
    for answer in 1..value + 1 {
        if answer.is_negative() {
            charholder.push(b'-');
        } else if answer == 0 {
            charholder.push(b'0');
            charholder.push(b'\n');
            continue;
        }

        let mut temp = answer.unsigned_abs();
        let digit_count = (temp.checked_ilog10().unwrap() + 1) as usize;
        let mut offset = 1usize;
        charholder.append(&mut vec![b' '; digit_count]);
        let end = charholder.len();
        while temp > 0 {
            let modulo = (temp % 10) as u8;
            temp /= 10;
            charholder[end - offset] = b'0' + modulo;
            offset += 1;
        }
        charholder.push(b'\n');
    }

    std::io::stdout().write_all(charholder.as_slice()).unwrap();
    std::io::stdout().flush().unwrap();
}
