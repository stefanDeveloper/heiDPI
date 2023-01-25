[![GitHub Stars](https://img.shields.io/github/stars/stefanDeveloper/heistream)](https://github.com/stefanDeveloper/heistream/) [![Docker Pulls](https://img.shields.io/docker/pulls/stefan96/heistream-ndpid.svg)](https://hub.docker.com/r/stefan96/heistream-ndpid/) [![Docker Stars](https://img.shields.io/docker/stars/stefan96/heistream-ndpid.svg)](https://hub.docker.com/r/stefan96/heistream-ndpid/)

# heiStream

nDPId Docker Image

## How to use this image

```bash
docker pull 
docker run -p 127.0.0.1:7000:7000 -net host stefan96/heistream-ndpid:producer-latest
docker run -net host stefan96/heistream-ndpid:consumer-latest
```

## 

