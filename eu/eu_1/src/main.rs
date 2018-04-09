// Multiples of 3 and 5
fn main() {
    let N = 1000;
    let m_1 = 3;
    let m_2 = 5;
    let r = multiples_of_n_and_n(N, m_1, m_2);
    println!("Sum multiples of {} and {} in range {}: {}", m_1, m_2, N, r);
}

fn multiples_of_n_and_n(N: u32, m_1: u32, m_2: u32) -> u32 {
    // Verbose:
    // let mut c = 0;
    // for x in 0..N {
    //     match (x % m_1, x % m_2) {
    //         (0, 0) => c += x,
    //         (0, _) => c += x,
    //         (_, 0) => c += x,
    //         _ => continue,
    //     }
    // }
    // c
    (1..N).filter(|&x| x % m_1 == 0 || x % m_2 == 0).sum()
}
