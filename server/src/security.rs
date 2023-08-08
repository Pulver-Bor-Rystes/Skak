use pwhash::bcrypt;

pub fn hash(password: &str) -> String {
    let hash: String = bcrypt::hash(password).expect("failed to hash password");

    return hash
}

pub fn verify(password: &str, hashed_password: &str) -> bool {
    bcrypt::verify(password, hashed_password)
}
