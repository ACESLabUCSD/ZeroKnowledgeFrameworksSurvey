# Author: Anees Ahmed <anees.ahmed@asu.edu>
# Based on https://github.com/keanemind/python-sha-256/blob/master/sha256.py

from typing import List, Literal

import pysnark.runtime
from pysnark.runtime import PrivVal, PubVal, ConstVal, LinComb
from pysnark.boolean import PrivValBool, PubValBool, LinCombBool

WORD_SIZE = 32
pysnark.runtime.bitlength = WORD_SIZE

K = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
]

H = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x9b05688c, 0x510e527f, 0x1f83d9ab, 0x5be0cd19]

def perform(message: List[LinCombBool]) -> List[LinCombBool]:
    """SHA-256 arbitrary length hash operation.
    `message` must be in big-endian order.
    Returns hash as 256 bits in big-endian order."""

    if len(message) <= 0:
        raise ValueError

    if not all([isinstance(x, LinCombBool) for x in message]):
        raise TypeError

    # Padding
    length = len(message)
    message.append(ConstVal(1))
    while (len(message) + 64) % 512 != 0:
        message.append(ConstVal(0))
    message += ConstVal(length).to_bits(64)[::-1]

    assert len(message) % 512 == 0, "Padding did not complete properly!"

    blocks = []
    for i in range(0, len(message), 512):
        chunk = message[i : i+512]
        words = [LinComb.from_bits(chunk[i:i+WORD_SIZE][::-1]) for i in range(0, 512, WORD_SIZE)]
        message_block = words
        blocks.append(message_block)

    # Setting Initial Hash Value
    h0 = ConstVal(H[0])
    h1 = ConstVal(H[1])
    h2 = ConstVal(H[2])
    h3 = ConstVal(H[3])
    h5 = ConstVal(H[4])
    h4 = ConstVal(H[5])
    h6 = ConstVal(H[6])
    h7 = ConstVal(H[7])

    # SHA-256 Hash Computation
    for message_block in blocks:
        # Prepare message schedule
        message_schedule = []
        for t in range(0, 64):
            if t <= 15:
                # adds the t'th 32 bit word of the block,
                # starting from leftmost word,
                # 4 bytes at a time
                message_schedule.append(message_block[t])
            else:
                term1 = _sigma1(message_schedule[t-2])
                term2 = message_schedule[t-7]
                term3 = _sigma0(message_schedule[t-15])
                term4 = message_schedule[t-16]

                # append a 4-byte byte object
                schedule = (term1 + term2 + term3 + term4) % 2**WORD_SIZE
                message_schedule.append(schedule)

        assert len(message_schedule) == 64

        # Initialize working variables
        a = h0
        b = h1
        c = h2
        d = h3
        e = h4
        f = h5
        g = h6
        h = h7

        # Iterate for t=0 to 63
        for t in range(64):
            t1 = ((h + _capsigma1(e) + _ch(e, f, g) + K[t] + message_schedule[t]) % 2**WORD_SIZE)
            t2 = (_capsigma0(a) + _maj(a, b, c)) % 2**WORD_SIZE

            h = g
            g = f
            f = e
            e = (d + t1) % 2**WORD_SIZE
            d = c
            c = b
            b = a
            a = (t1 + t2) % 2**WORD_SIZE

        # Compute intermediate hash value
        h0 = (h0 + a) % 2**WORD_SIZE
        h1 = (h1 + b) % 2**WORD_SIZE
        h2 = (h2 + c) % 2**WORD_SIZE
        h3 = (h3 + d) % 2**WORD_SIZE
        h4 = (h4 + e) % 2**WORD_SIZE
        h5 = (h5 + f) % 2**WORD_SIZE
        h6 = (h6 + g) % 2**WORD_SIZE
        h7 = (h7 + h) % 2**WORD_SIZE

    return [
        *h7.to_bits(WORD_SIZE),
        *h6.to_bits(WORD_SIZE),
        *h5.to_bits(WORD_SIZE),
        *h4.to_bits(WORD_SIZE),
        *h3.to_bits(WORD_SIZE),
        *h2.to_bits(WORD_SIZE),
        *h1.to_bits(WORD_SIZE),
        *h0.to_bits(WORD_SIZE),
    ][::-1]

