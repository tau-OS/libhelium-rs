ARG VARIANT="36"
FROM fedora:${VARIANT}

RUN useradd -m vscode
RUN echo vscode ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/vscode
RUN dnf -y install rpm-ostree gcc
RUN echo -e "[tauOS]\nname=tauOS\nbaseurl=https://repo.tauos.co/releases/1.1/\nenabled=1" > /etc/yum.repos.d/tau.repo
RUN rpm --import https://tauos.co/gpg.asc
RUN dnf -y install libhelium-devel git
USER vscode
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y