static TABLE: [u64; 256] = [
    0x39cb44b8, 0x23754f67, 0x5f017211, 0x3ebb24da, 0x351707c6, 0x63f9774b, 0x17827288, 0x0fe74821,
    0x5b5f670f, 0x48315ae8, 0x785b7769, 0x2b7a1547, 0x38d11292, 0x42a11b32, 0x35332244, 0x77437b60,
    0x1eab3b10, 0x53810000, 0x1d0212ae, 0x6f0377a8, 0x43c03092, 0x2d3c0a8e, 0x62950cbf, 0x30f06ffa,
    0x34f710e0, 0x28f417fb, 0x350d2f95, 0x5a361d5a, 0x15cc060b, 0x0afd13cc, 0x28603bcf, 0x3371066b,
    0x30cd14e4, 0x175d3a67, 0x6dd66a13, 0x2d3409f9, 0x581e7b82, 0x76526b99, 0x5c8d5188, 0x2c857971,
    0x15f51fc0, 0x68cc0d11, 0x49f55e5c, 0x275e4364, 0x2d1e0dbc, 0x4cee7ce3, 0x32555840, 0x112e2e08,
    0x6978065a, 0x72921406, 0x314578e7, 0x175621b7, 0x40771dbf, 0x3fc238d6, 0x4a31128a, 0x2dad036e,
    0x41a069d6, 0x25400192, 0x00dd4667, 0x6afc1f4f, 0x571040ce, 0x62fe66df, 0x41db4b3e, 0x3582231f,
    0x55f6079a, 0x1ca70644, 0x1b1643d2, 0x3f7228c9, 0x5f141070, 0x3e1474ab, 0x444b256e, 0x537050d9,
    0x0f42094b, 0x2fd820e6, 0x778b2e5e, 0x71176d02, 0x7fea7a69, 0x5bb54628, 0x19ba6c71, 0x39763a99,
    0x178d54cd, 0x01246e88, 0x3313537e, 0x2b8e2d17, 0x2a3d10be, 0x59d10582, 0x37a163db, 0x30d6489a,
    0x6a215c46, 0x0e1c7a76, 0x1fc760e7, 0x79b80c65, 0x27f459b4, 0x799a7326, 0x50ba1782, 0x2a116d5c,
    0x63866e1b, 0x3f920e3c, 0x55023490, 0x55b56089, 0x2c391fd1, 0x2f8035c2, 0x64fd2b7a, 0x4ce8759a,
    0x518504f0, 0x799501a8, 0x3f5b2cad, 0x38e60160, 0x637641d8, 0x33352a42, 0x51a22c19, 0x085c5851,
    0x032917ab, 0x2b770ac7, 0x30ac77b3, 0x2bec1907, 0x035202d0, 0x0fa933d3, 0x61255df3, 0x22ad06bf,
    0x58b86971, 0x5fca0de5, 0x700d6456, 0x56a973db, 0x5ab759fd, 0x330e0be2, 0x5b3c0ddd, 0x495d3c60,
    0x53bd59a6, 0x4c5e6d91, 0x49d9318d, 0x103d5079, 0x61ce42e3, 0x7ed5121d, 0x14e160ed, 0x212d4ef2,
    0x270133f0, 0x62435a96, 0x1fa75e8b, 0x6f092fbe, 0x4a000d49, 0x57ae1c70, 0x004e2477, 0x561e7e72,
    0x468c0033, 0x5dcc2402, 0x78507ac6, 0x58af24c7, 0x0df62d34, 0x358a4708, 0x3cfb1e11, 0x2b71451c,
    0x77a75295, 0x56890721, 0x0fef75f3, 0x120f24f1, 0x01990ae7, 0x339c4452, 0x27a15b8e, 0x0ba7276d,
    0x60dc1b7b, 0x4f4b7f82, 0x67db7007, 0x4f4a57d9, 0x621252e8, 0x20532cfc, 0x6a390306, 0x18800423,
    0x19f3778a, 0x462316f0, 0x56ae0937, 0x43c2675c, 0x65ca45fd, 0x0d604ff2, 0x0bfd22cb, 0x3afe643b,
    0x3bf67fa6, 0x44623579, 0x184031f8, 0x32174f97, 0x4c6a092a, 0x5fb50261, 0x01650174, 0x33634af1,
    0x712d18f4, 0x6e997169, 0x5dab7afe, 0x7c2b2ee8, 0x6edb75b4, 0x5f836fb6, 0x3c2a6dd6, 0x292d05c2,
    0x052244db, 0x149a5f4f, 0x5d486540, 0x331d15ea, 0x4f456920, 0x483a699f, 0x3b450f05, 0x3b207c6c,
    0x749d70fe, 0x417461f6, 0x62b031f1, 0x2750577b, 0x29131533, 0x588c3808, 0x1aef3456, 0x0f3c00ec,
    0x7da74742, 0x4b797a6c, 0x5ebb3287, 0x786558b8, 0x00ed4ff2, 0x6269691e, 0x24a2255f, 0x62c11f7e,
    0x2f8a7dcd, 0x643b17fe, 0x778318b8, 0x253b60fe, 0x34bb63a3, 0x5b03214f, 0x5f1571f4, 0x1a316e9f,
    0x7acf2704, 0x28896838, 0x18614677, 0x1bf569eb, 0x0ba85ec9, 0x6aca6b46, 0x1e43422a, 0x514d5f0e,
    0x413e018c, 0x307626e9, 0x01ed1dfa, 0x49f46f5a, 0x461b642b, 0x7d7007f2, 0x13652657, 0x6b160bc5,
    0x65e04849, 0x1f526e1c, 0x5a0251b6, 0x2bd73f69, 0x2dbf7acd, 0x51e63e80, 0x5cf2670f, 0x21cd0a03,
    0x5cff0261, 0x33ae061e, 0x3bb6345f, 0x5d814a75, 0x257b5df4, 0x0a5c2c5b, 0x16a45527, 0x16f23945
];
static DW: u64 = 0x100000000;