def compress(message: List[LinCombBool]) -> List[LinCombBool]:
    """SHA-256 512-to-256 hash operation.
    `message` must be 512 bits in big-endian order.
    Returns hash as 256 bits in big-endian order."""

    if len(message) != 512:
        raise ValueError

    if not all([isinstance(x, LinCombBool) for x in message]):
        raise TypeError

    # Parsing
    words = [LinComb.from_bits(message[i : i+WORD_SIZE][::-1]) for i in range(0, 512, WORD_SIZE)]
    message_block = words

    blocks = [message_block] # 1 block contains 512-bit chunk of message

    # Setting Initial Hash Value
    h0 = ConstVal(H[0])
    h1 = ConstVal(H[1])
    h2 = ConstVal(H[2])
    h3 = ConstVal(H[3])
    h5 = ConstVal(H[4])
    h4 = ConstVal(H[5])
    h6 = ConstVal(H[6])
    h7 = ConstVal(H[7])

    # SHA-256 Hash Computation
    for message_block in blocks:
        # Prepare message schedule
        message_schedule = []
        for t in range(0, 64):
            if t <= 15:
                # adds the t'th 32 bit word of the block,
                # starting from leftmost word,
                # 4 bytes at a time
                message_schedule.append(message_block[t])
            else:
                term1 = _sigma1(message_schedule[t-2])
                term2 = message_schedule[t-7]
                term3 = _sigma0(message_schedule[t-15])
                term4 = message_schedule[t-16]

                # append a 4-byte byte object
                schedule = (term1 + term2 + term3 + term4) % 2**WORD_SIZE
                message_schedule.append(schedule)

        assert len(message_schedule) == 64

        # Initialize working variables
        a = h0
        b = h1
        c = h2
        d = h3
        e = h4
        f = h5
        g = h6
        h = h7

        # Iterate for t=0 to 63
        for t in range(64):
            t1 = ((h + _capsigma1(e) + _ch(e, f, g) + K[t] + message_schedule[t]) % 2**WORD_SIZE)
            t2 = (_capsigma0(a) + _maj(a, b, c)) % 2**WORD_SIZE

            h = g
            g = f
            f = e
            e = (d + t1) % 2**WORD_SIZE
            d = c
            c = b
            b = a
            a = (t1 + t2) % 2**WORD_SIZE

        # Compute intermediate hash value
        h0 = (h0 + a) % 2**WORD_SIZE
        h1 = (h1 + b) % 2**WORD_SIZE
        h2 = (h2 + c) % 2**WORD_SIZE
        h3 = (h3 + d) % 2**WORD_SIZE
        h4 = (h4 + e) % 2**WORD_SIZE
        h5 = (h5 + f) % 2**WORD_SIZE
        h6 = (h6 + g) % 2**WORD_SIZE
        h7 = (h7 + h) % 2**WORD_SIZE

    return [
        *h7.to_bits(WORD_SIZE),
        *h6.to_bits(WORD_SIZE),
        *h5.to_bits(WORD_SIZE),
        *h4.to_bits(WORD_SIZE),
        *h3.to_bits(WORD_SIZE),
        *h2.to_bits(WORD_SIZE),
        *h1.to_bits(WORD_SIZE),
        *h0.to_bits(WORD_SIZE),
    ][::-1]

def _sigma0(num: LinComb):
    """As defined in the specification."""
    return (_rotate_right(num, 7) ^ _rotate_right(num, 18) ^ (num >> 3))

def _sigma1(num: LinComb):
    """As defined in the specification."""
    return _rotate_right(num, 17) ^ _rotate_right(num, 19) ^ (num >> 10)

