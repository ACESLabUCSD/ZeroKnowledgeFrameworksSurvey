# Zokrates

We provide a development environment for [Zokrates](https://zokrates.github.io/introduction.html) circuits. All you have to do is make a folder for your new program and add a `.zok` file to `src/`. You'll have to tweak the Dockerfile to specify your inputs. Running `./run.sh` will generate everything and copy it to your local machine. 

We are currently testing an approach to generate `.zkif` [zkInterface](https://github.com/QED-it/zkinterface) files with Zokrates, primarily for `libspartan`, but also for other frameworks that do not provide a high-level API.
