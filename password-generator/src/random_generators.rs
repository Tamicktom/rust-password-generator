use std::time::SystemTime;

pub fn random_number_generator() -> u64 {
    //get system time
    let now = SystemTime::now();

    //get duration since UNIX epoch
    let duration1 = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();

    //get the number of nanoseconds
    let nanoseconds = duration1.as_nanos();

    //return the number of nanoseconds
    nanoseconds as u64
}

pub fn random_character_generator() -> char {
    //generate a random number between 0 and 25
    let random_number = random_number_generator() % 26;

    //convert the number to a character
    let character = (random_number + 97) as u8 as char;

    //return the character
    character
}
