![heiFIP Logo](https://raw.githubusercontent.com/stefanDeveloper/heiDPI/main/assets/heidpi_logo.png?raw=true)


--------------------------------------------------------------------------------

[nDPId](https://github.com/utoni/nDPId) Docker Image for deep packet inspection. As described in [nDPId](https://github.com/utoni/nDPId/blob/main/README.md), we split the image into producer and consumer image for a more generic purpose. For the producer, the image starts the UNIX and UDP socket and nDPId respectively. Via environment variables, users can adapt the nDPId daemon and nDPIsrvd. As by now, we support all current nDPId parameters.

<table>
<tr>
  <td><b>Project License</b></td>
  <td>
    <a href="https://github.com/stefanDeveloper/heidpi/blob/main/LICENSE">
    <img src="https://img.shields.io/pypi/l/heidpi?logo=gnu&style=for-the-badge&color=blue" alt="License" />
    </a>
  </td>
</tr>
<tr>
  <td><b>Continuous Integration</b></td>
  <td>
    <a href="https://github.com/stefanDeveloper/heidpi/actions/workflows/python-app.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/stefanDeveloper/heidpi/python-app.yml?branch=main&logo=linux&style=for-the-badge&label=linux" alt="Linux WorkFlows" />
    </a>
    <a href="https://github.com/stefanDeveloper/heifip/actions/workflows/docker-publish-consumer.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/stefanDeveloper/heidpi/docker-publish-consumer.yml?branch=main&logo=docker&style=for-the-badge&label=docker" alt="Docker WorkFlows" />
    </a>
  </td>
</tr>
</table>

## Getting Started

Install using PyPi:

```sh
cd ./heidpi-logger && cmake . && make
```

Use the CLI for quick usage:

```
> ./heidpi_cpp -h
usage: heidpi_cpp [-h] [--host HOST | --unix UNIX] [--port PORT] [--write WRITE]
            [--config CONFIG] [--filter FILTER]
            [--show-daemon-events]
            [--show-packet-events]
            [--show-error-events]
            [--show-flow-events]
```

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

You can change the default configuration by mounting a config file `/usr/src/app/config.yml`:

```yaml
flow_event:
  ignore_fields: []
  flow_event_name:
    - update
    - end
    - idle
    - detected
  filename: flow_event
  threads: 25
```

## License

This project is licensed under the GPL-3.0 license - see the [LICENSE.md](LICENSE.md) file for details.
