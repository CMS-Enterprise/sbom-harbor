FROM python:3.9-bullseye

# Install base tools from APT
RUN apt-get update \
  && apt-get install -y --no-install-recommends \
  ca-certificates \
  curl \
  jq \
  git \
  zip

# Install NodeJs
RUN curl -fsSL https://deb.nodesource.com/setup_18.x > ./script && bash ./script && apt install -y nodejs && corepack enable

# Install Poetry
RUN curl https://install.python-poetry.org | python3 && cp /root/.local/bin/poetry /usr/local/bin

# Install pylint
RUN pip install pylint==2.15.5 pylint-exit

# Install AWS CLI
RUN curl https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip -o ./awscliv2.zip && unzip -q ./awscliv2.zip && ./aws/install

COPY ./start.sh .
RUN chmod +x start.sh
RUN ls -l

ENV RUNNER_ALLOW_RUNASROOT=true
CMD ["./start.sh"]
