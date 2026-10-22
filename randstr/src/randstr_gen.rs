use rand::distr::{Alphanumeric, SampleString};
use rand::RngExt;

pub fn generate_random_string() -> String {
    // rand no. for range
    let range_somth = rand::rng().random_range(25..=29);

    // Generates a 20-character pure gibberish string
    let gibberish = Alphanumeric.sample_string(&mut rand::rng(), range_somth);
    
    return gibberish;
}