from platform import system

# trunk-ignore(bandit/B404)
from subprocess import CalledProcessError, run

from click import echo
from dns.dnssec import ValidationFailure, validate
from dns.message import make_query
from dns.name import from_text
from dns.query import udp
from dns.rdatatype import DNSKEY, NS, A
from dns.resolver import resolve, resolve_at


def dns_lookup(query: str, record_type: str, server: str, root: bool) -> str:
    answer = resolve_at(server, query, record_type)
    return answer.rrset


def dns_sec() -> str:
    response = resolve("dave.io", NS)
    nsname = response.rrset[0].to_text()  # name
    response = resolve(nsname, A)
    nsaddr = response.rrset[0].to_text()  # IPv4
    request = make_query("dave.io", DNSKEY, want_dnssec=True)
    response = udp(request, nsaddr)
    if response.rcode() != 0:
        return "QUERY FAILED (SERVER ERROR OR NO DNSKEY RECORD)"
    answer = response.answer  # two RRSET: DNSKEY and RRSIG(DNSKEY)
    if len(answer) != 2:
        return "SOMETHING WENT WRONG"
    name = from_text("dave.io")
    try:
        validate(answer[0], answer[1], {name: answer[0]})
    except ValidationFailure:
        return "DNSKEY VALIDATION FAILED"
    else:
        return "DNSKEY VALIDATED OK"


def dns_flush() -> None:
    commands = {
        "windows": "ipconfig /flushdns",
        "darwin": "sudo dscacheutil -flushcache; sudo killall -HUP mDNSResponder",
        "linux": "sudo systemd-resolve --flush-caches",
    }
    os_name = system().lower()
    try:
        if os_name in commands.keys():
            if os_name == "darwin":
                real_os_name = "Darwin (macOS)"
            else:
                real_os_name = os_name.capitalize()
            echo(f"{real_os_name} detected, executing: {commands[os_name]}", err=True)
            result = run(
                commands[os_name],
                shell=True,  # trunk-ignore(bandit/B602): hardcoded commands
                capture_output=True,
                text=True,
            )
            echo(f"DNS cache flushed successfully.\n{result.stdout}", err=True)
        else:
            echo(f"Unsupported operating system: {os_name}", err=True)
    except CalledProcessError as e:
        echo(f"Error flushing DNS cache: {e.stderr}", err=True)
