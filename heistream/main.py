from audioop import add
from nfstream import NFStreamer
import psutil
import logging


def main(output_dir: str, interface: str):
    logging.info(f"Start collecting flows on {interface}.")

    logging.debug(f"Check if {interface} exists on this machine.")
    addrs = psutil.net_if_addrs()
    if not interface in addrs:
        logging.error(f"Interface {interface} does not exist on this machine.")
        raise ValueError(f"Interface {interface} does not exist")

    logging.debug(f"Create NFStream live capture.")
    NFStreamer(source=interface,
               statistical_analysis=True,
               performance_report=1,
               system_visibility_poll_ms=100,
               system_visibility_mode=1).to_csv(output_dir)

    logging.info(f"Collect flows.")
