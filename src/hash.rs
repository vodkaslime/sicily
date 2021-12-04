use sha2::{ Sha256, Digest };

pub fn digest(input: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());

    return format!("{:X}", hasher.finalize());
}