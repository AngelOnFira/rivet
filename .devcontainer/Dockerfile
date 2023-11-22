# # Start from the Nix image
# FROM nixos/nix as builder

# # Install Git
# RUN nix-env -iA nixpkgs.git

# # Clone the Rivet repository
# RUN git clone https://github.com/rivet-gg/rivet.git

# # Change into the repo directory
# WORKDIR /rivet

# # Make sure bolt is set up. We're just using --version to make sure that bolt is
# # set up. (For now it's --help since --version isn't on main)
# RUN nix-shell --run "bolt --help"

FROM mcr.microsoft.com/vscode/devcontainers/base:debian-12

RUN apt-get update \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install \
        git-lfs \
        libssl-dev \
        pkg-config \
        build-essential \
        ca-certificates \
        protobuf-compiler

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install k9s https://github.com/derailed/k9s/releases/download/v0.28.2/k9s_Linux_amd64.tar.gz
# Download, extract and move k9s to /usr/local/bin
RUN curl -LO https://github.com/derailed/k9s/releases/download/v0.28.2/k9s_Linux_amd64.tar.gz \
    && tar -xzf k9s_Linux_amd64.tar.gz \
    && mv k9s /usr/local/bin/ \
    && rm k9s_Linux_amd64.tar.gz

# # Create the nixbld group and add vscode user to it
# RUN groupadd -r nixbld && usermod -a -G nixbld vscode

# # Copy the nix cache from the builder stage
# COPY --from=builder /nix /nix

# # Change the owner of the /nix directory to vscode
# RUN chown -R vscode:vscode /nix

# # Set the working directory
# WORKDIR /workspace
