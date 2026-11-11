use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

const SEGMENT_ODD_COUNT: usize = 8_388_608;
const PREVIEW_COUNT: usize = 100;

fn integer_sqrt(number: u64) -> u64 {
    if number < 2 {
        return number;
    }

    let mut result = (number as f64).sqrt() as u64;

    loop {
        let next = match result.checked_add(1) {
            Some(value) => value,
            None => break,
        };

        match next.checked_mul(next) {
            Some(square) if square <= number => {
                result = next;
            }
            _ => break,
        }
    }

    loop {
        match result.checked_mul(result) {
            Some(square) if square <= number => break,
            _ => result -= 1,
        }
    }

    result
}

fn create_base_primes(limit: u64) -> Vec<u64> {
    if limit < 3 {
        return Vec::new();
    }

    /*
     * Chỉ lưu các số lẻ:
     *
     * index 0 tương ứng với 3
     * index 1 tương ứng với 5
     * index 2 tương ứng với 7
     */
    let odd_count = ((limit - 3) / 2 + 1) as usize;
    let mut composite = vec![0_u8; odd_count];

    let sieve_limit = integer_sqrt(limit);
    let mut index = 0_usize;

    while index < odd_count {
        let prime = 2_u64 * index as u64 + 3;

        if prime > sieve_limit {
            break;
        }

        if composite[index] == 0 {
            let start = ((prime * prime - 3) / 2) as usize;
            let step = prime as usize;
            let mut multiple_index = start;

            while multiple_index < odd_count {
                composite[multiple_index] = 1;
                multiple_index += step;
            }
        }

        index += 1;
    }

    let mut primes = Vec::new();

    for (index, &is_composite) in composite.iter().enumerate() {
        if is_composite == 0 {
            primes.push(2_u64 * index as u64 + 3);
        }
    }

    primes
}

struct BenchmarkResult {
    limit: u64,
    prime_count: u64,
    elapsed_seconds: f64,
    segments_processed: u64,
    first_primes: Vec<u64>,
    last_primes: Vec<u64>,
}

fn count_primes(limit: u64) -> BenchmarkResult {
    let started_at = Instant::now();

    if limit <= 2 {
        return BenchmarkResult {
            limit,
            prime_count: 0,
            elapsed_seconds: started_at.elapsed().as_secs_f64(),
            segments_processed: 0,
            first_primes: Vec::new(),
            last_primes: Vec::new(),
        };
    }

    /*
     * Số 2 là số nguyên tố chẵn duy nhất.
     */
    let mut prime_count = 1_u64;

    let mut first_primes =
        Vec::with_capacity(PREVIEW_COUNT);

    let mut last_primes =
        VecDeque::with_capacity(PREVIEW_COUNT);

    first_primes.push(2);
    last_primes.push_back(2);

    let maximum_value = limit - 1;
    let square_root = integer_sqrt(maximum_value);

    /*
     * Đây là những số nguyên tố dùng để đánh dấu
     * các hợp số trong từng đoạn.
     */
    let base_primes =
        create_base_primes(square_root);

    /*
     * Mỗi phần tử trong segment đại diện cho một số lẻ.
     * Vì vậy một segment 8 MiB bao phủ khoảng 16 triệu số.
     */
    let segment_span = (
        SEGMENT_ODD_COUNT as u64
    )
        .checked_mul(2)
        .expect("Kich thuoc segment bi tran so.");

    let mut segment_low = 3_u64;
    let mut segments_processed = 0_u64;

    while segment_low < limit {
        /*
         * segment_low luôn phải là số lẻ.
         */
        if segment_low % 2 == 0 {
            segment_low += 1;
        }

        let segment_high = segment_low
            .saturating_add(segment_span)
            .min(limit);

        /*
         * Số lượng số lẻ trong khoảng:
         *
         * [segment_low, segment_high)
         */
        let odd_count =
            ((segment_high - segment_low) + 1) / 2;

        let mut composite =
            vec![0_u8; odd_count as usize];

        for &prime in &base_primes {
            let prime_square =
                match prime.checked_mul(prime) {
                    Some(value) => value,
                    None => break,
                };

            /*
             * Nếu bình phương của prime đã nằm ngoài
             * segment thì các prime tiếp theo cũng vậy.
             */
            if prime_square >= segment_high {
                break;
            }

            /*
             * Tìm bội đầu tiên của prime không nhỏ hơn
             * segment_low.
             */
            let quotient =
                segment_low.saturating_add(prime - 1)
                    / prime;

            let first_divisible =
                match quotient.checked_mul(prime) {
                    Some(value) => value,
                    None => continue,
                };

            let mut first_multiple =
                prime_square.max(first_divisible);

            /*
             * Segment chỉ lưu số lẻ.
             * Nếu bội đầu tiên là chẵn, chuyển sang
             * bội lẻ kế tiếp.
             */
            if first_multiple % 2 == 0 {
                first_multiple =
                    first_multiple.saturating_add(prime);
            }

            if first_multiple >= segment_high {
                continue;
            }

            let mut mark_index = (
                (first_multiple - segment_low) / 2
            ) as usize;

            /*
             * Vì index chỉ đại diện cho số lẻ, bước nhảy
             * trong mảng bằng chính prime.
             */
            let step = prime as usize;

            while mark_index < composite.len() {
                composite[mark_index] = 1;
                mark_index += step;
            }
        }

        for (
            index,
            &is_composite,
        ) in composite.iter().enumerate() {
            if is_composite != 0 {
                continue;
            }

            let prime =
                segment_low + 2 * index as u64;

            if prime >= limit {
                break;
            }

            prime_count += 1;

            if first_primes.len() < PREVIEW_COUNT {
                first_primes.push(prime);
            }

            if last_primes.len() == PREVIEW_COUNT {
                last_primes.pop_front();
            }

            last_primes.push_back(prime);
        }

        segments_processed += 1;

        let processed_to =
            segment_high.min(limit);

        let progress =
            processed_to as f64
                / limit as f64
                * 100.0;

        println!(
            concat!(
                "Da xu ly {:>15} / {:>15}",
                " | {:>7.3}%",
                " | {:>15} so nguyen to",
                " | {:>10.3} giay"
            ),
            processed_to,
            limit,
            progress,
            prime_count,
            started_at.elapsed().as_secs_f64(),
        );

        segment_low = segment_high;
    }

    BenchmarkResult {
        limit,
        prime_count,
        elapsed_seconds:
            started_at.elapsed().as_secs_f64(),
        segments_processed,
        first_primes,
        last_primes:
            last_primes.into_iter().collect(),
    }
}