fn user_pro(username: &str, a2: bool, a3: u64, a4: u64) -> u64 {
    let username = username.to_uppercase();
    let mut v16 = 0;
    let length = username.len();
    let mut i = 0;

    if length <= 0 {
        0
    } else {
        let mut v13: u64 = 0;
        let mut v14: u64 = 0;
        let mut v7 = (15 * a4) % 0x100;
        let mut v15 = (17 * a3) % 0x100;
        let username = username.as_bytes();
        let mut result = 0;
        while i < length {
            let upperName_char = username[i];
            let v9 = (v16 + TABLE[upperName_char as usize]) % DW;
            if a2 {
                let v10 = (TABLE[v7 as usize]
                    + TABLE[v15 as usize]
                    + TABLE[(upperName_char as usize + 47)]
                    * (TABLE[(upperName_char as usize + 13)] ^ v9)) % DW;
                result = (TABLE[v14 as usize] + v10) % DW;
                v16 = (TABLE[v14 as usize] + v10) % DW;
            } else {
                let v12 = (TABLE[v7 as usize]
                    + TABLE[v15 as usize]
                    + TABLE[(upperName_char as usize + 23)]
                    * (TABLE[(upperName_char as usize + 63)] ^ v9)) % DW;
                result = (TABLE[v13 as usize] + v12) % DW;
                v16 = (TABLE[v13 as usize] + v12) % DW;
            }
            v14 += 19;
            v14 %= 0x100;
            i += 1;
            v15 += 9;
            v15 %= 0x100;
            v7 += 13;
            v7 %= 0x100;
            v13 += 7;
            v13 %= 0x100;
            println!("Result: {}", result);
        }
        result
    }
}

use rand::Rng;

fn magic_number(users: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let counts = if users > 0 && users <= 1000 { users } else { rng.gen_range(1..1000) };
    println!("counts = {}", counts);
    loop {
        let i: u64 = rng.gen_range(0..0x10000);
        let v8 = ((((i ^ 0x7892) + 19760) ^ 0x3421) % 0x10000) % 11;
        let v9 = ((((i ^ 0x7892) + 19760) ^ 0x3421) % 0x10000) / 11;
        if v8 == 0 && v9 == counts {
            return i;
        }
    }
}

fn magic_calculation(UserName: &str, UserCounts: u32) -> u32 {
    let EBPMinus8 = 0xC22Eu32;
    let EBPMinus5 = 1u32;
    let Local1 = UserName.as_bytes();
    let mut Local3 = 0;

    let mut Local5 = (UserCounts << 4) - UserCounts;
    let mut Local6 = EBPMinus8 + (EBPMinus8 << 4);
    let mut Local8 = 0u32;

    let mut Length = UserName.len();

    if Length == 0 {
        return 0;
    }

    let mut Local4 = 1u32; // 循环计数
    let mut EDX = 0u32;
    let mut ECX = 0;
    let mut EDI = 0;
    let mut Local7 = 0;
    while Length != 0 {
        let mut AL = Local1[Local4 as usize - 1];
        if AL >= 0x61 && AL <= 0x7A {
            AL -= 0x20;
        }

        EDX = Local7 & 0xFF;
        EDX = TABLE[EDX as usize] as u32;


        ECX = Local5 & 0xFF;
        EDX += TABLE[ECX as usize] as u32;


        ECX = Local6 & 0xFF;
        let j = EDX as u64 + TABLE[ECX as usize];
        let k = j % 0x100000000;
        // EDX += TABLE[ECX as usize] as u32;
        EDX = k as u32;


        ECX = AL as u32;
        EDI = TABLE[ECX as usize + 0xD] as u32;


        ECX = AL as u32;
        ECX = TABLE[ECX as usize] as u32;
        let h = ECX as u64 + Local3 as u64;
        let i = h % 0x100000000;
        // ECX += Local3;
        ECX = i as u32;

        EDI ^= ECX;
        let a = EDI as u64;
        let b = TABLE[AL as usize + 0x2F];
        let c = a * b;
        let d = c % 0x100000000;
        println!("a: {:X}, b: {:X}, c: {:X}, d: {:X}", a, b, c, d);
        // EDI = ((EDI as u64 * TABLE[AL as usize + 0x2F]) % 0x100000000) as u32;
        EDI = d as u32;

        println!("EDX: {:X}, EDI: {:X}", EDX, EDI);
        let f = EDX as u64 + d;
        let g = f % 0x100000000;
        // EDX = EDX + EDI;
        EDX = g as u32;
        Local3 = EDX;


        Local7 += 0x13;
        Local8 += 0x7;
        Local6 += 0x9;
        Local5 += 0xD;
        Local4 += 1;
        Length -= 1;
    }
    return Local3;
}

