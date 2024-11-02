FROM rust:latest

CMD ["/bin/bash"]

ARG DEBIAN_FRONTEND=noninteractive

ENV UDEV=ON 

RUN apt-get update && apt-get install -y \
    locales \
    git \
    udev \
    g++ \
    pkg-config \
    libx11-dev \
    libasound2-dev \
    libudev-dev \
    build-essential \
    curl \
    libxkbcommon-x11-0 \
    libxcursor-dev \
    libxrandr-dev \
    libxi-dev \
    mesa-utils \
    mesa-vulkan-drivers \
    x11-apps \
    lld \
    clang \
&& rm -rf /var/lib/apt/lists/*

ENV LC_ALL=en_US.UTF-8
ENV LANG=en_US.UTF-8
ENV LANGUAGE=en_US.UTF-8

RUN echo "en_US.UTF-8 UTF-8" | tee -a /etc/locale.gen
RUN locale-gen en_US.UTF-8

RUN useradd -rm -d /home/dev -s /bin/bash -u 1000 dev
RUN usermod -aG plugdev dev


RUN curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey | gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg \
        && curl -s -L https://nvidia.github.io/libnvidia-container/stable/deb/nvidia-container-toolkit.list | \
        sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#g' | \
        tee /etc/apt/sources.list.d/nvidia-container-toolkit.list
RUN apt-get update && apt-get install -y \
    nvidia-container-toolkit \
    && rm -rf /var/lib/apt/lists/*   
RUN nvidia-ctk runtime configure --runtime=docker

# AS ROOT
USER root:root
WORKDIR /root

# AS DEV
USER dev:dev
WORKDIR /home/dev

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN rustup component add llvm-tools-preview
RUN rustup default nightly
ENV PATH="/dev/.cargo/bin:${PATH}"


