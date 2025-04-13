use rand::{Rng, SeedableRng, distr::Alphanumeric, rngs::StdRng};

pub fn generate_access_token(length: usize) -> String {
    let rng = StdRng::from_os_rng();
    let token: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    token
}