fn format_numbers(
    numbers: &[u64],
) -> String {
    numbers
        .iter()
        .map(|number| number.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

fn main() {
    let limit_text =
        env::args().nth(1).unwrap_or_else(|| {
            eprintln!(
                "Cach dung: prime-benchmark <limit>"
            );

            std::process::exit(2);
        });

    let limit: u64 =
        limit_text.parse().unwrap_or_else(|_| {
            eprintln!(
                "Gioi han phai la so nguyen duong."
            );

            std::process::exit(2);
        });

    if limit < 2 {
        eprintln!(
            "Gioi han phai lon hon hoac bang 2."
        );

        std::process::exit(2);
    }

    println!(
        "Bat dau tim so nguyen to nho hon {}...",
        limit
    );

    println!(
        concat!(
            "Kich thuoc moi segment: ",
            "{} so le, khoang {:.1} MiB"
        ),
        SEGMENT_ODD_COUNT,
        SEGMENT_ODD_COUNT as f64
            / 1_048_576.0,
    );

    let result = count_primes(limit);

    let throughput =
        if result.elapsed_seconds > 0.0 {
            result.limit as f64
                / result.elapsed_seconds
                / 1_000_000.0
        } else {
            0.0
        };

    let report = format!(
        concat!(
            "Gioi han: {}\n",
            "So luong so nguyen to: {}\n",
            "Thoi gian tinh: {:.6} giay\n",
            "So segment da xu ly: {}\n",
            "Toc do trung binh: ",
            "{:.3} trieu so/giay\n",
            "\n",
            "100 so nguyen to dau tien:\n",
            "{}\n",
            "\n",
            "100 so nguyen to cuoi cung:\n",
            "{}\n"
        ),
        result.limit,
        result.prime_count,
        result.elapsed_seconds,
        result.segments_processed,
        throughput,
        format_numbers(&result.first_primes),
        format_numbers(&result.last_primes),
    );

    println!();
    println!("{report}");

    fs::write(
        "prime-result.txt",
        &report,
    )
    .expect(
        "Khong the ghi file prime-result.txt.",
    );
}
