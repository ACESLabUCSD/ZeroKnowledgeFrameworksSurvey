#include <libsnark/common/default_types/r1cs_gg_ppzksnark_pp.hpp>
#include <libsnark/zk_proof_systems/ppzksnark/r1cs_gg_ppzksnark/r1cs_gg_ppzksnark.hpp>
#include <libsnark/gadgetlib1/pb_variable.hpp>
#include <libsnark/gadgetlib1/gadgets/basic_gadgets.hpp>
#include <chrono>

using namespace libsnark;
using namespace std;
using namespace std::chrono;

void impl_relu() {
    typedef libff::Fr<default_r1cs_gg_ppzksnark_pp> FieldT;

    default_r1cs_gg_ppzksnark_pp::init_public_params();

    size_t dim = 10;

    protoboard<FieldT> pb;

    // pb_variable_array<FieldT> A_arr;
    // pb_variable_array<FieldT> B_arr;

    pb_variable<FieldT> A, B, less, less_or_eq;
    size_t n;
    A.allocate(pb, "A");
    B.allocate(pb, "B");
    less.allocate(pb, "less");
    less_or_eq.allocate(pb, "less_or_eq");
    // bool verified = 0;

    // comparison_gadget<FieldT> compute_comparison(pb, A, B, res, "compute_comparison");
    comparison_gadget<FieldT> compute_comparison(pb, n, A, B, less, less_or_eq, "compute_comparison");
    compute_comparison.generate_r1cs_constraints();

    // for (size_t i = 0; i < dim; ++i)
    // {
    //     pb.val(A_arr[i]) = FieldT::random_element();
    //     pb.val(B_arr[i]) = FieldT::random_element();
    // }

    auto start = high_resolution_clock::now();
    for (size_t a = 0; a < 1ul<<n; ++a)
    {
        for (size_t b = 0; b < 1ul<<n; ++b)
        {
            pb.val(A) = FieldT(a);
            pb.val(B) = FieldT(b);

            compute_comparison.generate_r1cs_witness();

#ifdef DEBUG
            printf("positive test for %zu < %zu\n", a, b);
#endif
            assert(pb.val(less) == (a < b ? FieldT::one() : FieldT::zero()));
            assert(pb.val(less_or_eq) == (a <= b ? FieldT::one() : FieldT::zero()));
            assert(pb.is_satisfied());
        }
    }
    // for (size_t j = 0; j < dim; ++j){
    //     const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();

    //     // generate keypair
    //     const r1cs_gg_ppzksnark_keypair<default_r1cs_gg_ppzksnark_pp> keypair = r1cs_gg_ppzksnark_generator<default_r1cs_gg_ppzksnark_pp>(constraint_system);

    //     // Add witness values
    //     compute_comparison.generate_r1cs_witness();

    //     // generate proof
    //     const r1cs_gg_ppzksnark_proof<default_r1cs_gg_ppzksnark_pp> proof = r1cs_gg_ppzksnark_prover<default_r1cs_gg_ppzksnark_pp>(keypair.pk, pb.primary_input(), pb.auxiliary_input());

    //     // verify
    //     verified = r1cs_gg_ppzksnark_verifier_strong_IC<default_r1cs_gg_ppzksnark_pp>(keypair.vk, pb.primary_input(), proof);
    // }
    auto stop = high_resolution_clock::now();
    auto duration = duration_cast<microseconds>(stop-start);
    cout << "Time Relu (us): " << duration.count() << endl;
    libff::print_time("comparison tests successful");

    // cout << verified << endl;
}
/*


printf("testing comparison_gadget on all %zu bit inputs\n", n);

    protoboard<FieldT> pb;

    pb_variable<FieldT> A, B, less, less_or_eq;
    A.allocate(pb, "A");
    B.allocate(pb, "B");
    less.allocate(pb, "less");
    less_or_eq.allocate(pb, "less_or_eq");

    comparison_gadget<FieldT> cmp(pb, n, A, B, less, less_or_eq, "cmp");
    cmp.generate_r1cs_constraints();

    for (size_t a = 0; a < 1ul<<n; ++a)
    {
        for (size_t b = 0; b < 1ul<<n; ++b)
        {
            pb.val(A) = FieldT(a);
            pb.val(B) = FieldT(b);

            cmp.generate_r1cs_witness();

#ifdef DEBUG
            printf("positive test for %zu < %zu\n", a, b);
#endif
            assert(pb.val(less) == (a < b ? FieldT::one() : FieldT::zero()));
            assert(pb.val(less_or_eq) == (a <= b ? FieldT::one() : FieldT::zero()));
            assert(pb.is_satisfied());
        }
    }

    libff::print_time("comparison tests successful");

*/