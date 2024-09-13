use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;
//&[u8] 是一个不可变的字节切片（slice）类型
//&[u8] 是对字节序列的引用
const UPPER: &[u8] = b"ABCDEFGHJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::with_capacity(length as usize);
    let mut chars = Vec::new();
    //去掉了引起视觉歧义的字符
    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
    }
    for _ in 0..(length - password.len() as u8) {
        let c = chars.choose(&mut rng).expect("chars won't be empty");
        password.push(*c);
    }
    password.shuffle(&mut rng);
    let p = String::from_utf8(password)?;
    println!("{}", p);
    let estimate = zxcvbn(&p, &[]);
    eprintln!("{}", estimate.score()); // 3
    Ok(())
}
