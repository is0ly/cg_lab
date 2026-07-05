use rand::RngExt;

#[must_use]
pub fn range_u32(min: u32, max_exclusive: u32) -> u32 {
    assert!(min < max_exclusive, "min must be less than max_exclusive");

    let mut rng = rand::rng();

    rng.random_range(min..max_exclusive)
}

#[must_use]
pub fn range_f32(min: f32, max: f32) -> f32 {
    assert!(min < max, "min must be less than max");

    let mut rng = rand::rng();

    rng.random_range(min..max)
}
