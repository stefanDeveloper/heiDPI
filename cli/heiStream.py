try:
    import click
except ImportError:
    raise ImportError(
        "Please install Python dependencies: " "click, colorama (optional)."
    )

from heistream.main import main
from . import CONTEXT_SETTINGS
from heistream.version import __version__

@click.version_option(version=__version__)
@click.group(context_settings=CONTEXT_SETTINGS)
def cli():
    click.secho("Starting heiStream CLI")


@click.option(
    "-w",
    "--write",
    "output_dir",
    type=click.Path(),
    required=True,
    help="Destination file path, stores result",
)
@click.option(
    "-i",
    "--interface",
    "interface",
    type=str,
    required=True,
    help="Interface for live capturing",
)
@cli.command(name="capture")
def capture(output_dir):
    main(output_dir)