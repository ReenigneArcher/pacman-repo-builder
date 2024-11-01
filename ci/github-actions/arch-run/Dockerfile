from archlinux:base-devel

copy pacman.conf /etc/pacman.conf

run pacman -Syu --noconfirm
run pacman -S --noconfirm --needed --overwrite '*' bash rustup gcc llvm pkgconf

copy run.bash /run.bash

# create builder user
run useradd -m builder
user builder

entrypoint ["/run.bash"]
