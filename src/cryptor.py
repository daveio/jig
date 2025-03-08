from os import urandom
from textwrap import dedent

from base58 import b58decode_check, b58encode_check
from click import echo
from cryptography.hazmat.primitives.ciphers.aead import ChaCha20Poly1305
from cryptography.hazmat.primitives.hashes import BLAKE2b, Hash


class Cryptor:
    def __init__(self, key: str) -> None:
        self.key = b58decode_check(key, autofix=True)

    def encrypt(self, plaintext: str | bytes, wrap: bool) -> bytes | str:
        if type(plaintext) is str:
            plaintext = Cryptor.unwrap(plaintext)
        algo = ChaCha20Poly1305(self.key)
        nonce = urandom(12)
        hasher = Hash(BLAKE2b(digest_size=64))
        hasher.update(plaintext)
        data_hash = hasher.finalize()
        encrypted = algo.encrypt(nonce, plaintext, data_hash)
        out = nonce + data_hash + encrypted
        if wrap:
            return Cryptor.wrap(out)
        else:
            return out

    def decrypt(self, ciphertext: str | bytes, stringify: bool) -> bytes | str:
        if type(ciphertext) is str:
            ciphertext = Cryptor.unwrap(ciphertext)
        algo = ChaCha20Poly1305(self.key)
        nonce = ciphertext[:12]
        data_hash = ciphertext[12:76]
        document = ciphertext[76:]
        decrypted = algo.decrypt(nonce, document, data_hash)
        hasher = Hash(BLAKE2b(digest_size=64))
        hasher.update(decrypted)
        calculated_hash = hasher.finalize()
        if calculated_hash == data_hash:
            return decrypted.decode("utf-8") if stringify else decrypted
        else:
            raise ValueError("Invalid ciphertext")

    def keygen(raw: bool) -> bytes | str:
        key = ChaCha20Poly1305.generate_key()
        if raw:
            return key
        return Cryptor.wrap(key)

    def wrap(material: bytes) -> str:
        return b58encode_check(material).decode("utf-8")

    def unwrap(material: str) -> bytes:
        return b58decode_check(material)

    def warn(warned: bool) -> str:
        if not warned:
            echo(
                dedent(
                    """
                    WARNING: Make sure to back up your encryption key!
                    If you lose it, you will be unable to decrypt your data.

                    To silence this warning, set the 'crypt.warned' field
                    in your configuration file to 'true'.

                """
                ),
                err=True,
            )