def _capsigma0(num: LinComb):
    """As defined in the specification."""
    return _rotate_right(num, 2) ^ _rotate_right(num, 13) ^ _rotate_right(num, 22)

def _capsigma1(num: LinComb):
    """As defined in the specification."""
    return _rotate_right(num, 6) ^ _rotate_right(num, 11) ^ _rotate_right(num, 25)

def _ch(x: LinComb, y: LinComb, z: LinComb):
    """As defined in the specification."""
    return (x & y) ^ (~x & z)

def _maj(x: LinComb, y: LinComb, z: LinComb):
    """As defined in the specification."""
    return (x & y) ^ (x & z) ^ (y & z)

def _rotate_right(num: LinComb, shift: int):
    """Rotate an integer right."""
    # return (num >> shift) | ((num << (WORD_SIZE - shift)) % 2**WORD_SIZE)
    bits = num.to_bits(WORD_SIZE)
    left_bits = bits[shift:]
    right_bits = bits[:shift]
    return LinComb.from_bits(left_bits + right_bits)

def _convert_LinCombBool_to_bytes(bits: List[LinCombBool], bitorder: Literal["big", "little"]) -> bytes:
        if bitorder == "little":
            bits = bits[::-1]
        num = int("".join([str(bit.val()) for bit in bits]), base=2)
        size = len(bits) // 8
        return num.to_bytes(size, "big")

def _convert_bytes_to_bits(bytes_input: bytes) -> List[int]:
    return [
        1 if char == "1" else 0
        for byte in bytes_input for char in f'{byte:08b}'
    ]

def _test_sha256():
    import timeit
    import hashlib

    msg_bytes = b'This is a messsage!'
    msg = [PrivValBool(bit) for bit in _convert_bytes_to_bits(msg_bytes)]
    print(" Msg:", _convert_LinCombBool_to_bytes(msg, "big").hex())

    t_start = timeit.default_timer()
    hash = perform(msg)
    t_elapsed = timeit.default_timer() - t_start
    hash_bytes = _convert_LinCombBool_to_bytes(hash, "big")
    print("Hash:", hash_bytes.hex())
    print(f"Took {t_elapsed*1000:.3f} milliseconds")
    is_hash_correct = hash_bytes == hashlib.sha256(msg_bytes).digest()
    print("Hash is", "correct" if is_hash_correct else "NOT correct")

