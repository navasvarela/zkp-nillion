test:
  cargo test

docker: 
  docker build docker/Dockerfile-client
  docker build docker/Dockerfile-server