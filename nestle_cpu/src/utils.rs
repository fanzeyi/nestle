use std::ops::RangeInclusive;

pub fn pprint_binaries(binaries: &[u8], range: RangeInclusive<usize>) {
    let start = *range.start();
    let chunk_size = 16;

    binaries[range]
        .chunks(chunk_size)
        .enumerate()
        .for_each(|(offset, chunks)| {
            println!(
                "{:04X}: {}",
                start + offset * chunk_size,
                chunks
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        });
}
