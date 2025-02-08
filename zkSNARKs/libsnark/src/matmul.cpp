#include <libsnark/common/default_types/r1cs_gg_ppzksnark_pp.hpp>
#include <libsnark/zk_proof_systems/ppzksnark/r1cs_gg_ppzksnark/r1cs_gg_ppzksnark.hpp>
#include <libsnark/gadgetlib1/pb_variable.hpp>
#include <libsnark/gadgetlib1/gadgets/basic_gadgets.hpp>
#include <chrono>

using namespace libsnark;
using namespace std;
using namespace std::chrono;

void impl_matmul() {
    typedef libff::Fr<default_r1cs_gg_ppzksnark_pp> FieldT;

    default_r1cs_gg_ppzksnark_pp::init_public_params();
    protoboard<FieldT> pb;

    size_t dim = 256;

    pb_variable_array<FieldT> A;
    pb_variable_array<FieldT> B;
    pb_variable<FieldT> res;
    bool verified = 0;

    res.allocate(pb, "res");
    A.allocate(pb, dim*dim, "A");
    B.allocate(pb, dim*dim, "B");

    inner_product_gadget<FieldT> compute_inner_product(pb, A, B, res, "compute_inner_product");
    compute_inner_product.generate_r1cs_constraints();

    for (size_t i = 0; i < dim*dim; ++i)
    {
        pb.val(A[i]) = FieldT::random_element();
        pb.val(B[i]) = FieldT::random_element();
    }
    const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();

    // generate keypair
    const r1cs_gg_ppzksnark_keypair<default_r1cs_gg_ppzksnark_pp> keypair = r1cs_gg_ppzksnark_generator<default_r1cs_gg_ppzksnark_pp>(constraint_system);

    // Add witness values
    compute_inner_product.generate_r1cs_witness();

    // generate proof
    auto proof_start = high_resolution_clock::now();
    const r1cs_gg_ppzksnark_proof<default_r1cs_gg_ppzksnark_pp> proof = r1cs_gg_ppzksnark_prover<default_r1cs_gg_ppzksnark_pp>(keypair.pk, pb.primary_input(), pb.auxiliary_input());
    auto proof_stop = high_resolution_clock::now();

    // verify
    auto verify_start = high_resolution_clock::now();
    verified = r1cs_gg_ppzksnark_verifier_strong_IC<default_r1cs_gg_ppzksnark_pp>(keypair.vk, pb.primary_input(), proof);
    auto verify_stop = high_resolution_clock::now();

    auto proof_duration = duration_cast<microseconds>(proof_stop-proof_start);
    auto verify_duration = duration_cast<microseconds>(verify_stop-verify_start);
    cout << "ProofGen Time Matmul (us): " << proof_duration.count() << endl;
    cout << "Verify Time Matmul (us): " << verify_duration.count() << endl;

    cout << verified << endl;
}
