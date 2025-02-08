/*
 *  This file is part of the optimized implementation of the Picnic signature
 * scheme. See the accompanying documentation for complete details.
 *
 *  The code is provided under the MIT license, see LICENSE for
 *  more details.
 *  SPDX-License-Identifier: MIT
 */

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include "../limbo.h"
#include <cinttypes>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <fstream>
#include <iostream>

#ifdef DETAILED_IN_CODE_TIMERS
/* GLOBAL VARIABLES FOR TIMING */
std::vector<std::chrono::high_resolution_clock::time_point> start_timers(16);
std::vector<std::chrono::high_resolution_clock::time_point> stop_timers(16);
std::vector<std::chrono::duration<double>> time_spans(16);
/**/
#endif

void load_circuit(Circuit &C, const string &location, bool verbose = false) {
  if (verbose) {
    cout << "Loading circuit from location " << location << endl;
  }
  std::ifstream inpf(location);
  if (inpf.fail()) {
    throw std::runtime_error("file location");
  }
  inpf >> C;
  inpf.close();

  if (verbose) {
    cout << "Finished loading circuit" << endl;
  }
}

struct timing_and_size_t {
  double gen, prove, serialize, deserialize, verify;
  uint64_t size;
};

static void print_timings(timing_and_size_t &timing) {
  printf("I/O gen = %f, proof gen = %f, verify = %f, size (in bits) = %" PRIu64,
         timing.gen, timing.prove, timing.verify, timing.size);
}

int main(int argc, char **argv) {

  std::cout << "CONSTANTS ARE DEFINED AS FOLLOW." << std::endl;
  std::cout << "PARTY64 : " << PARTY64 << std::endl;
  std::cout << "SEC : " << SEC << std::endl;
#if defined(SAME_CHALLENGE)
  std::cout << "SAME CHALLENGE set to True" << std::endl;
#else
  std::cout << "SAME CHALENGE set to False" << std::endl;
#endif
  std::cout << std::endl;
  if (argc < 3) {
    std::cout << "Need to give the path to the Bristol Circuit and which parameters to use" << std::endl;
    std::cout << "e.g. ./bench my_circuit.txt 0" << std::endl;
    return -1;
  }
  Circuit C;
  load_circuit(C, argv[1]);
  const limbo_instance_t &instance = limbo_instance_get(atoi(argv[2])); 
  size_t nbr = 1;
#ifdef DETAILED_IN_CODE_TIMERS
    for (size_t i = 0; i < 16; i++) {
      time_spans[i] = std::chrono::milliseconds::zero();
    }
#endif
    std::cout << std::endl;
    timing_and_size_t timing = {0, 0, 0, 0, 0, 0};

    if (PARTY64 == 1 && (instance.num_MPC_parties > 64)) {
      std::cout << "skipping paramet set " << argv[2]
                << " because PARTY64=1 and num_MPC_parties > 64" << std::endl;
      return -1;
    }

    if (PARTY64 == 2 && (instance.num_MPC_parties <= 64)) {
      std::cout << "skipping paramet set " << argv[2]
                << " because PARTY64=2 and num_MPC_parties <= 64" << std::endl;
      return -1;
    }

    // Generate input output
    for (size_t r = 0; r < nbr; r++) {

      // Generate I/O pair
      std::chrono::high_resolution_clock::time_point start_time =
          std::chrono::high_resolution_clock::now();
      std::pair<std::vector<uint8_t>, std::vector<uint8_t>> input_output =
          limbo_gen(instance, C);
      std::chrono::high_resolution_clock::time_point tmp_time =
          std::chrono::high_resolution_clock::now();
      timing.gen += std::chrono::duration_cast<std::chrono::duration<double>>(
                        tmp_time - start_time)
                        .count();

      // Generate proof
      start_time = std::chrono::high_resolution_clock::now();
      limbo_proof_t signature =
          limbo_prove(instance, input_output.first, C);
      tmp_time = std::chrono::high_resolution_clock::now();
      timing.prove += std::chrono::duration_cast<std::chrono::duration<double>>(
                          tmp_time - start_time)
                          .count();

      if (r == 0) {
        timing.size = compute_proof_size(signature, instance);
      }

      // Verify proof
      start_time = std::chrono::high_resolution_clock::now();
      bool ok = false;
      ok = limbo_verify(instance, input_output.second, signature, C);
      tmp_time = std::chrono::high_resolution_clock::now();
      timing.verify +=
          std::chrono::duration_cast<std::chrono::duration<double>>(tmp_time -
                                                                    start_time)
              .count();
      // bool ok = false;
      if (not ok) {
        throw std::runtime_error("false proof");
      }
    }
    timing.gen = timing.gen / ((double)nbr);
    timing.prove = timing.prove / ((double)nbr);
    timing.verify = timing.verify / ((double)nbr);
    std::cout << "PARAMETERS ARE: (" << argv[2] << "), WITH TIMING AVERAGED OVER "
              << nbr << " RUNS" << std::endl;
    std::cout << "\t -nb_mult_gates : " << instance.nb_mult_gates << std::endl;
    std::cout << "\t -nb_add_gates : " << instance.nb_add_gates << std::endl;
    std::cout << "\t -input_size : " << instance.input_size << std::endl;
    std::cout << "\t -output_size : " << instance.output_size << std::endl;
    std::cout << "\t -digest_size : " << instance.digest_size << std::endl;
    std::cout << "\t -seed_size : " << instance.seed_size << std::endl;
    std::cout << "\t -num_rounds : " << instance.num_rounds << std::endl;
    std::cout << "\t -num_MPC_parties : " << instance.num_MPC_parties
              << std::endl;
    std::cout << "\t -compression_factor : " << instance.compression_factor
              << std::endl;
    std::cout << "\t -lambda : " << instance.lambda << std::endl;
    print_timings(timing);

#ifdef DETAILED_IN_CODE_TIMERS
    std::cout << "\n\n MICRO BENCHMARK " << std::endl;
    std::cout << "timers = {" << std::endl;
    for (unsigned int i = 0; i < 16; i++) {
      printf("\t 'timer_%u' : %f ,\n", i, (time_spans[i] / ((double)nbr)));
    }
    std::cout << "\n}\n" << std::flush;
#endif
    std::cout << std::endl;
  
}