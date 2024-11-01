FROM archlinux:base-devel

COPY pacman.conf /etc/pacman.conf

RUN pacman -Syu --noconfirm
RUN pacman -S --noconfirm --needed --overwrite '*' bash rustup gcc llvm pkgconf

COPY run.bash /run.bash

# Setup builder user, arch prevents running makepkg as root
RUN useradd -m builder
RUN echo 'builder ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers

# Create the rustup home directory and set permissions
RUN mkdir -p /github/home/.rustup
RUN chown -R builder:builder /github/home

# patch the build flags
RUN sed -i 's,#MAKEFLAGS="-j2",MAKEFLAGS="-j$(nproc)",g' /etc/makepkg.conf

# Setup builder user
USER builder

ENTRYPOINT ["/run.bash"]
