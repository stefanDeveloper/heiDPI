[![Build](https://github.com/stefanDeveloper/heidpi/actions/workflows/docker-image.yml/badge.svg)](https://github.com/stefanDeveloper/heidpi/actions/workflows/docker-image.yml) [![GitHub Stars](https://img.shields.io/github/stars/stefanDeveloper/heidpi)](https://github.com/stefanDeveloper/heidpi/) [![Docker Pulls](https://img.shields.io/docker/pulls/stefan96/heidpi.svg)](https://hub.docker.com/r/stefan96/heidpi/) ![Docker Stars](https://img.shields.io/docker/stars/stefan96/heidpi)


# heidpi - nDPId Docker Image

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
docker pull stefan96/heidpi:producer-latest
docker pull stefan96/heidpi:consumer-latest
```

Run producer and consumer separately from each other using UDP socket:

```sh
docker run -p 127.0.0.1:7000:7000 --net host stefan96/heidpi:producer-latest
docker run -e HOST=127.0.0.1 --net host stefan96/heidpi:consumer-latest
```

or use the `docker-compose.yml`:

```sh
docker-compose up
```

Additionally, you use a UNIX socket:

```sh
docker run -v ${PWD}/heidpi-data:/tmp/ --net host stefan96/heidpi:producer-latest
docker run -v ${PWD}/heidpi-data:/tmp/ -v ${PWD}/heidpi-logs:/var/log -e UNIX=/tmp/nDPIsrvd-daemon-distributor.sock --net host stefan96/heidpi:consumer-latest
```

## Environment Variables

### Producer

| Variable                     | Type    | Default           |
|------------------------------|---------|-------------------|
| `INTERFACE` | `string` | |
| `PORT` | `int` | 7000 |
| `MAX_THREADS` | `int` | 4 |
| `FLOW_ANALYSIS` | `boolean` | false |
| `JA3_URL` | `string` | |
| `SSL_SHA1_URL` | `string` | |
| `TUNE_PARAM` | `string` | |

For `TUNE_PARAM`, concatenate the subopts below like `max-flows-per-thread=2024;max-idle-flows-per-thread=64;...`

```
subopts:
    max-flows-per-thread = 2048
    max-idle-flows-per-thread = 64
    max-reader-threads = 16
    daemon-status-interval = 600000000
    flow-scan-interval = 10000000
    generic-max-idle-time = 600000000
    icmp-max-idle-time = 120000000
    udp-max-idle-time = 180000000
    tcp-max-idle-time = 3145032704
    tcp-max-post-end-flow-time = 120000000
    max-packets-per-flow-to-send = 15
    max-packets-per-flow-to-process = 32
    max-packets-per-flow-to-analyse = 32
    error-event-threshold-n = 16
    error-event-threshold-time = 10000000
```

### Consumer

| Variable                     | Type    | Default           |
|------------------------------|---------|-------------------|
| `UNIX` | `string` | |
| `HOST` | `string` | |
| `PORT` | `string` | 7000 |
| `JSON_PATH` | `string` | `/var/log/nDPIdsrvd.json` |
| `SHOW_ERROR_EVENTS` | `boolean` | 0 |
| `SHOW_DAEMON_EVENTS` | `boolean` | 0 |
| `SHOW_PACKET_EVENTS` | `boolean` | 0 |
| `SHOW_FLOW_EVENTS` | `boolean` | 1 |
| `MAX_BUFFERED_LINES` | `int` | 1024 |

## License

This project is licensed under the GPL-3.0 license - see the [LICENSE.md](LICENSE.md) file for details.
