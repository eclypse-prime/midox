use anyhow::Result;
use midly::Smf;

pub fn bytes_to_smf(bytes: &[u8]) -> Result<Smf> {
    let smf = Smf::parse(bytes)?;
    Ok(smf)
}
