// Based on https://github.com/emp-toolkit/emp-zk/blob/master/test/bool/memory_scalability.cpp

#include <emp-zk/emp-zk.h>
#include <iostream>
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
const string circuit_file_location = macro_xstr(EMP_CIRCUIT_PATH) + string("bristol_format/");
const int input_bits = 512; // SHA256
const int output_bits = 256; // SHA256

void calc_plain_hash(bool *out, const bool *msg, const char *filename)
{
	setup_plain_prot(false, "");
	BristolFormat cf(filename);
	assert((input_bits == cf.n1));
	assert((output_bits == cf.n3));

	Bit *W = new Bit[input_bits];
	Bit *O = new Bit[input_bits];
	for (int i = 0; i < input_bits; ++i)
		W[i] = Bit(msg[i], PUBLIC);
	for (int i = 0; i < input_bits; ++i)
		O[i] = Bit(false, PUBLIC);
	cf.compute(O, W, nullptr);
	for (int i = 0; i < 64; ++i)
		cf.compute(O, O, nullptr);
	for (int i = 0; i < output_bits; ++i)
		out[i] = O[i].reveal<bool>(PUBLIC);
	finalize_plain_prot();
}

void calc_hash(Bit *out, const Bit *msg, BristolFormat *cf)
{
	Bit temp[input_bits];
	cf->compute(temp, msg, nullptr);
	for (int i = 0; i < 64; ++i)
		cf->compute(temp, temp, nullptr);
	for (int i = 0; i < output_bits; ++i)
		out[i] = temp[i];
}

void merkle_tree_dfs(Bit *out, int node_index, const int deepest_level_index, const Bit *witness, BristolFormat *cf)
{
	if (node_index >= deepest_level_index)
	{
		int witness_index = node_index - deepest_level_index;
		calc_hash(out, &witness[input_bits * witness_index], cf);
	}
	else
	{
		Bit msg[output_bits * 2];
		merkle_tree_dfs(&msg[          0], 2 * node_index + 1, deepest_level_index, witness, cf);
		merkle_tree_dfs(&msg[output_bits], 2 * node_index + 2, deepest_level_index, witness, cf);
		calc_hash(out, msg, cf);
	}
}

void merkle_tree_plain_dfs(bool *out, int node_index, const int deepest_level_index, const bool *witness, const char *filename)
{
	if (node_index >= deepest_level_index)
	{
		int witness_index = node_index - deepest_level_index;
		calc_plain_hash(out, &witness[input_bits * witness_index], filename);
	}
	else
	{
		bool msg[output_bits * 2];
		merkle_tree_plain_dfs(&msg[          0], 2 * node_index + 1, deepest_level_index, witness, filename);
		merkle_tree_plain_dfs(&msg[output_bits], 2 * node_index + 2, deepest_level_index, witness, filename);
		calc_plain_hash(out, msg, filename);
	}
}

void test_merkle_tree_dfs(BoolIO<NetIO> *ios[threads], int party, int depth)
{
	cout << "merkle tree of depth: " << depth << endl;
	string file = circuit_file_location + "sha-256.txt";

	int tree_width = 1 << (depth - 1);
	int tree_nodes = 2 * tree_width - 1;
	cout << "number of nodes: " << tree_nodes << endl;
	cout << "number of plaintext leaf nodes: " << tree_width << endl;

	bool *witness_bits = new bool[input_bits * tree_width]; // num of input leaves = tree_width
	bool *calc_root_bits = new bool[output_bits];

	PRG prg;
	block prg_seed_bloc = makeBlock(12345, 54321);
	prg.reseed(&prg_seed_bloc); // fixed seed => both parties have same witness
	prg.random_bool(witness_bits, input_bits * tree_width);

	int deepest_level_index = (1 << (depth - 1)) - 1; // index of first (leftmost) node in the bottommost level

	merkle_tree_plain_dfs(calc_root_bits, 0, deepest_level_index, witness_bits, file.c_str());

	setup_zk_bool<BoolIO<NetIO>>(ios, threads, party);

	BristolFormat cf(file.c_str());
	assert((input_bits == cf.n1));
	assert((output_bits == cf.n3));

	Bit *calc_root = new Bit[output_bits];

	Bit *witness = new Bit[input_bits * tree_width]; // num of input leaves = tree_width
	for (int i = 0; i < input_bits * tree_width; ++i)
		witness[i] = Bit(witness_bits[i], ALICE);

	auto calc_start = clock_start();
	merkle_tree_dfs(calc_root, 0, deepest_level_index, witness, &cf);
	cout << "merkle tree (depth=" << depth << ") operation: " << time_from(calc_start) << " us " << party << "\n";

	auto check_start = clock_start();
	for (int i = 0; i < output_bits; ++i)
	{
		if (calc_root[i].reveal() != calc_root_bits[i])
			error("=== WRONG ===\n");
	}
	cout << "check operation: " << time_from(check_start) << " us " << party << "\n";

	bool cheated = finalize_zk_bool<BoolIO<NetIO>>();
	if (cheated)
		error("=== CHEATED ===\n");

	delete[] witness;
	delete[] witness_bits;
	delete[] calc_root;
	delete[] calc_root_bits;

// #if defined(__linux__)
// 	struct rusage rusage;
// 	if (!getrusage(RUSAGE_SELF, &rusage))
// 		cout << "[Linux]Peak resident set size: " << (size_t)rusage.ru_maxrss << endl;
// 	else
// 		cout << "[Linux]Query RSS failed" << endl;
// #elif defined(__APPLE__)
// 	struct mach_task_basic_info info;
// 	mach_msg_type_number_t count = MACH_TASK_BASIC_INFO_COUNT;
// 	if (task_info(mach_task_self(), MACH_TASK_BASIC_INFO, (task_info_t)&info, &count) == KERN_SUCCESS)
// 		cout << "[Mac]Peak resident set size: " << (size_t)info.resident_size_max << endl;
// 	else
// 		cout << "[Mac]Query RSS failed" << endl;
// #endif
}

int main(int argc, char **argv)
{
	parse_party_and_port(argv, &party, &port);
	BoolIO<NetIO> *ios[threads];
	for (int i = 0; i < threads; ++i)
		ios[i] = new BoolIO<NetIO>(new NetIO(party == ALICE ? nullptr : "127.0.0.1", port + i), party == ALICE);

	cout << endl
			  << "------------ circuit zero-knowledge proof test ------------" << endl
			  << endl;
	;

	int depth = 0;
	if (argc < 3)
	{
		cout << "usage: bin/bool/merkle_tree PARTY PORT DEPTH_OF_MERKLE_TREE" << endl;
		return -1;
	}
	else if (argc == 3)
	{
		depth = 8;
	}
	else
	{
		depth = atoi(argv[3]);
	}

	test_merkle_tree_dfs(ios, party, depth);

	for (int i = 0; i < threads; ++i)
	{
		delete ios[i]->io;
		delete ios[i];
	}
	return 0;
}
