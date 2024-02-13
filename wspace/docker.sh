#build
docker build -t ws -f build.dockerfile .
#run
docker run -it --rm -p 50051:50051 ws