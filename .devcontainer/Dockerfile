FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

ARG ANCHOR_CLI="0.29.0"
ARG SOLANA_CLI="1.17.17"
ARG NVM_VERSION="0.39.7"
# Latest node version
ARG NODE_VERSION="node"

USER vscode

SHELL ["/bin/zsh", "-c"]
ENV NVM_DIR /home/vscode/.nvm
RUN mkdir $NVM_DIR

RUN curl -o- https://raw.githubusercontent.com/creationix/nvm/v${NVM_VERSION}/install.sh | bash
RUN source $NVM_DIR/nvm.sh && \
    nvm install ${NODE_VERSION} && \
    nvm use ${NODE_VERSION} && \
    nvm alias default node 

RUN echo 'export NVM_DIR="$HOME/.nvm"' >> ~/.zshrc
RUN echo '[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"' >> ~/.zshrc

RUN curl -o- https://release.solana.com/v${SOLANA_CLI}/install | bash
RUN echo 'PATH="/home/vscode/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.zshrc

RUN source $NVM_DIR/nvm.sh && npm install -g @coral-xyz/anchor-cli@${ANCHOR_CLI}
