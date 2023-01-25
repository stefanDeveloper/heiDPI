#!/usr/bin/env python3
import sys
import os
import json

import nDPIsrvd
from nDPIsrvd import nDPIsrvdSocket, TermColor

JSON_PATH = os.getenv("JSON_PATH", "/var/log/nDPIdsrvd.json")

def onJsonLineRecvd(json_dict, instance, current_flow, global_user_data):
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

    sys.stderr.write('Recv buffer size: {}\n'.format(nDPIsrvd.NETWORK_BUFFER_MAX_SIZE))
    sys.stderr.write('Connecting to {} ..\n'.format(address[0]+':'+str(address[1]) if type(address) is tuple else address))

    nsock = nDPIsrvdSocket()
    nsock.connect(address)
    nsock.loop(onJsonLineRecvd, None, None)
