Producer

[![Build](https://github.com/stefanDeveloper/heidpi/actions/workflows/docker-publish-producer.yml/badge.svg)](https://github.com/stefanDeveloper/heidpi/actions/workflows/docker-publish-producer.yml) [![GitHub Stars](https://img.shields.io/github/stars/stefanDeveloper/heidpi)](https://github.com/stefanDeveloper/heidpi/) [![Docker Pulls](https://img.shields.io/docker/pulls/stefan96/heidpi-producer.svg)](https://hub.docker.com/r/stefan96/heidpi-producer/) ![Docker Stars](https://img.shields.io/docker/stars/stefan96/heidpi-producer)

Consumer

[![Build](https://github.com/stefanDeveloper/heidpi/actions/workflows/docker-publish-consumer.yml/badge.svg)](https://github.com/stefanDeveloper/heidpi/actions/workflows/docker-publish-consumer.yml) [![GitHub Stars](https://img.shields.io/github/stars/stefanDeveloper/heidpi)](https://github.com/stefanDeveloper/heidpi/) [![Docker Pulls](https://img.shields.io/docker/pulls/stefan96/heidpi-consumer.svg)](https://hub.docker.com/r/stefan96/heidpi-consumer/) ![Docker Stars](https://img.shields.io/docker/stars/stefan96/heidpi-consumer)


# heiDPI - nDPId Docker Image

nDPId Docker Image for deep packet inspection. As described in [nDPId](https://github.com/utoni/nDPId/blob/main/README.md), we split the image into producer and consumer image for a more generic purpose. For the producer, the image starts the UNIX and UDP socket and nDPId respectively. Via environment variables, users can adapt the nDPId daemon and nDPIsrvd. As by now, we support all current nDPId parameters.

## Getting Started


### Prerequisities


In order to run this container you'll need docker installed.

* [Windows](https://docs.docker.com/windows/started)
* [OS X](https://docs.docker.com/mac/started/)
* [Linux](https://docs.docker.com/linux/started/)

### Usage

Pull images:

```sh
docker pull stefan96/heidpi-producer:main
docker pull stefan96/heidpi-consumer:main
```

Run producer and consumer separately from each other using UDP socket:

```sh
docker run -p 127.0.0.1:7000:7000 --net host stefan96/heidpi-producer:main
docker run -e HOST=127.0.0.1 --net host stefan96/heidpi-consumer:main
```

or use the `docker-compose.yml`:

```sh
docker-compose up
```

Additionally, you use a UNIX socket:

```sh
docker run -v ${PWD}/heidpi-data:/tmp/ --net host stefan96/heidpi-producer:main
docker run -v ${PWD}/heidpi-data:/tmp/ -v ${PWD}/heidpi-logs:/var/log -e UNIX=/tmp/nDPIsrvd-daemon-distributor.sock --net host stefan96/heidpi-consumer:main
```

## Configuration

For a more detail view on how to customize your images, see:

- [Producer](./README.producer.md)
- [Consumer](./README.consumer.md)

## License

This project is licensed under the GPL-3.0 license - see the [LICENSE.md](LICENSE.md) file for details.
