FROM archlinux:base-devel

RUN pacman -Syu --noconfirm && \ 
    pacman -S --noconfirm \
    git \
    curl

# enable multilib in pacman conf (gets 32 and 64 bit version)
RUN echo -e "\n[multilib]\nInclude = /etc/pacman.d/mirrorlist" >> /etc/pacman.conf

# create a non-root user and add to sudoers
RUN useradd -m builduser && \
    echo "builduser ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

# builduser permissions to directories
RUN mkdir -p /home/builduser/ && \
    chown -R builduser:builduser /home/builduser && \
    chmod -R 755 /home/builduser

RUN pacman -Syu

# switch to non-root user
USER builduser

WORKDIR /home/builduser

# install yay
RUN git clone https://aur.archlinux.org/yay.git && \
    cd yay && makepkg -si --noconfirm && \
    cd .. && rm -rf yay

RUN yay -S --noconfirm mingw-w64-cppwinrt

RUN yay -S --noconfirm clang \
    mingw-w64-x86_64-clang

# download and install cmake 4.0.0 prerelease (has manifest support)
# RUN curl -L https://github.com/Kitware/CMake/releases/download/v4.0.0-rc1/cmake-4.0.0-rc1-linux-x86_64.tar.gz -o cmake.tar.gz && \
#     tar xzvf cmake.tar.gz

USER root

# Symlink CMake binaries to /usr/local/bin so they can be called anywhere
# RUN ln -s /cmake-4.0.0-rc1-linux-x86_64/bin/cmake /usr/local/bin/cmake

WORKDIR /

CMD ["/bin/bash"]
