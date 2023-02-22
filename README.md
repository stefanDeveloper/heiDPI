
[![Build](https://github.com/stefanDeveloper/heidpi/actions/workflows/docker-publish-producer.yml/badge.svg)](https://github.com/stefanDeveloper/heidpi/actions/workflows/docker-publish-producer.yml) [![GitHub Stars](https://img.shields.io/github/stars/stefanDeveloper/heidpi)](https://github.com/stefanDeveloper/heidpi/) [![Docker Pulls](https://img.shields.io/docker/pulls/stefan96/heidpi-producer.svg)](https://hub.docker.com/r/stefan96/heidpi-producer/) ![Docker Stars](https://img.shields.io/docker/stars/stefan96/heidpi-producer)


[![Build](https://github.com/stefanDeveloper/heidpi/actions/workflows/docker-publish-consumer.yml/badge.svg)](https://github.com/stefanDeveloper/heidpi/actions/workflows/docker-publish-consumer.yml) [![GitHub Stars](https://img.shields.io/github/stars/stefanDeveloper/heidpi)](https://github.com/stefanDeveloper/heidpi/) [![Docker Pulls](https://img.shields.io/docker/pulls/stefan96/heidpi-consumer.svg)](https://hub.docker.com/r/stefan96/heidpi-consumer/) ![Docker Stars](https://img.shields.io/docker/stars/stefan96/heidpi-consumer)


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

## Environment Variables

### Producer

| Variable                     | Type    | Default           |
|------------------------------|---------|-------------------|
| `INTERFACE` | `string` | |
| `PORT` | `int` | 7000 |
| `MAX_THREADS` | `int` | 4 |
| `FLOW_ANALYSIS` | `int` | 0 |
| `JA3_URL` | `string` | |
| `SSL_SHA1_URL` | `string` | |
| `TUNE_PARAM` | `string` | |
| `PCAP_FILTER` | `string` | |
| `NDPI_CUSTOM_PROTOCOLS` | `string` | |
| `NDPI_CUSTOM_CATEGORIES` | `string` | |
| `HOSTNAME` | `string` | |

For `TUNE_PARAM`, concatenate the subopts below like `max-flows-per-thread=2024;max-idle-flows-per-thread=64;...`
As derived from [nDPId Tuning](https://github.com/utoni/nDPId/blob/main/README.md#ndpid-tuning):

 * `max-flows-per-thread` (N, caution advised): affects max. memory usage
 * `max-idle-flows-per-thread` (N, safe): max. allowed idle flows which memory get's free'd after `flow-scan-interval`
 * `max-reader-threads` (N, safe): amount of packet processing threads, every thread can have a max. of `max-flows-per-thread` flows
 * `daemon-status-interval` (ms, safe): specifies how often daemon event `status` will be generated
 * `compression-scan-interval` (ms, untested): specifies how often `nDPId` should scan for inactive flows ready for compression
 * `compression-flow-inactivity` (ms, untested): the earliest period of time that must elapse before `nDPId` may consider compressing a flow that did neither send nor receive any data
 * `flow-scan-interval` (ms, safe): min. amount of time after which `nDPId` will scan for idle or long-lasting flows
 * `generic-max-idle-time` (ms, untested): time after which a non TCP/UDP/ICMP flow will time out
 * `icmp-max-idle-time` (ms, untested): time after which an ICMP flow will time out
 * `udp-max-idle-time` (ms, caution advised): time after which an UDP flow will time out
 * `tcp-max-idle-time` (ms, caution advised): time after which a TCP flow will time out
 * `tcp-max-post-end-flow-time` (ms, caution advised): a TCP flow that received a FIN or RST will wait that amount of time before flow tracking will be stopped and the flow memory free'd
 * `max-packets-per-flow-to-send` (N, safe): max. `packet-flow` events that will be generated for the first N packets of each flow
 * `max-packets-per-flow-to-process` (N, caution advised): max. packets that will be processed by `libnDPI`
 * `max-packets-per-flow-to-analyze` (N, safe): max. packets to analyze before sending an `analyse` event, requires `-A`

### Consumer

| Variable                     | Type    | Default           |
|------------------------------|---------|-------------------|
| `UNIX` | `string` | |
| `HOST` | `string` | |
| `PORT` | `int` | 7000 |
| `JSON_PATH` | `string` | `/var/log/nDPIdsrvd.json` |
| `SHOW_ERROR_EVENTS` | `int` | 0 |
| `SHOW_DAEMON_EVENTS` | `int` | 0 |
| `SHOW_PACKET_EVENTS` | `int` | 0 |
| `SHOW_FLOW_EVENTS` | `int` | 1 |
| `MAX_BUFFERED_LINES` | `int` | 1024 |

### Config file

You can change the default configuration by mounting a config file `/usr/src/app/config.yml`.

```yaml
appName: heiDPI

logging:
  level: INFO
  encoding: utf-8
  format: "%(asctime)s %(levelname)s:%(message)s"
  datefmt: "%Y-%m-%dT%I:%M:%S"
  # filemode: w # a for append, will not override current file
  # filename: heiDPI.log

flow_event:
  ignore_fields: []
  flow_event_name:
    - update
    - end
    - idle
    - detected
  filename: flow_event
  threads: 25

daemon_event:
  ignore_fields: []
  daemon_event_name:
    - init
    - status
  filename: daemon_event
  threads: 25

packet_event:
  ignore_fields: []
  packet_event_name:
    - packet-flow
  filename: packet_event
  threads: 25

error_event:
  ignore_fields: []
  error_event_name:
    - error-flow
  filename: error_event
  threads: 25
```

## License

This project is licensed under the GPL-3.0 license - see the [LICENSE.md](LICENSE.md) file for details.
