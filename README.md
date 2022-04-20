## Usage

### Generate Evaluation License

```rust
fn generate_evaluation_license(user_name: &str, daystamp_of_expiration: u32) -> String
```

- `user_name`: the username to generate license
- `daystamp_of_expiration`: the days from 1970-01-01

### Generate Version License

```rust
fn generate_version_license(user_name: &str, license_count: u16, major_version: u8) -> String
```

- `user_name`: the username to generate license
- `license_count`:
    - 1 - Single License
    - 2 ~ 999 - Multiuser License
    - 1000 - Site License
- `major_version`: the major version

### Generate Time License

```rust
fn generate_time_license(user_name: &str, daystamp_of_expiration: u32, license_count: u16) -> String
```

- `user_name`: the username to generate license
- `daystamp_of_expiration`: the days from 1970-01-01
- `license_count`:
    - 1 - Single License
    - 2 ~ 999 - Multiuser License
    - 1000 - Site License

### Examples

```rust
use chrono::{TimeZone, Utc};
// Expiration date: Dec 31, 2099
let expiration_day = Utc.ymd(2099, 12, 31);
let datetime_1970 = Utc.ymd(1970, 1, 1);
let expiration = (expiration_day - datetime_1970).num_days() as u32 - 1;
// Set username
let username = "Tommy Lau";
// Generate evaluation license
let evaluation_license = generate_evaluation_license(username, expiration);
// Generate a "Site License" for "Version 12"
let version_license = generate_version_license(username, 1000, 12);
// Generate a "Site License" with expiration date of Dec 31, 2099
let time_license = generate_time_license(username, expiration, 1000);
```

## Bypass Activation Check

Point checking server to localhost in hosts:

```
127.0.0.1 www.sweetscape.com
```

Then run server.py as root, since the script needs to bind 80 port:

```
sudo ./server.py
```

Open 010 Editor and activate again.

## References

Python:

- https://github.com/p1ay8y3ar/010editor_keygen

ASM:

- https://github.com/iDone/010-Editor-Keygen

C++:

- https://github.com/tennc/KeyGen-Of-010Editor
- https://github.com/dstmath/010Editor-keygen
