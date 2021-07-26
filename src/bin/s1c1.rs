fn repr(num: u32) -> char {
    let ascii = if num < 26 {
        'A' as u32 + num
    } else if num < 52 {
        'a' as u32 + num - 26
    } else if num < 62 {
        '0' as u32 + num - 52
    } else if num == 62 {
        '+' as u32
    } else if num == 63 {
        '/' as u32
    } else {
        unreachable!("Can only represent 0-63! {}", num);
    };
    char::from_u32(ascii).unwrap()
}

fn hex2decimal(ch: char) -> u32 {
    let num = ch as u32;
    if ch >= '0' && ch <= '9' {
        num - '0' as u32
    } else if ch >= 'a' && ch <= 'f' {
        num - 'a' as u32 + 10
    } else {
        unreachable!("Can only convert 0-f! {}", ch);
    }
}

fn hex2base64(word: &str) -> String {
    let mut result = Vec::with_capacity(word.len() * 2 / 3);
    let mut residue: u32 = 0;
    let len = word.len();
    for (i, ch) in word.chars().rev().enumerate() {
        residue += (hex2decimal(ch) as u32) << ((i % 3) * 4);
        if i % 3 == 2 || i + 1 == len {
            result.push(repr(residue % 64));
            result.push(repr(residue / 64));
            residue = 0;
        }
    }

    result.reverse();
    result.into_iter().collect()
}


fn main() {
    println!("{}", hex2base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
}
