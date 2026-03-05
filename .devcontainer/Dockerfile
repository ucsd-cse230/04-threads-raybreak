FROM mcr.microsoft.com/vscode/devcontainers/base
USER vscode
ENV HOME=/home/vscode

# Install python
RUN sudo apt-get update && sudo apt-get install -y python3 python3-pip python3-dev

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y 

# Add Cargo to PATH
ENV PATH="/home/vscode/.cargo/bin:${PATH}"