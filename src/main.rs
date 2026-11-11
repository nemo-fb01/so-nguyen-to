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

    while result
        .checked_add(1)
        .and_then(|value| value.checked_mul(value))
        .is_some_and(|square| square <= number)
    {
        result += 1;
    }

    while result
        .checked_mul(result)
        .is_none_or(|square| square > number)
    {
        result -= 1;
    }

    result
}

fn create_base_primes(limit: u64) -> Vec<u64> {
    if limit < 3 {
        return Vec::new();
    }

    /*
     * Mỗi phần tử đại diện cho một số lẻ:
     *
     * index 0 -> 3
     * index 1 -> 5
     * index 2 -> 7
     */
    let odd_count = ((limit - 3)*/ 2 + 1) as usize;
    let mut com*osite = vec![0_u8; odd_count];

  * let sieve_limit = integer_sqrt(li*it);

    let mut index = 0_usize;*
    while index < odd_count {
   *    let prime = 2_u64 * index as u*4 + 3;

        if prime > sieve_l*mit {
            break;
        }*
        if composite[index] == 0 *
            let start = ((prime **prime - 3) / 2) as usize;
        *   let step = prime as usize;

   *        let mut multiple_index = s*art;

            while multiple_i*dex < odd_count {
                *omposite[multiple_index] = 1;
    *           multiple_index += step;*            }
        }

        i*dex += 1;
    }

    composite
   *    .iter()
        .enumerate()
 *      .filter_map(|(index, &is_com*osite)| {
            if is_compos*te == 0 {
                Some(2_u*4 * index as u64 + 3)
            * else {
                None
     *      }
        })
        .collec*()
}

struct BenchmarkResult {
   *limit: u64,
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
    let mut first_primes = Vec::with_capacity(PREVIEW_COUNT);
    let mut last_primes = VecDeque::with_capacity(PREVIEW_COUNT);

    first_primes.push(2);
    last_primes.push_back(2);

    let maximum_value = limit - 1;
    let square_root = integer_sqrt(maximum_value);
    let base_primes = create_base_primes(square_root);

    /*
     * Mỗi đoạn chỉ chứa các số lẻ.
     *
     * SEGMENT_ODD_COUNT = 8.388.608 byte,
     * tương đương khoảng 8 MiB bộ nhớ làm việc.
     */
    let segment_span =
        (SEGMENT_ODD_COUNT as u64)
            .checked_mul(2)
            .expect("Segment size overflow");

    let mut segment_low = 3_u64;
    let mut segments_processed = 0_u64;

    while segment_low < limit {
        /*
         * Bảo đảm đầu đoạn luôn là số lẻ.
         */
        if segment_low % 2 == 0 *
            segment_low += 1;
   *    }

        let segment_high = *egment_low
            .saturating*add(segment_span)
            .min*limit);

        let odd_count =
 *          ((segment_high - segment*low) + 1) / 2;

        let mut co*posite =
            vec![0_u8; odd_count as usize];

        for &pr*me in &base_primes {
            l*t prime_square = match prime.check*d_mul(prime) {
                Som*(value) => value,
                None => break,
            };

            if prime_square >= segment_high {
                break;
            }

            let quotient =
                segment_low.saturating_add(prime - 1) / prime;

            let first_divisible = match quotient.checked_mul(prime) {
                Some(value) => value,
                None => continue,
            };

            let mut first_multiple =
                prime_square.max(first_divisible);

            /*
             * Đoạn chỉ lưu số lẻ. Nếu bội đầu tiên là số chẵn,
             * chuyển sang bội lẻ tiếp theo.
             */
            if first_multiple % * == 0 {
                first_mult*ple =
                    first_mu*tiple.saturating_add(prime);
     *      }

            if first_mult*ple >= segment_high {
            *   continue;
            }

      *     let mut index =
             *  ((first_multiple - segment_low) * 2) as usize;

            let ste* = prime as usize;

            wh*le index < composite.len() {
     *          composite[index] = 1;
  *             index += step;
      *     }
        }

        for (ind*x, &is_composite) in composite.ite*().enumerate() {
            if is*composite != 0 {
                c*ntinue;
            }

           *let prime = segment_low + 2 * inde* as u64;

            if prime >= *imit {
                break;
    *       }

            prime_count *= 1;

            if first_primes.*en() < PREVIEW_COUNT {
           *    first_primes.push(prime);
    *       }

            if last_prim*s.len() == PREVIEW_COUNT {
       *        last_primes.pop_front();
 *          }

            last_prim*s.push_back(prime);
        }

   *    segments_processed += 1;

    *   let processed_to = segment_high*min(limit);
        let progress =*            processed_to as f64 / *imit as f64 * 100.0;

        prin*ln!(
            "Đã xử lý {:>15} * {:>15} | \
             {:>7.3}% * {:>15} số nguyên tố | \
         *   {:>10.3} giây",
            pro*essed_to,
            limit,
     *      progress,
            prime_*ount,
            started_at.elaps*d().as_secs_f64(),
        );

   *    segment_low = segment_high;

 *      if segment_low % 2 == 0 {
  *         segment_low += 1;
       *}
    }

    BenchmarkResult {
   *    limit,
        prime_count,
  *     elapsed_seconds: started_at.e*apsed().as_secs_f64(),
        seg*ents_processed,
        first_prim*s,
        last_primes: last_prime*.into_iter().collect(),
    }
}

f* format_numbers(numbers: &[u64]) -* String {
    numbers
        .ite*()
        .map(u64::to_string)
  *     .collect::<Vec<_>>()
        *join(", ")
}

fn main() {
    let *imit_text = env::args()
        .n*h(1)
        .unwrap_or_else(|| {
*           eprintln!(
            *   "Cách dùng: prime-benchmark <li*it>"
            );

            s*d::process::exit(2);
        });

*   let limit: u64 = limit_text
   *    .parse()
        .unwrap_or_el*e(|_| {
            eprintln!(
   *            "Giới hạn phải là một *ố nguyên dương."
            );

            std::process::exit(2);
        });

    if limit < 2 {
        eprintln!(
            "Giới hạn phải lớn hơn hoặc bằng 2."
        );

        std::process::exit(2);
    }

    println!(
        "Bắt đầu tìm số nguyên tố nhỏ hơn {}...",
        limit
    );

    println!(
        "Kích thước mỗi đoạn: {} số lẻ, khoảng {:.1} MiB",
        SEGMENT_ODD_COUNT,
        SEGMENT_ODD_COUNT as f64 / 1_048_576.0,
    );

    let result = count_primes(limit);

    let report = format!(
        concat!(
            "Giới hạn: {}\n",
            "Số lượng số nguyên tố: {}\n",
            "Thời gian tính: {:.6} giây\n",
            "Số đoạn đã xử lý: {}\n",
            "Tốc độ trung bình: {:.3} triệu số/giây\n",
            "\n",
            "100 số nguyên tố đầu tiên:\n",
            "{}\n",
            "\n",
            "100 số nguyên tố cuối cùng:\n",
            "{}\n"
        ),
        result.limit,
        result.prime_count,
        result.elapsed_seconds,
        result.segments_processed,
        result.limit as f64
            / result.elapsed_seconds
            / 1_000_000.0,
        format_numbers(&result.first_primes),
        format_numbers(&result.last_primes),
    );

    println!("\n{report}");

    fs::write(
        "prime-result.txt",
        &report,
    )
    .expect("Không thể ghi prime-result.txt");
}
