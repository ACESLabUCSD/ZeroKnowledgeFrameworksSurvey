FROM python:3.10-slim-bullseye

WORKDIR /app

COPY . /app

# Upgrade pip to the latest version
RUN pip install --upgrade pip

# Install the picozk library along with optional dependencies for examples
RUN pip install .[examples]
