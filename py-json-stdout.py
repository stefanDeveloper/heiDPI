#!/usr/bin/env python3
import sys
import os
import json

import nDPIsrvd
from nDPIsrvd import nDPIsrvdSocket

JSON_PATH = os.getenv("JSON_PATH", "test.json")

SHOW_PACKET_EVENTS = os.getenv("SHOW_PACKET_EVENTS", False)
SHOW_FLOW_EVENTS = os.getenv("SHOW_FLOW_EVENTS", True)
SHOW_ERROR_EVENTS = os.getenv("SHOW_ERROR_EVENTS", False)
SHOW_DAEMON_EVENTS = os.getenv("SHOW_DAEMON_EVENTS", False)


def onJsonLineRecvd(json_dict, instance, current_flow, global_user_data):
    if (SHOW_ERROR_EVENTS and "error_event_id" in json_dict) or (SHOW_PACKET_EVENTS and "packet_event_id" in json_dict) or (SHOW_FLOW_EVENTS and "flow_event_id" in json_dict) or (SHOW_DAEMON_EVENTS and "daemon_event_id" in json_dict):
        with open(JSON_PATH, "a") as f:
            json.dump(json_dict, f)
            f.write("\n")
    return True


if __name__ == '__main__':
    argparser = nDPIsrvd.defaultArgumentParser()
    args = argparser.parse_args()
    print(args)
    address = nDPIsrvd.validateAddress(args)
    print(address)

    sys.stderr.write('Recv buffer size: {}\n'.format(
        nDPIsrvd.NETWORK_BUFFER_MAX_SIZE))
    sys.stderr.write('Connecting to {} ..\n'.format(
        address[0]+':'+str(address[1]) if type(address) is tuple else address))

    nsock = nDPIsrvdSocket()
    nsock.connect(address)
    nsock.loop(onJsonLineRecvd, None, None)