fn keygen(UserName: &str, LicenseType: u32, UserCounts: u32) -> String
{
    let mut SI = 0;
    let mut Key = [0; 10];
    let mut KeyString = "".to_string();

    if UserName.is_empty() || LicenseType > 2 {
        return KeyString;
    }

    match LicenseType
    {
        // LICENSE_TYPE_SINGLE_USER:
        1 => SI = 1,
        // LICENSE_TYPE_SITE:
        2 => SI = 0x3E8,
        // LICENSE_TYPE_MULTI_USER:
        _ => if SI == 0 || SI >= 1000 {
            let mut rng = rand::thread_rng();
            SI = rng.gen_range(1..1000);
        } else {
            SI = UserCounts;
        }
    }

    let mut EDX = SI;
    Key[3] = 0xAC;
    let MagicNumber = magic_calculation(UserName, SI);

    Key[4] = (MagicNumber & 0xFF);

    Key[5] = ((MagicNumber >> 0x8) & 0xFF);

    Key[6] = (MagicNumber >> 0x10) & 0xFF;

    Key[7] = (MagicNumber >> 0x18) & 0xFF;

    //let a = ((((SI as u128 * 0xB) ^ 0x3421) - 0x4D30) ^ 0x7892) % 0x100000000;
    let a = SI as u64 * 0xB;
    let b = a as i64 ^ 0x3421;
    let c = b - 0x4D30;
    let d = c ^ 0x7892;
    // let EBX = (((((SI as u64 * 0xB) ^ 0x3421) - 0x4D30) ^ 0x7892) % 0x100000000) as u32;
    let EBX = d as u32;
    EDX = EBX;
    EDX = EDX >> 8;

    Key[1] = (EDX & 0xFF) ^ Key[7];

    Key[2] = (EBX & 0xFF) ^ Key[5];


    Key[0] = (0xFF95D981 & 0xFF) ^ Key[6];

    Key[8] = ((0xFF95D981 >> 8) & 0xFF) ^ Key[4];

    Key[9] = ((0xFF95D981 >> 0x10) & 0xFF) ^ Key[5];

    for v in Key {
        let s = format!("{:02X}", v).to_string();
        KeyString += s.as_str();
    }

    return KeyString;
}

fn main() {
    println!("Hello, world!");
    let mut p = [0u64; 8];
    p[3] = 0x9c;
    // p[3] = 0xac;
    // v9: Users counts
    // 1 - Single User License
    // 2 ~ 999 - Multiuser License
    // 1000 - Site License
    let v9 = 1000; // Single License
    let v8 = magic_number(v9);
    println!("v8, v9 = {}, {}", v8, v9);
    // let user = user_pro("Tommy Lau", true, 0, v9);
    let user = user_pro("Tommy Lau", true, 0, v9);
    println!("old magic: {:X}", user);
    let magic = magic_calculation("Tommy Lau", v9 as u32);
    println!("new magic: {:X}", magic);
    let key = keygen("Tommy Lau", 1, 1);
    assert_eq!(key, "4589DFAC9CB7C4174522");
    println!("New Key: {}", key);
    p[4] = user % 0x100;
    p[5] = (user >> 8) % 0x100;
    p[6] = (user >> 16) % 0x100;
    p[7] = (user >> 24) % 0x100;

    p[2] = p[5] ^ (v8 % 0x100);
    p[1] = p[7] ^ (v8 >> 8);

    let mut rng = rand::thread_rng();
    loop {
        p[0] = rng.gen_range(0..256);
        let v10 = (((p[6] ^ p[0]) ^ 0x18 + 61) % 0x100) ^ 0xa7;
        if v10 >= 10 { break; }
    }

    for (i, v) in p.iter().enumerate() {
        if i % 2 == 0 && i != 0 {
            print!("-");
        }
        print!("{:02X}", v);
    }
    // print!("-{:2X}", 0xd9 ^ p[4]);
    // println!("{:2X}", 0x95 ^ p[5]);
}
