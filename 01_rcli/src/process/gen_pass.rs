use anyhow::Ok;
use rand::seq::SliceRandom;
use rand::thread_rng;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghjkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!\"#$%&'()*+,-./:;<=>?@[]^_{|}~";

// 避免参数和数据结构过分绑定
pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        // 对大大写的 O 和 0 在视觉上引发歧义的去除
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if lower {
        // 这里去除了小写 i 和小写的 l 避免视觉歧义
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

    // usize -> u8
    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        // 解引用的内容就会进行拷贝 u8 是支持拷贝的数据类型
        password.push(*c);
    }

    password.shuffle(&mut rng);

    // TODO: make sure the password has at least one of each type

    println!("{}", String::from_utf8(password)?);
    Ok(())
}
