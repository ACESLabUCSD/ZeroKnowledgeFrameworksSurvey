# SoK: Zero Knowledge Frameworks

This is a supplement to our [SoK on Zero-Knowledge Frameworks](). In this repo, we provide open-source environments for all of the frameworks we discuss in our SoK, plus a few more.

We introduce this as a work to lower the barrier of entry for ZKP developers. Our goal with this repository is to eliminate all of the prep work for new developers, which is often an arduous process. Using our environments and provided benchmarks, the developers can immediately start developing custom apps with our provided examples as a syntax guide.

This work is under active development - we plan on adding more environments and keeping things updated as much as we can. Currently, we provide a few environments for works that we do not discuss in our paper.

## Entering Docker environments

For all environments, we provide detailed instructions for how to build our benchmarks and build your own custom circuits. If you want to enter any of these environments on your own terminal to look around, run the following command:
```
docker run -it <env>
```

## Benchmarks
For the works that we discuss in our paper, we provide the following benchmarks:

**Matrix Multiplication**: Verifying public product C of private NxN matrices A and B

**SHA-256**: Verifying public hash of private input

We do note that some of the frameworks we use are in active development, so the examples may break. If you catch this, please let us know so we can update our work.

All of our benchmarks were done on a 128GB RAM AMD Ryzen CPU Desktop. ZKPs are memory intensive, so if you are computing on a less powerful machine, some of these benchmarks may not work. We have parametrized the matrix multiplication benchmark so that you can vary the matrix size N, which should help you find how memory-intensive of a program your machine allows per framework.

## Contributing
If you have an improvement you'd like to make, a new framework to add, or a correction you'd like to suggest, please submit a pull request. We also encourage developers to publish their own custom, non-proprietary applications to the framework folders to grow the scope of this work!

If you believe your framework has been misrepresented (or not represented at all), please email me at [nsheyban@ucsd.edu](mailto:nsheyban@ucsd.edu).

## Future Work and Adjacent Works

As we said, we aim to lower the barrier of entry for ZK developers. Our SoK focuses on the performance, usability, and accessibility of these frameworks, providing developers that are new to ZK a great starting point to choose the right ZKP construction and framework for their applications. Our aim is to keep growing this repository, adding new frameworks and updating the included frameworks, as the ZK landscape evolves.

In an adjacent effort to lower the barrier of entry to ZKPs, we also have an application-focused [website](https://practical-zk.github.io). We encourage developers of novel applications or authors of cutting-edge work to contribute to this website!

## Reference
If our work has been useful to you, please cite us!
```

```

## TODO
- Add which domains each framework excels in
    - Do the same for schemes
- Future proof the codebase (use custom forks for each framework)
- Fix some of the frameworks that have been updated
- Add benchmarks for frameworks that are not in SoK
- Build `zkinterface` pipeline for better interoperability (and to get `libspartan` working)
- Add further documentation for `Rosetta`
- Add some pictures and graphs
<<<<<<< HEAD
- Add build workflows to automate docker error discovery
=======
>>>>>>> abbf69a23c86a90d0ccf913fc3b724f813642461
