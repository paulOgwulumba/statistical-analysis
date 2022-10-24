pub mod measures_of_central_tendency;
pub mod measures_of_dispersion;

fn main() {
    let first_batch = vec![23, 56, 23, 78];

    let mean_first_batch = measures_of_central_tendency::get_mean(first_batch).unwrap_or_default();

    println!("The mean of first_batch is {}", mean_first_batch);
}