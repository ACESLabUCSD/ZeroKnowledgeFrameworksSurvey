#!/usr/bin/python3

import pathlib
import random
import hashlib
import subprocess
import time


BENCHMARK_ROUNDS = 10


def get_project_dir():
    return pathlib.Path(__file__).parent


def gen_random_bytes(num_bytes):
    return bytearray(random.getrandbits(8) for _ in range(num_bytes))


def make_array_from_bytes(bytes):
    return "[" + ", ".join(hex(byte) for byte in bytes) + "]"


def echo(*string):
    print("[*]", *string)


def main():
    print('Running benchmarks for matmult')

    proof_time = 0
    proof_size = 0
    verify_time = 0
    for _ in range(BENCHMARK_ROUNDS):
        echo("Running preliminary checks...")
        subprocess.check_call(["nargo", "check"])
        echo("Done")

        # echo("Generating Prover.toml file with random circuit inputs...")
        # prover_toml = f"a = {array_a}\nb = {array_b}\nc = {array_c}\n"
        # prover_toml_path = get_project_dir() / "Prover.toml"
        # open(prover_toml_path, "w").write(prover_toml)
        # echo("Done")

        echo("Generating proof...")
        time_start = time.time()
        subprocess.check_call(["nargo", "prove", "p"])
        time_duration = time.time() - time_start
        echo("Done")
        proof_time += time_duration

        proof_path = get_project_dir() / "proofs" / "p.proof"
        proof_size = proof_path.stat().st_size

        echo("Verifying proof...")
        time_start = time.time()
        subprocess.check_call(["nargo", "verify", "p"])
        time_duration = time.time() - time_start
        echo("Done")
        verify_time += time_duration

    avg_proof_time = (proof_time * 10**6) / BENCHMARK_ROUNDS
    avg_verify_time = (verify_time * 10**6) / BENCHMARK_ROUNDS
    echo(f"(Setup +) Proof time: {avg_proof_time:.0f} us")
    echo(f"Proof size: {proof_size} B")
    echo(f"Verify time: {avg_verify_time:.0f} us")


if __name__ == '__main__':
    main()
