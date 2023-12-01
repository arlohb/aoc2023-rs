#![warn(clippy::unwrap_used, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

fn main() -> anyhow::Result<()> {
    let _ = std::fs::read_to_string("input.txt")?;

    Ok(())
}
