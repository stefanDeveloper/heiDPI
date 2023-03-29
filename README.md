![heiFIP Logo](https://raw.githubusercontent.com/stefanDeveloper/heiDPI/main/assets/heidpi_logo.png?raw=true)


--------------------------------------------------------------------------------

[nDPId](https://github.com/utoni/nDPId) Docker Image for deep packet inspection. As described in [nDPId](https://github.com/utoni/nDPId/blob/main/README.md), we split the image into producer and consumer image for a more generic purpose. For the producer, the image starts the UNIX and UDP socket and nDPId respectively. Via environment variables, users can adapt the nDPId daemon and nDPIsrvd. As by now, we support all current nDPId parameters.

## Conumer

<table>
<tr>
  <td><b>Live Notebook</b></td>
  <td>
    <a href="https://mybinder.org/v2/gh/heidpi/heiDPI-tutorials/main?filepath=demo_notebook.ipynb">
    <img src="https://img.shields.io/badge/notebook-launch-blue?logo=jupyter&style=for-the-badge" alt="live notebook" />
    </a>
  </td>
</tr>
<tr>
  <td><b>Latest Release</b></td>
  <td>
    <a href="https://pypi.python.org/pypi/heidpi">
    <img src="https://img.shields.io/pypi/v/heidpi.svg?logo=pypi&style=for-the-badge" alt="latest release" />
    </a>
  </td>
</tr>
<tr>
  <td><b>Supported Versions</b></td>
  <td>
    <a href="https://pypi.org/project/heidpi/">
    <img src="https://img.shields.io/pypi/pyversions/heidpi?logo=python&style=for-the-badge" alt="python3" />
    </a>
    <a href="https://pypi.org/project/heidpi/">
    <img src="https://img.shields.io/badge/pypy-3.7%20%7C%203.8%20%7C%203.9-blue?logo=pypy&style=for-the-badge" alt="pypy3" />
    </a>
  </td>
</tr>
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

## Producer

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

## Configuration

For a more detail view on how to customize your images, see:

- [Producer](https://github.com/stefanDeveloper/heiDPI/blob/main/README.producer.md)
- [Consumer](https://github.com/stefanDeveloper/heiDPI/blob/main/README.consumer.md)

## License

This project is licensed under the GPL-3.0 license - see the [LICENSE.md](https://github.com/stefanDeveloper/heiDPI/blob/main/LICENSE) file for details.
