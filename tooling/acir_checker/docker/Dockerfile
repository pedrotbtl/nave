FROM ubuntu:22.04

RUN apt update
RUN apt install -y wget
RUN wget https://github.com/cvc5/cvc5/archive/refs/tags/cvc5-1.3.0.tar.gz
RUN tar xf cvc5-1.3.0.tar.gz
WORKDIR /cvc5-cvc5-1.3.0
RUN apt update
RUN apt install -y software-properties-common
RUN apt install -y cmake
RUN apt install -y build-essential
RUN apt install -y python3-pip
RUN apt install -y python3.10-venv
RUN apt install -y m4
RUN ./configure.sh --static --auto-download --cocoa --gpl
WORKDIR /cvc5-cvc5-1.3.0/build
RUN make
RUN make check
RUN make install
