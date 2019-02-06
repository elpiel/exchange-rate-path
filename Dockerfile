FROM rust:stretch

# Some of the dependencies I need to build a few libraries,
# personalize to your needs. You can use multi-stage builds
# to produce a lightweight image.
RUN apt-get update && \
    apt-get install -y cmake libelf-dev libdw-dev binutils-dev libiberty-dev

# Install KCOV
ENV KCOV_VERSION 36
RUN wget https://github.com/SimonKagstrom/kcov/archive/v$KCOV_VERSION.tar.gz && \
    tar xzf v$KCOV_VERSION.tar.gz && \
    rm v$KCOV_VERSION.tar.gz && \
    cd kcov-$KCOV_VERSION && \
    mkdir build && cd build && \
    cmake .. && make && make install && \
    cd ../.. && rm -rf kcov-$KCOV_VERSION

# Stable as default
RUN rustup default stable

# Add rustfmt
RUN rustup component add rustfmt

# Install Cargo Make
RUN cargo install cargo-make

#ENV RUSTFLAGS "-C link-dead-code"
#ENV CFG_RELEASE_CHANNEL "stable"

#RUN bash -l -c 'echo $(rustc --print sysroot)/lib >> /etc/ld.so.conf'
#RUN bash -l -c 'echo /usr/local/lib >> /etc/ld.so.conf'
#RUN ldconfig
