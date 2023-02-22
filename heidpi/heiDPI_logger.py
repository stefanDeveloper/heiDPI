import argparse
from concurrent.futures import ThreadPoolExecutor
import os
import json
import stat
import logging
import datetime
import copy
import multiprocessing

from heidpi import App
from heidpi import heiDPIsrvd
from heidpi import heiDPI_env

DEFAULT_HOST = '127.0.0.1'
DEFAULT_PORT = 7000
DEFAULT_UNIX = '/tmp/ndpid-distributor.sock'

def dir_path(string):
    if os.path.isdir(string):
        return string
    else:
        raise NotADirectoryError(string)

def file_path(string):
    if os.path.isfile(string):
        return string
    else:
       raise FileNotFoundError(string)

def get_timestamp():
    date_time = datetime.datetime.fromtimestamp(datetime.datetime.now().timestamp())
    return date_time.strftime(LOGGING_CONFIG["datefmt"])

def heidpi_process_packet_events(json_dict, instance, current_flow, global_user_data):
    POOL_PACKET.submit(heidpi_log_event, PACKET_CONFIG, json_dict, SHOW_PACKET_EVENTS, "packet_event_id", "packet_event_name")
    return True

def heidpi_process_flow_events(json_dict, instance, current_flow, global_user_data):
    POOL_FLOW.submit(heidpi_log_event, FLOW_CONFIG, json_dict, SHOW_FLOW_EVENTS, "flow_event_id", "flow_event_name")
    return True

def heidpi_process_daemon_events(json_dict, instance, current_flow, global_user_data):
    POOL_DAEMON.submit(heidpi_log_event, DAEMON_CONFIG, json_dict, SHOW_DAEMON_EVENTS, "daemon_event_id", "daemon_event_name")
    return True

def heidpi_process_error_events(json_dict, instance, current_flow, global_user_data):
    POOL_ERROR.submit(heidpi_log_event, ERROR_CONFIG, json_dict, SHOW_ERROR_EVENTS, "error_event_id", "error_event_name")
    return True

def heidpi_log_event(config_dict, json_dict, show_event: bool, event_id: str, event_name: str):
    json_dict_copy = copy.deepcopy(json_dict)
    if show_event and (event_id in json_dict_copy):
        if json_dict_copy[event_name] in config_dict[event_name]:
            json_dict_copy['timestamp'] = get_timestamp()
            ignore_fields = config_dict["ignore_fields"]

            if ignore_fields != []:   
                list(map(json_dict_copy.pop, ignore_fields, [None] * len(ignore_fields)))

            with open(f'{JSON_PATH}/{config_dict["filename"]}.json', "a") as f:
                json.dump(json_dict_copy, f)
                f.write("\n")

def heidpi_worker(address, function):

    nsock = heiDPIsrvd.nDPIsrvdSocket()
    nsock.connect(address)
    nsock.loop(function, None, None)

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

def main():
    parser = argparse.ArgumentParser(description='heiDPI Python Interface', formatter_class=argparse.ArgumentDefaultsHelpFormatter)

    group = parser.add_mutually_exclusive_group()
    group.add_argument('--host', type=str, action=heiDPI_env.env_default('HOST'), required=False, help='nDPIsrvd host IP')
    group.add_argument('--unix', type=str, action=heiDPI_env.env_default('UNIX'), required=False, help='nDPIsrvd unix socket path')

    parser.add_argument('--port', type=int, action=heiDPI_env.env_default('PORT'), default=DEFAULT_PORT, help='nDPIsrvd TCP port')

    parser.add_argument('--write', type=dir_path, action=heiDPI_env.env_default('WRITE'), default='/var/log', help='heiDPI write path for logs')

    parser.add_argument('--config', type=file_path, action=heiDPI_env.env_default('CONFIG'), default=f'{os.getcwd()}/config.yml', help='heiDPI write path for logs')

    parser.add_argument('--show-daemon-events', type=int, action=heiDPI_env.env_default('SHOW_DAEMON_EVENTS'), default=0, required=False, help='heiDPI shows daemon events')
    parser.add_argument('--show-packet-events', type=int, action=heiDPI_env.env_default('SHOW_PACKET_EVENTS'), default=0, required=False, help='heiDPI shows packet events')
    parser.add_argument('--show-error-events', type=int, action=heiDPI_env.env_default('SHOW_ERROR_EVENTS'), default=0, required=False, help='heiDPI shows error events')
    parser.add_argument('--show-flow-events', type=int, action=heiDPI_env.env_default('SHOW_FLOW_EVENTS'), default=0, required=False, help='heiDPI shows flow events')

    args = parser.parse_args()
    address = validateAddress(args)

    App(args.config)

    global SHOW_ERROR_EVENTS
    global SHOW_PACKET_EVENTS
    global SHOW_FLOW_EVENTS
    global SHOW_DAEMON_EVENTS
    global JSON_PATH

    SHOW_ERROR_EVENTS = args.show_error_events
    SHOW_PACKET_EVENTS = args.show_packet_events
    SHOW_FLOW_EVENTS = args.show_flow_events
    SHOW_DAEMON_EVENTS = args.show_daemon_events
    JSON_PATH = args.write

    global PACKET_CONFIG
    global FLOW_CONFIG
    global DAEMON_CONFIG
    global ERROR_CONFIG

    PACKET_CONFIG = App.config()["packet_event"].get()
    FLOW_CONFIG = App.config()["flow_event"].get()
    DAEMON_CONFIG = App.config()["daemon_event"].get()
    ERROR_CONFIG = App.config()["error_event"].get()

    global LOGGING_CONFIG

    LOGGING_CONFIG = App.config()["logging"].get()

    logging.info('Recv buffer size: {}'.format(
        heiDPIsrvd.NETWORK_BUFFER_MAX_SIZE))
    logging.info('Connecting to {} ..'.format(
        address[0]+':'+str(address[1]) if type(address) is tuple else address))

    #######################################################################################
    if SHOW_FLOW_EVENTS:
        global POOL_FLOW
        
        POOL_FLOW = ThreadPoolExecutor(max_workers=FLOW_CONFIG['threads'])

        heidpi_flow_job = multiprocessing.Process(
                target=heidpi_worker,
                args=(address, heidpi_process_flow_events))
        heidpi_flow_job.start()

    #######################################################################################
    if SHOW_PACKET_EVENTS:
        global POOL_PACKET
        

        POOL_PACKET = ThreadPoolExecutor(max_workers=PACKET_CONFIG['threads'])
        
        heidpi_packet_job = multiprocessing.Process(
                target=heidpi_worker,
                args=(address, heidpi_process_packet_events))
        heidpi_packet_job.start()

    #######################################################################################
    if SHOW_DAEMON_EVENTS:
        global POOL_DAEMON

        POOL_DAEMON = ThreadPoolExecutor(max_workers=DAEMON_CONFIG['threads'])

        heidpi_daemon_job = multiprocessing.Process(
                target=heidpi_worker,
                args=(address, heidpi_process_daemon_events))
        heidpi_daemon_job.start()


    #######################################################################################
    if SHOW_ERROR_EVENTS:
        global POOL_ERROR
        
        POOL_ERROR = ThreadPoolExecutor(max_workers=ERROR_CONFIG['threads'])

        heidpi_error_job = multiprocessing.Process(
                target=heidpi_worker,
                args=(address, heidpi_process_error_events))
        heidpi_error_job.start()

if __name__ == '__main__':
    main()
