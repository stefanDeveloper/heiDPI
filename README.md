[![Build](https://github.com/stefanDeveloper/heiStream/actions/workflows/docker-image.yml/badge.svg)](https://github.com/stefanDeveloper/heiStream/actions/workflows/docker-image.yml) [![GitHub Stars](https://img.shields.io/github/stars/stefanDeveloper/heistream)](https://github.com/stefanDeveloper/heistream/) [![Docker Pulls](https://img.shields.io/docker/pulls/stefan96/heistream-ndpid.svg)](https://hub.docker.com/r/stefan96/heistream-ndpid/) ![Docker Stars](https://img.shields.io/docker/stars/stefan96/heistream-ndpid)


# heiStream - nDPId Docker Image

nDPId Docker Image for deep packet inspection. As described in [nDPId](https://github.com/utoni/nDPId/blob/main/README.md), we split the image into producer and consumer image for a more generic purpose. For the producer, the image starts the UNIX and UDP socket and nDPId respectively. Via environment variables, users can   

## Getting Started


### Prerequisities


In order to run this container you'll need docker installed.

* [Windows](https://docs.docker.com/windows/started)
* [OS X](https://docs.docker.com/mac/started/)
* [Linux](https://docs.docker.com/linux/started/)

### Usage

Pull images:

```sh
docker pull stefan96/heistream-ndpid:producer-latest
docker pull stefan96/heistream-ndpid:consumer-latest
```

Run producer and consumer separately from each other using UDP socket:

```sh
docker run -p 127.0.0.1:7000:7000 --net host stefan96/heistream-ndpid:producer-latest
docker run -e HOST=127.0.0.1 --net host stefan96/heistream-ndpid:consumer-latest
```

or use the `docker-compose.yml`:

```sh
docker-compose up
```

Additionally, you use a UNIX socket:

```sh
docker run -v ${PWD}/heistream-data:/tmp/ --net host stefan96/heistream-ndpid:producer-latest
docker run -v ${PWD}/heistream-data:/tmp/ -v ${PWD}/heistream-logs:/var/log -e UNIX=/tmp/nDPIsrvd-daemon-distributor.sock --net host stefan96/heistream-ndpid:consumer-latest
```

## Environment Variables

### Producer

| Variable                     | Type    | Default           |
|------------------------------|---------|-------------------|
| `INTERFACE` | `string` | |
| `PORT` | `int` | 7000 |
| `MAX_THREADS` | `int` | 4 |
| `FLOW_ANALYSIS` | `boolean` | false |
| `JA3_URL` | `string` | https://sslbl.abuse.ch/blacklist/ja3_fingerprints.csv |
| `SSL_SHA1_URL` | `string` | https://sslbl.abuse.ch/blacklist/sslblacklist.csv |
| `TUNE_PARAM` | `string` | |

For `TUNE_PARAM`, concatinate the subopts below like `-o max-flows-per-thread=2024 -o ....`

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
| `SHOW_ERROR_EVENTS` | `boolean` | False |
| `SHOW_DAEMON_EVENTS` | `boolean` | False |
| `SHOW_PACKET_EVENTS` | `boolean` | False |
| `SHOW_FLOW_EVENTS` | `boolean` | True |
| `MAX_BUFFERED_LINES` | `int` | 1024 |

## License

This project is licensed under the GPL-3.0 license - see the [LICENSE.md](LICENSE.md) file for details.
