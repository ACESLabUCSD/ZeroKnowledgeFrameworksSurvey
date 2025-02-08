# EMP-Toolkit

Here we provide a simple way to develop [Bristol Fashion circuits](https://nigelsmart.github.io/MPC-Circuits/) (useful in MPCitH) using the [EMP-toolkit](https://github.com/emp-toolkit)

To generate your BC, simply add a function to `genCircuits.cpp`. This will generate a `.txt` file that holds your BC representation. We provide `circuits/count.py` to count the number of `XOR` and `AND` gates. After changing `genCircuits.cpp`, run `./run.sh` to generate everything.