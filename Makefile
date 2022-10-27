test:
  cargo test

run-server:
  cargo run --bin zkp-server

run-client:
  cargo run --bin zkp-client


docker: 
  docker build docker/Dockerfile-client
  docker build docker/Dockerfile-server