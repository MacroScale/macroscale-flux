FROM ubuntu:latest

WORKDIR /

COPY ./docker/buildenv.sh /buildenv.sh

RUN chmod +x buildenv.sh

RUN apt update && \ 
    apt install -y \
    build-essential \
    libtool \
    autoconf \
    unzip \
    curl \
    tree \
    mingw-w64

# download and install cmake 4.0.0 prerelease (has manifest support)
RUN curl -L https://github.com/Kitware/CMake/releases/download/v4.0.0-rc1/cmake-4.0.0-rc1-linux-x86_64.tar.gz -o cmake-4.0.0-rc1-linux-x86_64.tar.gz && \
    tar xzvf cmake-4.0.0-rc1-linux-x86_64.tar.gz

# set PATH for cmake
ENV PATH="cmake-4.0.0-rc1-linux-x86_64/bin:$PATH"

# download and install cppwinrt mingw-w64
# ...

ENTRYPOINT ["/bin/bash", "/buildenv.sh"]
