FROM debian:testing

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install \
    -y --no-install-recommends --no-install-suggests vim git \ 
    cmake build-essential libssl-dev ca-certificates

RUN apt-get install -y python3
    
RUN git clone --recursive https://github.com/nojansheybani/emp-tool.git && cd emp-tool && \
    cmake . && make && make install 

RUN git clone --recursive https://github.com/emp-toolkit/emp-ot.git && cd emp-ot && \
    cmake . && make && make install 

RUN git clone --recursive https://github.com/emp-toolkit/emp-zk.git && cd emp-zk && \
    cmake . && make && make install 
