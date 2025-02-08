pragma circom 2.0.0;

include "/circomlib/circuits/sha256/sha256.circom";

template Main() {
    signal input in[8];
    signal output out[256];

    component sha256 = Sha256(8);

    sha256.in <== in;
    out <== sha256.out;
}

component main = Main();
