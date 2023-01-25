#!/bin/bash

# exit script if any command fails (non-zero value)
set -e

/root/nDPIsrvd -p /tmp/nDPIsrvd-daemon.pid \
            -c /tmp/nDPIsrvd-daemon-collector.sock \
            -s /tmp/nDPIsrvd-daemon-distributor.sock \
            -S 0.0.0.0:7000 \
            -u root \
            -d \
            -L /tmp/nDPIsrvd.log \
            -C $MAX_BUFFERED_LINES

exec /root/nDPId -p /tmp/nDPId-daemon.pid \
            -c /tmp/nDPIsrvd-daemon-collector.sock \
            -u root \
            -L /tmp/nDPId.log \
            -A \
            -o max-reader-threads=$MAX_THREADS
