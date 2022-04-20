use std::ops::BitXor;
use chrono::{TimeZone, Utc};
use clap::Parser;

static TABLE: [u32; 256] = [
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

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// User name to generate license
    #[clap(short, long)]
    name: String,
}

fn format_password(password: &[u8]) -> String {
    let mut ret = "".into();

    for (i, v) in password.iter().enumerate() {
        if i % 2 == 0 && i != 0 {
            ret = format!("{}-", ret);
        }
        ret = format!("{}{:02X}", ret, v);
    }

    ret
}

fn calculate_checksum(user_name: &str, registration_version: bool, a3: u32, license_count: u16) -> Result<u32, String> {
    let mut result: u32 = 0;
    let mut unk0 = (a3 as u8).wrapping_mul(17);
    let mut unk1 = (license_count as u8).wrapping_mul(15);
    let mut j = 0u8;
    for cur in user_name.to_uppercase().bytes().map(|x| x as usize) {
        let temp = result.wrapping_add(TABLE[cur]);
        if registration_version {
            result = TABLE[j as usize]
                .wrapping_add(TABLE[unk1 as usize])
                .wrapping_add(TABLE[unk0 as usize])
                .wrapping_add(TABLE[(cur + 0x2F) & 0xFF]
                    .wrapping_mul(temp ^ TABLE[(cur + 0xD) & 0xFF]));
            j = j.wrapping_add(19);
        } else {
            result = TABLE[j as usize]
                .wrapping_add(TABLE[unk1 as usize])
                .wrapping_add(TABLE[unk0 as usize])
                .wrapping_add(TABLE[(cur + 0x17) & 0xFF]
                    .wrapping_mul(temp ^ TABLE[(cur + 0x3F) & 0xFF]));
            j = j.wrapping_add(7);
        }
        unk1 = unk1.wrapping_add(13);
        unk0 = unk0.wrapping_add(9);
    }

    Ok(result)
}

fn encode_expire_date(daystamp_of_expiration: u32, seed: u32) -> Result<u32, String> {
    if daystamp_of_expiration >= 0x1000000 {
        return Err("Days exceed 0x1000000".into());
    }

    let mut result = daystamp_of_expiration.wrapping_mul(0x11);
    while result < 0xFF000000 {
        result += 0x1000000;
    }

    loop {
        let temp = result.bitxor(0xFFE53167)
            .wrapping_add(0x2C175)
            .bitxor(0x22C078)
            .bitxor(seed);

        if temp >= 0x1000000 {
            result = result.wrapping_add(0x1000000);
        } else {
            result = temp;
            break;
        }
    }

    Ok(result)
}

fn encode_license_count(desired_license_count: u16) -> Result<u16, String> {
    if desired_license_count > 1000 || desired_license_count == 0 {
        return Err("Desired license count incorrect".into());
    }

    let ret = desired_license_count
        .wrapping_mul(11)
        .bitxor(0x3421)
        .wrapping_sub(0x4d30)
        .bitxor(0x7892);

    Ok(ret)
}

fn generate_evaluation_license(user_name: &str, daystamp_of_expiration: u32) -> String {
    let mut password = [0u8; 8];

    let password_checksum = calculate_checksum(user_name, false, 255, 1).unwrap();
    password[4..8].clone_from_slice(&password_checksum.to_le_bytes());

    let encoded_expire_duration = encode_expire_date(daystamp_of_expiration, password_checksum).unwrap().to_le_bytes();
    password[..4].clone_from_slice(&encoded_expire_duration);

    password[3] = 0xFC;

    format_password(&password)
}

fn generate_version_license(user_name: &str, license_count: u16, major_version: u8) -> String {
    let mut password = [0u8; 8];

    let major_version = major_version + 1;

    let password_checksum = calculate_checksum(
        user_name, true,
        if major_version >= 2 { 0 } else { major_version } as u32,
        license_count).unwrap().to_le_bytes();
    password[4..8].clone_from_slice(&password_checksum);

    let encoded_license_count = encode_license_count(license_count).unwrap().to_le_bytes();
    password[1] = encoded_license_count[1] ^ password[7];
    password[2] = encoded_license_count[0] ^ password[5];

    password[0] = major_version
        .bitxor(0xA7)
        .wrapping_sub(0x3D)
        .bitxor(0x18)
        .bitxor(password[6]);

    password[3] = 0x9C;

    format_password(&password)
}

fn generate_time_license(user_name: &str, daystamp_of_expiration: u32, license_count: u16) -> String {
    let mut password = [0u8; 10];

    let password_checksum = calculate_checksum(user_name, true, daystamp_of_expiration, license_count).unwrap().to_le_bytes();
    password[4..8].clone_from_slice(&password_checksum);

    let encoded_expire_duration = encode_expire_date(daystamp_of_expiration, 0x5B8C27).unwrap().to_le_bytes();
    password[0] = encoded_expire_duration[0] ^ password[6];
    password[8] = encoded_expire_duration[1] ^ password[4];
    password[9] = encoded_expire_duration[2] ^ password[5];

    let encoded_license_count = encode_license_count(license_count).unwrap().to_le_bytes();
    password[1] = encoded_license_count[1] ^ password[7];
    password[2] = encoded_license_count[0] ^ password[5];

    password[3] = 0xAC;

    format_password(&password)
}

fn main() {
    // Parse command line
    let cli = Cli::parse();

    // Expiration date: Dec 31, 2099
    let expiration_day = Utc.ymd(2099, 12, 31);
    let datetime_1970 = Utc.ymd(1970, 1, 1);
    let duration_days = (expiration_day - datetime_1970).num_days() as u32 - 1;
    let args: Vec<String> = std::env::args().collect();
    let expiration = if args.len() > 1 {
        match args[1].parse::<u32>() {
            Ok(v) => v,
            _ => duration_days,
        }
    } else { duration_days };

    let username = cli.name.as_str();
    let time_license = generate_time_license(username, expiration, 1000);
    let version_license = generate_version_license(username, 1000, 12);
    let evaluation_license = generate_evaluation_license(username, expiration);
    println!("Username: {}", username);
    println!("Time License: {}", time_license);
    println!("Version License: {}", version_license);
    println!("Evaluation License: {}", evaluation_license);
}

/*
enum ReleaseDaystamp {
    Version12 = 18887,
    Version11 = 18535,
    Version10 = 18236,
    Version9 = 17814,
    Version8 = 17289,
    Version7 = 16883,
    Version6 = 16420,
    Version5 = 15880,
    Version4 = 15474,
    Version3 = 14018,
};
 */