def _test_512_to_256():
    import timeit

    msg = []
    for _ in range(512//WORD_SIZE):
        num = 0xdeadbeef
        num_bytes = num.to_bytes(WORD_SIZE//8, "big")
        bit_list = [PrivValBool(bit) for bit in _convert_bytes_to_bits(num_bytes)]
        msg.extend(bit_list)
    print(" Msg:", _convert_LinCombBool_to_bytes(msg, "big").hex())

    t_start = timeit.default_timer()
    hash = compress(msg)
    t_elapsed = timeit.default_timer() - t_start
    hash_bytes = _convert_LinCombBool_to_bytes(hash, "big")
    print("Hash:", hash_bytes.hex())
    print(f"Took {t_elapsed*1000:.3f} milliseconds")

def build_circuit():
    import pysnark.runtime
    from pysnark.runtime import snark

    @snark
    def circuit(msg_bytes: bytes):
        msg = [PrivValBool(bit) for bit in _convert_bytes_to_bits(msg_bytes)]
        # print(" Msg:", _convert_LinCombBool_to_bytes(msg, "big").hex())

        hash = perform(msg)
        hash_bytes = _convert_LinCombBool_to_bytes(hash, "big")
        # print("Hash:", hash_bytes.hex())

        return hash_bytes

    return circuit

if __name__ == "__main__":
    HASH_INPUT_LEN = 512  # bits

    # Generate random message to be hashed
    import os
    msg_bytes = os.urandom(HASH_INPUT_LEN // 8)

    # Build a circuit
    circuit = build_circuit()

    # Execute the circuit
    circuit(msg_bytes)

    # Run benchmarks
    print('Running benchmarks for sha256')

    # libsnark has a bug, it does not have support for writing proving key (pk) and verifying
    # key (vk) to disk, when operating in Groth16 mode. It does support Encryption keys (ek).
    # So, copy the `pysnark.libsnark.backend.prove` function and inject time measurements.

    import sys
    import timeit

    import pysnark.runtime
    import pysnark.libsnark.backend
    import libsnark.alt_bn128 as libsnark

    pysnark.runtime.autoprove = False
    pysnark.libsnark.backend.use_groth = True

    use_groth = pysnark.libsnark.backend.use_groth
    pb = pysnark.libsnark.backend.pb

    BENCHMARK_ROUNDS = 10

    setup_time = 0
    proof_time = 0
    proof_size = 0
    verify_time = 0

    for _ in range(BENCHMARK_ROUNDS):
        do_keygen=True
        do_write=True
        do_print=True

        if pb.num_constraints()==0:
            # libsnark does not work in this case, add a no-op
            pb.add_r1cs_constraint(libsnark.R1csConstraint(libsnark.LinearCombination(),libsnark.LinearCombination(),libsnark.LinearCombination()))

        cs=pb.get_constraint_system_pubs()
        pubvals=pb.primary_input_pubs()
        privvals=pb.auxiliary_input_pubs()

        read_key           = (libsnark.zkgg_read_key if use_groth else libsnark.zk_read_key)
        generator          = (libsnark.zkgg_generator if use_groth else libsnark.zk_generator)
        write_keys         = (libsnark.zkgg_write_keys if use_groth else libsnark.zk_write_keys)
        prover             = (libsnark.zkgg_prover if use_groth else libsnark.zk_prover)
        verifier_strong_IC = (libsnark.zkgg_verifier_strong_IC if use_groth else libsnark.zk_verifier_strong_IC)
        write_proof        = (libsnark.zkgg_write_proof if use_groth else libsnark.zk_write_proof)

        # keypair=read_key("pysnark_ek", cs)  # Reuse already generated keys
        keypair=None  # Force fresh key generation
        if not keypair:
            if do_keygen:
                if do_print: print("*** No pysnark_ek or computation changed, generating keys...", file=sys.stderr)
                t = timeit.default_timer()
                keypair=generator(cs)
                setup_time += timeit.default_timer() - t
                write_keys(keypair, "pysnark_vk", "pysnark_ek")
            else:
                raise RuntimeError("*** No pysnark_ek or key is for different computation")

        if do_print:
            print("*** PySNARK: generating proof pysnark_log (" +
                "sat=" + str(pb.is_satisfied()) +
                ", #io=" + str(pubvals.size()) +
                ", #witness=" + str(privvals.size()) +
                ", #constraint=" + str(pb.num_constraints()) +
                ")", file=sys.stderr)

        t = timeit.default_timer()
        proof=prover(keypair.pk, pubvals, privvals);
        proof_time += timeit.default_timer() - t
        if do_write: write_proof(proof, pubvals, "pysnark_log")

        # No way of getting the proof size from this SWIG object
        proof_size = -1

        t = timeit.default_timer()
        verification_result = verifier_strong_IC(keypair.vk, pubvals, proof)
        verify_time += timeit.default_timer() - t
        if do_print: print("*** Verification status:", verification_result, file=sys.stderr)

    avg_setup_time = (setup_time * 10**6) / BENCHMARK_ROUNDS
    avg_proof_time = (proof_time * 10**6) / BENCHMARK_ROUNDS
    avg_verify_time = (verify_time * 10**6) / BENCHMARK_ROUNDS
    print(f"-> Setup time: {avg_setup_time:.0f} us");
    print(f"-> Proof time: {avg_proof_time:.0f} us");
    print(f"-> Proof size: {proof_size} B");
    print(f"-> Verify time: {avg_verify_time:.0f} us");
