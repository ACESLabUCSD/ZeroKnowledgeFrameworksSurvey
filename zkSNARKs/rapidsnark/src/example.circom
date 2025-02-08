// Code provided by https://github.com/socathie/circomlib-matrix/tree/master

pragma circom 2.0.0;

template matElemMul (m,n) {
    signal input a[m][n];
    signal input b[m][n];
    signal output out[m][n];
    
    for (var i=0; i < m; i++) {
        for (var j=0; j < n; j++) {
            out[i][j] <== a[i][j] * b[i][j];
        }
    }
}

template matElemSum (m,n) {
    signal input a[m][n];
    signal output out;

    signal sum[m*n];
    sum[0] <== a[0][0];
    var idx = 0;
    
    for (var i=0; i < m; i++) {
        for (var j=0; j < n; j++) {
            if (idx > 0) {
                sum[idx] <== sum[idx-1] + a[i][j];
            }
            idx++;
        }
    }

    out <== sum[m*n-1];
}



// matrix multiplication
template matMul (m,n,p) {
    signal input a[m][n];
    signal input b[n][p];
    signal output out[m][p];

    component matElemMulComp[m][p];
    component matElemSumComp[m][p];
    
    for (var i=0; i < m; i++) {
        for (var j=0; j < p; j++) {
            matElemMulComp[i][j] = matElemMul(1,n);
            matElemSumComp[i][j] = matElemSum(1,n);
            for (var k=0; k < n; k++) {      
                matElemMulComp[i][j].a[0][k] <== a[i][k];
                matElemMulComp[i][j].b[0][k] <== b[k][j];
            }
            for (var k=0; k < n; k++) {
                matElemSumComp[i][j].a[0][k] <== matElemMulComp[i][j].out[0][k];
            }
            out[i][j] <== matElemSumComp[i][j].out;
        }
    }
}

component main = matMul(128,128,128);