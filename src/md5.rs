use rayon::prelude::*;

const CHUNK_SIZE: usize = 100_000;

pub fn md5_search<T>(prefix: &str, test: T) -> impl Iterator<Item = usize>
where
    T: Fn(&[u8; 16]) -> bool + Send + Sync + Clone,
{
    let mut context = md5::Context::new();
    context.consume(prefix.trim());

    let mut chunk_start = 1;

    std::iter::from_fn(move || loop {
        let chunk_end = chunk_start + CHUNK_SIZE;
        let mut found: Vec<_> = (chunk_start..chunk_end)
            .into_par_iter()
            .filter(|n| {
                let mut c2 = context.clone();
                c2.consume(n.to_string());
                let digest = c2.compute();
                test(&digest.0)
            })
            .collect();

        chunk_start = chunk_end;

        if !found.is_empty() {
            found.sort();
            return Some(found.into_iter());
        }
    })
    .flatten()
}
