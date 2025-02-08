#include "emp-zk/emp-zk.h"
#include <iostream>
#include <vector>
#include "emp-tool/emp-tool.h"
#if defined(__linux__)
	#include <sys/time.h>
	#include <sys/resource.h>
#elif defined(__APPLE__)
	#include <unistd.h>
	#include <sys/resource.h>
	#include <mach/mach.h>
#endif

using namespace emp;
using namespace std;

int port, party;
const int threads = 1;
int bitwidth = 16;

void test_circuit_zk_relu(BoolIO<NetIO> *ios[threads], int party, int matrix_sz) {
	long long test_n = matrix_sz * matrix_sz;

    int64_t *ar;
	int64_t *cr;
	ar = new int64_t[test_n];
	cr = new int64_t[test_n];
	for(int i = 0; i < test_n; i++) {
        if (i < (int)(test_n/2)){
		    ar[i] = -i - 1;
        }
        else {
            ar[i] = 5;
        }
		cr[i] = 5;
	}

	for(int i = 0; i < matrix_sz; i++) {
		for(int j = 0; j < matrix_sz; j++) {
            if(ar[i*matrix_sz+j] < 0) {
                cr[i*matrix_sz+j] = 0;
                // cout << i << " " << j << " YES2\n";
            }
		}
	}


	setup_zk_bool<BoolIO<NetIO>>(ios, threads, party);

	Integer *mat_a = new Integer[test_n];
	Integer *mat_c = new Integer[test_n];
    Integer zero = Integer(bitwidth, 0, PUBLIC);
    Integer random = Integer(bitwidth, 5, PUBLIC);
	Bit zero_bit = Bit(0, ALICE);
	vector<Bit> check_vec(bitwidth, zero_bit);
	
	for(int i = 0; i < test_n; i++) {
        if (i < (int)(test_n/2)){
            mat_a[i] = Integer(bitwidth, -1, ALICE);
        }
        else{
            mat_a[i] = Integer(bitwidth, 5, ALICE);
        }
        mat_c[i] = random;
	}

	auto start = clock_start();
	for(int i = 0; i < matrix_sz; ++i) {
		for(int j = 0; j < matrix_sz; ++j) {
            Bit check = mat_a[i*matrix_sz+j].geq(zero);
			check_vec[0] = check;
			Integer int_check(check_vec);
            mat_c[i*matrix_sz+j] = int_check * mat_a[i*matrix_sz+j];
		}
	}
	auto timeuse = time_from(start);

    for(int i = 0; i < matrix_sz; ++i) {
		for(int j = 0; j < matrix_sz; ++j) {
            int reveal = mat_c[i*matrix_sz+j].reveal<int>(PUBLIC);
            if (reveal != cr[i*matrix_sz+j]){
                cout << "WRONG: " << reveal << "!=" << cr[i*matrix_sz+j] <<  " in index " << i << "," << j << endl;
            }
        }
    }

	finalize_zk_bool<BoolIO<NetIO>>();
	cout << matrix_sz << "\t" << timeuse << " us\t" << party << " " << endl;

	delete[] ar;
	delete[] cr;
	delete[] mat_a;
	delete[] mat_c;
}

int main(int argc, char** argv) {
	parse_party_and_port(argv, &party, &port);
	BoolIO<NetIO>* ios[threads];
	for(int i = 0; i < threads; ++i)
		ios[i] = new BoolIO<NetIO>(new NetIO(party == ALICE?nullptr:"127.0.0.1",port+i), party==ALICE);

	std::cout << std::endl << "------------ circuit zero-knowledge proof test ------------" << std::endl << std::endl;;

	int num = 0;
	if(argc < 3) {
		std::cout << "usage: bin/bool/relu_bool PARTY PORT DIMENSION" << std::endl;
		return -1;
	} else if (argc == 3) {
		num = 2;
	} else {
		num = atoi(argv[3]);
	}

	

	test_circuit_zk_relu(ios, party, num);

	for(int i = 0; i < threads; ++i) {
		delete ios[i]->io;
		delete ios[i];
	}
	return 0;
}
