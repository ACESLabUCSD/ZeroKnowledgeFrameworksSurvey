# r1csReader

We provide a simple way to read `.r1cs` files in this folder. All you need to do is generate a `.r1cs` file using our provided Circom or Zokrates environments, paste it here, and change `read.js` to use your file!

To build everything, you can run:
```
docker build -t r1cs .
```

You'll see something like this, which is the most useful information that you'll need:
```
nVars: 274433,
nOutputs: 0,
nPubInputs: 8192,
nPrvInputs: 4096,
nLabels: 274433,
nConstraints: 266240,
  ```