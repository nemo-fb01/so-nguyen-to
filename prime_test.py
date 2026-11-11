import argparse
import math
import time


def count_primes(limit: int):
    segment_size = 1_000_000
    sqrt_limit = math.isqrt(limit - 1)

    # Tìm các số nguyên tố cơ sở đến căn bậc hai của limit.
    base_sieve = bytearray(b"\x01") * (sqrt_limit + 1)

    if sqrt_limit >= 0:
        base_sieve[0] = 0

    if sqrt_limit >= 1:
        base_sieve[1] = 0

    for candidate in range(
        2,
        math.isqrt(sqrt_limit) + 1,
    ):
        if not base_sieve[candidate]:
            continue

        start = candidate * candidate
        count = (
            (sqrt_limit - start) // candidate
        ) + 1

        base_sieve[start:sqrt_limit + 1:candidate] = (
            b"\x00" * count
        )

    base_primes = [
        number
        for number in range(2, sqrt_limit + 1)
        if base_sieve[number]
    ]

    prime_count = 0
    first_primes = []
    last_primes = []

    started_at = time.perf_counter()

    for segment_start in range(
        2,
        limit,
        segment_size,
    ):
        segment_end = min(
            segment_start + segment_size,
            limit,
        )

        segment = bytearray(b"\x01") * (
            segment_end - segment_start
        )

        for prime in base_primes:
            if prime * prime >= segment_end:
                break

            first_multiple = max(
                prime * prime,
                (
                    (segment_start + prime - 1)
                    // prime
                ) * prime,
            )

            count = (
                (segment_end - 1 - first_multiple)
                // prime
            ) + 1

            segment[
                first_multiple - segment_start
                : segment_end - segment_start
                : prime
            ] = b"\x00" * count

        for offset, is_prime in enumerate(segment):
            if not is_prime:
                continue

            prime = segment_start + offset
            prime_count += 1

            if len(first_primes) < 100:
                first_primes.append(prime)

            last_primes.append(prime)

            if len(last_primes) > 100:
                last_primes.pop(0)

        processed = segment_end
        elapsed = time.perf_counter() - started_at

        print(
            f"Đã xử lý {processed:,}/{limit:,} "
            f"trong {elapsed:.3f} giây",
            flush=True,
        )

    elapsed = time.perf_counter() - started_at

    return {
        "limit": limit,
        "prime_count": prime_count,
        "elapsed": elapsed,
        "first_primes": first_primes,
        "last_primes": last_primes,
    }


def main():
    parser = argparse.ArgumentParser()

    parser.add_argument(
        "--limit",
        required=True,
        type=int,
    )

    args = parser.parse_args()

    if args.limit < 2:
        raise ValueError(
            "Giới hạn phải lớn hơn hoặc bằng 2."
        )

    result = count_primes(args.limit)

    report = (
        f"Giới hạn: {result['limit']:,}\n"
        f"Số lượng số nguyên tố: "
        f"{result['prime_count']:,}\n"
        f"Thời gian tính: "
        f"{result['elapsed']:.6f} giây\n\n"
        f"100 số đầu:\n"
        f"{', '.join(map(str, result['first_primes']))}\n\n"
        f"100 số cuối:\n"
        f"{', '.join(map(str, result['last_primes']))}\n"
    )

    print("\n" + report)

    with open(
        "prime-result.txt",
        "w",
        encoding="utf-8",
    ) as file:
        file.write(report)


if __name__ == "__main__":
    main()
