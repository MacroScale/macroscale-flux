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
    mingw-w64-x86_64-clang \
    mingw-w64-x86_64-toolchain \
# contains dbghelp lib for mini dumps
    mingw-w64-winpthreads

USER root

WORKDIR /

CMD ["/bin/bash"]
