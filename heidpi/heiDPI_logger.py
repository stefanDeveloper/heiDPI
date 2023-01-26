import argparse
import sys
import os
import json
import stat
from datetime import datetime
import logging

from heidpi import App
from heidpi import heiDPIsrvd

DEFAULT_HOST = '127.0.0.1'
DEFAULT_PORT = 7000
DEFAULT_UNIX = '/tmp/ndpid-distributor.sock'

global SHOW_ERROR_EVENTS
global SHOW_PACKET_EVENTS
global SHOW_FLOW_EVENTS
global SHOW_DAEMON_EVENTS
global JSON_PATH

def dir_path(string):
    if os.path.isdir(string):
        return string
    else:
        raise NotADirectoryError(string)

def file_path(string):
    if os.path.exists(string):
        return string
    else:
       raise NotADirectoryError(string)

def onJsonLineRecvd(json_dict, instance, current_flow, global_user_data):
    date_time = datetime.fromtimestamp(datetime.now().timestamp())
    str_date_time = date_time.strftime(App.config()["logging"]["datefmt"].get())

    json_dict['timestamp'] = str_date_time
    if SHOW_ERROR_EVENTS and ("error_event_id" in json_dict):
        with open(f'{JSON_PATH}/error_event.json', "a") as f:
            json.dump(json_dict, f)
            f.write("\n")
    if SHOW_PACKET_EVENTS and ("packet_event_id" in json_dict):
        if json_dict["packet_event_name"] in App.config()["packet_event"]["packet_event_name"].get():
            if App.config()["packet_event"]["irngore_fields"] != []:   
                list(map(json_dict.pop, App.config()["packet_event"]["ignore_fields"].get()))
        with open(f'{JSON_PATH}/packet_event.json', "a") as f:
            json.dump(json_dict, f)
            f.write("\n")
    if SHOW_FLOW_EVENTS and ("flow_event_id" in json_dict):
        if json_dict["flow_event_name"] in App.config()["flow_event"]["flow_event_name"].get():
            if App.config()["flow_event"]["irngore_fields"] != []:   
                list(map(json_dict.pop, App.config()["flow_event"]["ignore_fields"].get()))
            with open(f'{JSON_PATH}/flow_event.json', "a") as f:
                json.dump(json_dict, f)
                f.write("\n")
    if SHOW_DAEMON_EVENTS and ("daemon_event_id" in json_dict):
        with open(f'{JSON_PATH}/daemon_event.json', "a") as f:
            json.dump(json_dict, f)
            f.write("\n")
    return True


def validateAddress(args):
    tcp_addr_set = False
    address = None

    if args.host is None:
        address_tcpip = (DEFAULT_HOST, args.port)
    else:
        address_tcpip = (args.host, args.port)
        tcp_addr_set = True

    if args.unix is None:
        address_unix = DEFAULT_UNIX
    else:
        address_unix = args.unix

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
    parser = argparse.ArgumentParser(description='heiDPI Python Interface', formatter_class=argparse.ArgumentDefaultsHelpFormatter)
    parser.add_argument('--host', type=str, help='nDPIsrvd host IP')
    parser.add_argument('--port', type=int, default=DEFAULT_PORT, help='nDPIsrvd TCP port')
    parser.add_argument('--unix', type=str, help='nDPIsrvd unix socket path')
    parser.add_argument('--write', type=dir_path, default='/var/log', help='heiDPI write path for logs')

    #parser.add_argument('--config', type=dir_path, default='./config.yml', help='heiDPI write path for logs')

    parser.add_argument('--show-daemon-events', type=bool, default=False, help='heiDPI shows daemon events')
    parser.add_argument('--show-packet-events', type=bool, default=False, help='heiDPI shows packet events')
    parser.add_argument('--show-error-events', type=bool, default=False, help='heiDPI shows error events')
    parser.add_argument('--show-flow-events', type=bool, default=True, help='heiDPI shows flow events')

    args = parser.parse_args()
    address = validateAddress(args)

    App("config.yml")

    SHOW_ERROR_EVENTS = args.show_error_events
    SHOW_PACKET_EVENTS = args.show_packet_events
    SHOW_FLOW_EVENTS = args.show_flow_events
    SHOW_DAEMON_EVENTS = args.show_daemon_events
    JSON_PATH = args.write

    logging.info('Recv buffer size: {}\n'.format(
        heiDPIsrvd.NETWORK_BUFFER_MAX_SIZE))
    logging.info('Connecting to {} ..\n'.format(
        address[0]+':'+str(address[1]) if type(address) is tuple else address))

    nsock = heiDPIsrvd.nDPIsrvdSocket()
    nsock.connect(address)
    nsock.loop(onJsonLineRecvd, None, None)
