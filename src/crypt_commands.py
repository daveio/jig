from codecs import encode
from secrets import choice, token_hex
from string import ascii_letters, digits
from sys import stdin, stdout
from textwrap import dedent

from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric.x25519 import X25519PrivateKey

from config import get_key
from cryptor import Cryptor


class WireguardKeypair:
    def __init__(self, private: str, public: str) -> None:
        self.private: str = private
        self.public: str = public

    def __repr__(self) -> str:
        return f"Keypair(private={self.private}, public={self.public})"


def crypt_random_hex(length: int) -> str:
    return token_hex(length)


def crypt_random_pw(length: int) -> str:
    valid_punctuation = "-_.@#$%&*+=:"
    alphabet = ascii_letters + digits + valid_punctuation
    pwd = choice(digits)
    pwd += "".join(choice(alphabet) for _ in range(length - 2))
    pwd += choice(valid_punctuation)
    return pwd


def crypt_simple_decrypt() -> None:
    key = get_key()
    ciphertext = stdin.buffer.read().decode("utf-8")
    plaintext = Cryptor(key).decrypt(ciphertext, stringify=False)
    stdout.buffer.write(plaintext)
    stdout.flush()


def crypt_simple_encrypt() -> str:
    key = get_key()
    plaintext = stdin.buffer.read()
    return Cryptor(key).encrypt(plaintext, wrap=True)


def crypt_simple_key() -> str:
    return Cryptor.keygen(raw=False)


def crypt_wireguard(script: bool) -> str:
    encoding = serialization.Encoding.Raw
    priv_format = serialization.PrivateFormat.Raw
    pub_format = serialization.PublicFormat.Raw
    private_key = X25519PrivateKey.generate()
    private_bytes = private_key.private_bytes(
        encoding=encoding,
        format=priv_format,
        encryption_algorithm=serialization.NoEncryption(),
    )
    private_text = encode(private_bytes, "base64").decode("utf8").strip()
    public_bytes = private_key.public_key().public_bytes(
        encoding=encoding, format=pub_format
    )
    public_text = encode(public_bytes, "base64").decode("utf8").strip()
    keypair = WireguardKeypair(private_text, public_text)
    if script:
        return f"{keypair.private} {keypair.public}"
    return dedent(
        f"""
        Private key : {keypair.private}
        Public key  : {keypair.public}
        """
    )
