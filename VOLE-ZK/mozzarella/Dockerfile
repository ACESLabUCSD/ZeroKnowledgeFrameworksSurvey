FROM rustlang/rust:nightly

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install \
    -y --no-install-recommends --no-install-suggests vim git

RUN git clone https://github.com/nojansheybani/Mozzarella.git
RUN cd Mozzarella && cargo build --release
WORKDIR Mozzarella/target/release