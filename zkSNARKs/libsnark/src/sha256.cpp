// Based on https://github.com/scipr-lab/libsnark/blob/master/libsnark/gadgetlib1/gadgets/hashes/sha256/tests/test_sha256_gadget.cpp
// Based on https://github.com/scipr-lab/libsnark/blob/master/libsnark/zk_proof_systems/ppzksnark/r1cs_gg_ppzksnark/examples/run_r1cs_gg_ppzksnark.tcc

#include <libff/common/default_types/ec_pp.hpp>
#include <libff/common/profiling.hpp>
#include <libff/common/utils.hpp>

#include <libsnark/common/default_types/r1cs_gg_ppzksnark_pp.hpp>
#include <libsnark/gadgetlib1/gadgets/hashes/sha256/sha256_gadget.hpp>
#include <libsnark/zk_proof_systems/ppzksnark/r1cs_gg_ppzksnark/r1cs_gg_ppzksnark.hpp>

using namespace libsnark;

template<typename ppT>
protoboard<libff::Fr<ppT>> test_r1cs_circuit()
{
    using fieldT = libff::Fr<ppT>;

    protoboard<fieldT> pb;

    digest_variable<fieldT> left(pb, SHA256_digest_size, "left");
    digest_variable<fieldT> right(pb, SHA256_digest_size, "right");
    digest_variable<fieldT> output(pb, SHA256_digest_size, "output");

    sha256_two_to_one_hash_gadget<fieldT> f(pb, left, right, output, "f");
    f.generate_r1cs_constraints();
    printf("Number of constraints for sha256_two_to_one_hash_gadget: %zu\n", pb.num_constraints());

    const libff::bit_vector left_bv = libff::int_list_to_bits({0x426bc2d8, 0x4dc86782, 0x81e8957a, 0x409ec148, 0xe6cffbe8, 0xafe6ba4f, 0x9c6f1978, 0xdd7af7e9}, 32);
    const libff::bit_vector right_bv = libff::int_list_to_bits({0x038cce42, 0xabd366b8, 0x3ede7e00, 0x9130de53, 0x72cdf73d, 0xee825114, 0x8cb48d1b, 0x9af68ad0}, 32);
    const libff::bit_vector hash_bv = libff::int_list_to_bits({0xeffd0b7f, 0x1ccba116, 0x2ee816f7, 0x31c62b48, 0x59305141, 0x990e5c0a, 0xce40d33d, 0x0b1167d1}, 32);

    left.generate_r1cs_witness(left_bv);
    right.generate_r1cs_witness(right_bv);

    f.generate_r1cs_witness();

    output.generate_r1cs_witness(hash_bv);

    assert(pb.is_satisfied());

    return pb;
}

template <typename ppT>
bool test_groth16(const protoboard<libff::Fr<ppT>> &pb) {
    libff::enter_block("Call to test_groth16");

    libff::print_header("R1CS GG-ppzkSNARK Generator");
    r1cs_gg_ppzksnark_keypair<ppT> keypair = r1cs_gg_ppzksnark_generator<ppT>(pb.get_constraint_system());
    printf("\n");
    libff::print_indent();
    libff::print_mem("after generator");

    libff::print_header("Preprocess verification key");
    r1cs_gg_ppzksnark_processed_verification_key<ppT> pvk = r1cs_gg_ppzksnark_verifier_process_vk<ppT>(keypair.vk);

    libff::print_header("R1CS GG-ppzkSNARK Prover");
    r1cs_gg_ppzksnark_proof<ppT> proof =
        r1cs_gg_ppzksnark_prover<ppT>(keypair.pk, pb.primary_input(), pb.auxiliary_input());
    printf("\n");
    libff::print_indent();
    libff::print_mem("after prover");

    if (false) {
        libff::enter_block("Test serialization of proof");
        proof = libff::reserialize<r1cs_gg_ppzksnark_proof<ppT>>(proof);
        libff::leave_block("Test serialization of proof");
    }

    libff::print_header("R1CS GG-ppzkSNARK Verifier");
    const bool ans = r1cs_gg_ppzksnark_verifier_strong_IC<ppT>(keypair.vk, pb.primary_input(), proof);
    printf("\n");
    libff::print_indent();
    libff::print_mem("after verifier");
    printf("* The verification result is: %s\n", (ans ? "PASS" : "FAIL"));

    libff::leave_block("Call to test_groth16");

    return ans;
}

int impl_sha256()
{
    libff::start_profiling();

    // typedef default_r1cs_gg_ppzksnark_pp ppT;
    typedef libff::default_ec_pp ppT;

    ppT::init_public_params();

    libff::enter_block("SHA256");
    protoboard<libff::Fr<ppT>> pb = test_r1cs_circuit<ppT>();
    bool success = test_groth16<ppT>(pb);
    libff::leave_block("SHA256");
}
