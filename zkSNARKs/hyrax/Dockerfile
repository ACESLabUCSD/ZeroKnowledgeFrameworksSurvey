ARG UBUNTU_VERSION=18.04
FROM ubuntu:$UBUNTU_VERSION

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install \
    -y wget unzip curl --no-install-recommends --no-install-suggests vim git \ 
    cmake build-essential cmake git libgmp3-dev libboost-all-dev libssl-dev pkg-config libssl-dev libgmp-dev libtool libtool-bin python2.7.x python-pip python-cffi pypy automake

# RUN python3 -m pip install cffi
# RUN python3 -m pip install pypy

RUN mkdir -p $HOME/toolchains/src && cd $HOME/toolchains/src && wget http://shoup.net/ntl/ntl-10.5.0.tar.gz && tar xzvf ntl-10.5.0.tar.gz && cd ntl-10.5.0/src && ./configure DEF_PREFIX=/usr PREFIX=$HOME/toolchains SHARED=on NTL_THREAD_BOOST=on NTL_STD_CXX11=on && make -j4 && make install

RUN git clone https://github.com/hyraxZK/hyraxZK.git

WORKDIR hyraxZK

RUN git config --global url.https://github.com:/.insteadOf git://github.com/

RUN git submodule update --init --recursive

# RUN mkdir m4 && mkdir libpws/m4 && mkdir pylaurent/m4 && mkdir pymiracl/m4

RUN make -j4

COPY matmul/run_fennel.py fennel/run_fennel.py

COPY matmul/ matmul/

WORKDIR matmul

RUN python matmult.py 32

RUN chmod +x run.sh && ./run.sh

RUN chmod +x process.sh && ./process.sh

RUN chmod +x build.sh && ./build.sh

WORKDIR ..

COPY sha256/ sha256/

WORKDIR sha256

RUN python sha256gen.py -4

RUN chmod +x run.sh && ./run.sh 4
