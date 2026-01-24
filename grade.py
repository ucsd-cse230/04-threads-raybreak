#!/usr/bin/env python3
"""
Grade script that runs cargo test and calculates a score.
Vanilla tests are worth 5 points;
Tests with "test_seq_crack" are worth 15 points;
Tests with "test_par_crack" are worth 30 points.
"""

import subprocess
import re


def run_cargo_test():
    """Run cargo test and return the output."""
    result = subprocess.run(
        ["cargo", "test", "--release", "--no-fail-fast"],
        capture_output=True,
        text=True
    )
    # Combine stdout and stderr (cargo outputs to both)
    return result.stdout + result.stderr


def get_weight(test_name: str) -> int:
    """Return the weight for a test based on its name."""
    if "test_par_crack" in test_name:
        return 30
    if "test_seq_crack" in test_name:
        return 15
    return 5


def parse_test_results(output: str) -> tuple[int, int]:
    """
    Parse cargo test output to get weighted total and failed scores.
    Returns (total_score, failed_score)
    """
    total_score = 0
    failed_score = 0

    # Match individual test result lines
    # Format: "test test_name ... ok" or "test test_name ... FAILED"
    pattern = r'^test (\S+) \.\.\. (\w+)$'

    for line in output.splitlines():
        match = re.match(pattern, line)
        if match:
            test_name = match.group(1)
            result = match.group(2)
            weight = get_weight(test_name)
            total_score += weight
            if result == "FAILED":
                failed_score += weight

    return total_score, failed_score


def main():
    output = run_cargo_test()

    TOTAL, FAIL = parse_test_results(output)

    if TOTAL > 0:
        score = TOTAL - FAIL
        print(f"SCORE = {score} / {TOTAL}")
    else:
        print("SCORE = 0 (no tests found)")


if __name__ == "__main__":
    main()
