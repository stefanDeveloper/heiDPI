![heiFIP Logo](https://raw.githubusercontent.com/stefanDeveloper/heiDPI/main/assets/heidpi_logo.png?raw=true)


--------------------------------------------------------------------------------

A [nDPId](https://github.com/utoni/nDPId/) Docker Image for deep packet inspection. As described in [nDPId README](https://github.com/utoni/nDPId/blob/main/README.md), we split the image into [producer](https://hub.docker.com/r/stefan96/heidpi-producer) and [consumer](https://hub.docker.com/r/stefan96/heidpi-consumer) image for a more generic purpose. For the producer, the image starts the UNIX and UDP socket and nDPId respectively. Via environment variables, users can adapt the nDPId daemon and nDPIsrvd. As by now, we support all current nDPId parameters.

<table>
<tr>
  <td><b>Continuous Integration</b></td>
  <td>
    <a href="https://github.com/stefanDeveloper/heidpi/actions/workflows/docker-publish-producer.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/stefanDeveloper/heidpi/docker-publish-producer.yml?branch=main&logo=docker&style=for-the-badge&label=docker" alt="Docker WorkFlows" />
    </a>
  </td>
</tr>
</table>

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

## License

This project is licensed under the GPL-3.0 license - see the [LICENSE.md](LICENSE.md) file for details.
