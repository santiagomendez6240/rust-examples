## ---------------------------------------------------------------------
## IMPORTANT: This Dockerfile is not intended to be used in production. 
## ---------------------------------------------------------------------

FROM rust:latest

# Set the working directory
WORKDIR /usr/src/app
COPY . /usr/src/app
# Install cargo-watch for auto-reloading
# Install cargo-tarpaulin to generate test coverage
#RUN cargo install cargo-watch && cargo install cargo-tarpaulin
#RUN apt-get update && apt-get install vim -y

# Keep the container running
CMD ["tail", "-f", "/dev/null"]
#CMD ["uname", "-a"]