#include <cstdint>
#include "emp-tool/emp-tool.h"
using namespace emp;

void matmul(int dim) {
	Integer a(8, 0, ALICE);
	Integer b(8, 0, ALICE);
	Integer *A = new Integer[dim*dim];
	Integer *B = new Integer[dim*dim];
	Integer *C = new Integer[dim*dim];
	for(int i = 0; i < dim; ++i)
		for(int j = 0; j < dim; ++j) {
			A[i*dim+j] = a;
			B[i*dim+j] = b;
		}
	for(int i = 0; i < dim; ++i)
		for(int j = 0; j < dim; ++j) {
			C[i*dim+j] = Integer(8, 0, PUBLIC);
			for(int k = 0; k < dim; ++k)
				C[i*dim+j] = C[i*dim+j] + A[i*dim+k]*B[k*dim+j];
		}
	for(int i = 0; i < dim; ++i)
		for(int j = 0; j < dim; ++j)
			C[i*dim+j].reveal<string>();
}

void comp(int n) {
	Integer a(n, 1, ALICE);
	Integer b(n, 0, BOB);
	Bit c = a.geq(b);
	c.reveal<bool>();
}

int main(int argc, char** argv) {
	setup_plain_prot(true, "matmul8.txt");
	matmul(8);
	finalize_plain_prot();
	BristolFormat bf8("matmul8.txt");

	setup_plain_prot(true, "matmul16.txt");
	matmul(16);
	finalize_plain_prot();
	BristolFormat bf16("matmul16.txt");

	setup_plain_prot(true, "matmul24.txt");
	matmul(24);
	finalize_plain_prot();
	BristolFormat bf24("matmul24.txt");

	setup_plain_prot(true, "matmul32.txt");
	matmul(32);
	finalize_plain_prot();
	BristolFormat bf32("matmul32.txt");

	setup_plain_prot(true, "matmul48.txt");
	matmul(48);
	finalize_plain_prot();
	BristolFormat bf48("matmul48.txt");

	setup_plain_prot(true, "matmul64.txt");
	matmul(64);
	finalize_plain_prot();
	BristolFormat bf64("matmul64.txt");

	setup_plain_prot(true, "matmul100.txt");
	matmul(100);
	finalize_plain_prot();
	BristolFormat bf100("matmul100.txt");

	setup_plain_prot(true, "matmul128.txt");
	matmul(128);
	finalize_plain_prot();
	BristolFormat bf128("matmul128.txt");
}