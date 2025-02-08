package examples;

import java.math.BigInteger;
import java.util.Arrays;

import examples.auxiliary.SHA256;
import universal.UniversalCircuitGenerator;

public class SHA256Demo{

	private UniversalCircuitGenerator generator;
	public SHA256Demo(UniversalCircuitGenerator generator) {
		this.generator = generator;
	}


	public void setInputToUniversalCircuit() {
		generator.prepareForSpecification();
		BigInteger[] sampleInput = new BigInteger[16];
		// sample input (64 bytes):
		// 61626380000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000018
		Arrays.fill(sampleInput, BigInteger.ZERO);
		sampleInput[0] = BigInteger.valueOf(0x61626380);
		sampleInput[15] = BigInteger.valueOf(0x18);

		int[] circuitInput = generator.createStmtArray( 16, sampleInput);
		int[] outputs = new SHA256(generator, circuitInput).getOutputs();

		generator.makeStmtOutputArray(outputs);
		generator.finalizeSpecification();
	}


	public static void main(String[] args) {
		// In this example, we set the number of operations of the universal circuit such that they are all utilized.
		// This is to measure the amplification cost.
		// See the matrix mul example for a more natural way for defining the universal circuit

		UniversalCircuitGenerator generator =  new UniversalCircuitGenerator("univ_circuit",  24, 920, 960, 479, 128, 0);
		generator.generateCircuit();
		generator.writeCircuitFile();

		SHA256Demo specifier = new SHA256Demo(generator);
		specifier.setInputToUniversalCircuit();
		generator.getCircuitEvaluator().evaluateCircuit();
		generator.getCircuitEvaluator().writeInputFile("sha256");

		// expected output (32 bytes):
		// ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
	}

}
