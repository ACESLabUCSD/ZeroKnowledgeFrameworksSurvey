#!/usr/bin/python3

import pathlib
import random
import hashlib
import subprocess
import time


HASH_INPUT_LEN = 512  # bits
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
    print('Running benchmarks for sha256')

    setup_time = 0
    proof_time = 0
    proof_size = 0
    verify_time = 0

    echo("Running preliminary checks...")
    subprocess.check_call(["nargo", "check"])
    echo("Done")

    echo("Compiling circuit...")
    subprocess.check_call(["nargo", "compile"])
    echo("Done")

    for _ in range(BENCHMARK_ROUNDS):
        echo("Generating vk...")
        time_start = time.time()
        subprocess.check_call(["bb", "write_vk", "-b", "./target/sha256.json", "-o", "./target"])
        time_duration = time.time() - time_start
        echo("Done")
        setup_time += time_duration

        echo("Generating Prover.toml file with random circuit inputs...")
        msg = gen_random_bytes(HASH_INPUT_LEN // 8)
        digest = hashlib.sha256(msg).digest()
        msg_array = make_array_from_bytes(msg)
        digest_array = make_array_from_bytes(digest)
        prover_toml = f"msg = {msg_array}\nknown_hash = {digest_array}\n"
        prover_toml_path = get_project_dir() / "Prover.toml"
        open(prover_toml_path, "w").write(prover_toml)
        echo("Done")

        echo("Performing witness generation...")
        time_start = time.time()
        subprocess.check_call(["nargo", "execute"])
        time_duration = time.time() - time_start
        echo("Done")
        proof_time += time_duration

        echo("Generating proof...")
        time_start = time.time()
        subprocess.check_call(["bb", "prove", "-b", "./target/sha256.json", "-w", "./target/sha256.gz", "-o", "./target"])
        time_duration = time.time() - time_start
        echo("Done")
        proof_time += time_duration

        proof_path = get_project_dir() / "target" / "proof"
        public_inputs_path = get_project_dir() / "target" / "public_inputs"
        proof_size = proof_path.stat().st_size + public_inputs_path.stat().st_size

        echo("Verifying proof...")
        time_start = time.time()
        subprocess.check_call(["bb", "verify", "-k", "./target/vk", "-p", "./target/proof", "-i", "./target/public_inputs"])
        time_duration = time.time() - time_start
        echo("Done")
        verify_time += time_duration

    avg_setup_time = (setup_time * 10**6) / BENCHMARK_ROUNDS
    avg_proof_time = (proof_time * 10**6) / BENCHMARK_ROUNDS
    avg_verify_time = (verify_time * 10**6) / BENCHMARK_ROUNDS
    echo(f"Setup time: {avg_setup_time:.0f} us")
    echo(f"Proof time: {avg_proof_time:.0f} us")
    echo(f"Proof size: {proof_size} B")
    echo(f"Verify time: {avg_verify_time:.0f} us")


if __name__ == '__main__':
    main()
