#!/usr/bin/env python3
import sys
import os
import json
import stat

import nDPIsrvd
from nDPIsrvd import nDPIsrvdSocket

DEFAULT_HOST = '127.0.0.1'
DEFAULT_PORT = 7000
DEFAULT_UNIX = '/tmp/ndpid-distributor.sock'

JSON_PATH = os.getenv("JSON_PATH", "/var/log/nDPIdsrvd.json")

UNIX = os.getenv("UNIX", "")
HOST = os.getenv("HOST", "")
PORT = os.getenv("PORT", 7000)

SHOW_FLOW_EVENTS = os.getenv("SHOW_FLOW_EVENTS", True)
SHOW_PACKET_EVENTS = os.getenv("SHOW_PACKET_EVENTS", False)
SHOW_ERROR_EVENTS = os.getenv("SHOW_ERROR_EVENTS", False)
SHOW_DAEMON_EVENTS = os.getenv("SHOW_DAEMON_EVENTS", False)


def onJsonLineRecvd(json_dict, instance, current_flow, global_user_data):
    if (SHOW_ERROR_EVENTS and "error_event_id" in json_dict) or (SHOW_PACKET_EVENTS and "packet_event_id" in json_dict) or (SHOW_FLOW_EVENTS and "flow_event_id" in json_dict) or (SHOW_DAEMON_EVENTS and "daemon_event_id" in json_dict):
        with open(JSON_PATH, "a") as f:
            json.dump(json_dict, f)
            f.write("\n")
    return True

def validateAddress():
    tcp_addr_set = False
    address = None

    if HOST is "":
        address_tcpip = (DEFAULT_HOST, PORT)
    else:
        address_tcpip = (HOST, PORT)
        tcp_addr_set = True

    if UNIX is "":
        address_unix = DEFAULT_UNIX
    else:
        address_unix = UNIX

    possible_sock_mode = 0
    try:
        possible_sock_mode = os.stat(address_unix).st_mode
    except:
        pass
    if tcp_addr_set == False and stat.S_ISSOCK(possible_sock_mode):
        address = address_unix
    else:
        address = address_tcpip

    return address

if __name__ == '__main__':
    address = validateAddress()
    print(address)

    sys.stderr.write('Recv buffer size: {}\n'.format(
        nDPIsrvd.NETWORK_BUFFER_MAX_SIZE))
    sys.stderr.write('Connecting to {} ..\n'.format(
        address[0]+':'+str(address[1]) if type(address) is tuple else address))

    nsock = nDPIsrvdSocket()
    nsock.connect(address)
    nsock.loop(onJsonLineRecvd, None, None)
