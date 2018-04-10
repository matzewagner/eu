// Multiples of 3 and 5
fn main() {
    eu_1();
    eu_2();
}

fn eu_1() {
    let N = 1000;
    let m_1 = 3;
    let m_2 = 5;
    let r = multiples_of_n_and_n(N, m_1, m_2);
    println!("Sum multiples of {} and {} in range {}: {}", m_1, m_2, N, r);
}

fn multiples_of_n_and_n(N: u32, m_1: u32, m_2: u32) -> u32 {
    // brute force:
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

    // alt 0:
    // (1..N).filter(|&x| x % m_1 == 0 || x % m_2 == 0).sum()

    // alt 1:
    // for each m_, increment until N and save values in a list.
    // compare the 2 lists and remove duplicates.
    // sum

    // alt 2 (arithmetic progressions):
    // sum of terms as (n_terms/2)(a+last)
    sum_divisible_by(N, m_1) + sum_divisible_by(N, m_2) - sum_divisible_by(N, (m_1 * m_2))
}

fn sum_divisible_by(N: u32, m: u32) -> u32 {
    let target = N - 1;
    let num_terms = (target / m) as u32;
    let sum = m * (num_terms * (num_terms + 1)) / 2 as u32;
    sum
}

fn eu_2() {
    let N = 4000000;
    fib(N);
}

fn fib(N: u32) -> u32 {
    0
}
