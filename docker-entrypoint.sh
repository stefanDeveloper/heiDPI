#!/bin/sh

MY_INFO="\
###########################################
###             heiDPI Image            ###
###########################################

Starting services
----------------------------------
"

echo "$MY_INFO"

# Exit on any command failure
set -e

params_ndpisrvd=""
params_ndpid=""

###########################################
### Create params for nDPIsrvd ###
###########################################  

if [ "$MAX_BUFFERED_LINES" -gt 0 ] 2>/dev/null; then
    params_ndpisrvd="$params_ndpisrvd -C $MAX_BUFFERED_LINES"
fi

###########################################
### Create params for nDPId ###
###########################################

url_regex="^\(https\?\|ftp\|file\)://"

case "$JA3_URL" in
    http://* | https://* | ftp://* | file://*)
        curl "$JA3_URL" > /root/ja3_fingerprints.csv
        params_ndpid="$params_ndpid -J /root/ja3_fingerprints.csv"
        ;;
esac

case "$SSL_SHA1_URL" in
    http://* | https://* | ftp://* | file://*)
        curl "$SSL_SHA1_URL" > /root/sslblacklist.csv
        params_ndpid="$params_ndpid -S /root/sslblacklist.csv"
        ;;
esac

if [ -n "$INTERFACE" ]; then
    params_ndpid="$params_ndpid -i $INTERFACE"
fi

if [ "$FLOW_ANALYSIS" = "1" ]; then
    params_ndpid="$params_ndpid -A"
fi

if [ -n "$TUNE_PARAM" ]; then
    OLD_IFS="$IFS"
    IFS=','

    for word in $TUNE_PARAM; do
        params_ndpid="$params_ndpid -o $word"
    done

    IFS="$OLD_IFS"
fi

if [ -n "$PCAP_FILTER" ]; then
    params_ndpid="$params_ndpid -B $PCAP_FILTER"
fi

if [ -n "$NDPI_CUSTOM_PROTOCOLS" ]; then
    params_ndpid="$params_ndpid -P $NDPI_CUSTOM_PROTOCOLS"
fi

if [ -n "$NDPI_CUSTOM_CATEGORIES" ]; then
    params_ndpid="$params_ndpid -C $NDPI_CUSTOM_CATEGORIES"
fi

if [ -n "$HOSTNAME" ]; then
    params_ndpid="$params_ndpid -a $HOSTNAME"
fi

###########################################
### Start nDPIsrvd ###
###########################################

echo "Start nDPIsrvd..."

# Use eval to expand parameter string correctly
eval /root/nDPIsrvd -p /tmp/nDPIsrvd-daemon.pid \
    -c /tmp/nDPIsrvd-daemon-collector.sock \
    -s /tmp/nDPIsrvd-daemon-distributor.sock \
    -S 0.0.0.0:$PORT \
    -u root \
    -d \
    -L /tmp/nDPIsrvd.log \
    $params_ndpisrvd

###########################################
### Start nDPId ###
###########################################

echo "Start nDPId..."

exec /root/nDPId -p /tmp/nDPId-daemon.pid \
    -c /tmp/nDPIsrvd-daemon-collector.sock \
    -u root \
    -L /tmp/nDPId.log \
    $params_ndpid
