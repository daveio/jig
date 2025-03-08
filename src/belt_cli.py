import click

from audio_commands import audio_info
from config import get_config_path, get_warned, write_default_config
from crypt_commands import (
    crypt_random_hex,
    crypt_random_pw,
    crypt_simple_decrypt,
    crypt_simple_encrypt,
    crypt_simple_key,
    crypt_wireguard,
)
from cryptor import Cryptor
from dns_commands import dns_flush, dns_lookup, dns_sec
from domain_commands import domain_expiry, domain_ns
from tls_commands import tls_cert_req, tls_cert_selfsign, tls_ciphers


@click.group()
@click.version_option()
def cli() -> None:
    pass


@cli.command()
@click.option(
    "-o",
    "--overwrite",
    is_flag=True,
    show_default=True,
    default=False,
    help="Overwrite existing files without asking for confirmation",
)
def init(overwrite: bool) -> None:
    config_path = get_config_path()
    if config_path.is_file():
        if overwrite:
            write_default_config()
        elif click.confirm(
            f"Config file already exists at {config_path}. Overwrite?", abort=True
        ):
            write_default_config()
    else:
        write_default_config()


# @cli.command()
# def test() -> None:
#     pass


@cli.group()
def audio() -> None:
    pass


@cli.group()
def crypt() -> None:
    pass


@cli.group()
def domain() -> None:
    pass


@cli.group()
def dns() -> None:
    pass


@cli.group()
def tls() -> None:
    pass


@audio.command()
def info(path: click.Path) -> None:
    click.echo(audio_info(path))


@crypt.group()
def random() -> None:  # DevSkim: ignore DS148264
    pass


@random.command()
@click.argument("length", nargs=1, type=int, required=False, default=16)
def hex(length: int) -> None:
    click.echo(crypt_random_hex(length))


@random.command()
@click.argument("length", nargs=1, type=int, required=False, default=16)
def pw(length: int) -> None:
    click.echo(crypt_random_pw(length))


@crypt.group()
def simple() -> None:
    pass


@simple.command()
def decrypt() -> None:
    Cryptor.warn(get_warned())
    crypt_simple_decrypt()


@simple.command()
def encrypt() -> None:
    Cryptor.warn(get_warned())
    click.echo(crypt_simple_encrypt())


@simple.command()
def key() -> None:
    Cryptor.warn(get_warned())
    click.echo(crypt_simple_key())


@crypt.command()
@click.option(
    "-s",
    "--script",
    is_flag=True,
    show_default=True,
    default=False,
    help="Print keys for a script, as PRIVATEKEY PUBLICKEY",
)
def wireguard(script: bool) -> None:
    click.echo(crypt_wireguard(script))


@dns.command()
def flush() -> None:
    dns_flush()


@dns.command()
@click.argument("query", nargs=1, type=str, required=True)
@click.argument("record_type", nargs=1, type=str, required=False, default="A")
@click.option("-s", "--server", type=str, help="DNS server to use", default="1.1.1.1")
@click.option(
    "-r", "--root", is_flag=True, help="Use root servers directly", default=False
)
def lookup(query: str, record_type: str, server: str, root: bool) -> None:
    click.echo(dns_lookup(query, record_type, server, root))


@dns.command()
def sec() -> None:
    click.echo(dns_sec())


@domain.command()
def expiry() -> None:
    click.echo(domain_expiry())


@domain.command()
def ns() -> None:
    click.echo(domain_ns())


@tls.group()
def cert() -> None:
    pass


@cert.command()
def selfsign() -> None:
    click.echo(tls_cert_selfsign())


@cert.command()
def req() -> None:
    click.echo(tls_cert_req())


@tls.command()
def ciphers() -> None:
    click.echo(tls_ciphers())


if __name__ == "__main__":
    cli()
