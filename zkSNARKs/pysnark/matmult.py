from pysnark.runtime import PrivVal
from pysnark.array import Array
from pysnark.runtime import snark

from pysnark.fixedpoint import PrivValFxp, PubValFxp, PrivVal, PubVal

dimension = 64

# Define the circuit
@snark
def circuit(A, B):
    # Define the matrix product C as an array of public numbers
    C = [[PubVal(0) for j in range(dimension)] for i in range(dimension)]

    # Compute each element of C using dot products of the corresponding rows and columns
    for i in range(dimension):
        for j in range(dimension):
            C[i][j] = sum([A[i][k] * B[k][j] for k in range(dimension)])

    for i in range(dimension):
        for j in range(dimension):
            C[i][j].assert_zero()

if __name__ == '__main__':
    # Define the matrices A and B as arrays of private numbers
    A = [[PrivVal(0) for j in range(dimension)] for i in range(dimension)]
    B = [[PrivVal(0) for j in range(dimension)] for i in range(dimension)]

    # Execute the circuit
    circuit(A, B)

    # Run benchmarks
    print('Running benchmarks for matmult')

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
