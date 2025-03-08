import os
from pathlib import Path

import click
from mutagen.flac import FLAC
from mutagen.mp3 import MP3
from mutagen.mp4 import MP4

decoders = {".flac": FLAC, ".mp3": MP3, ".m4a": MP4}


def audio_info(path: click.Path) -> None:
    """
    This function walks through a directory and its subdirectories,
    identifies audio files, and prints their file paths, sample rates,
    bits per sample, and file sizes.

    Parameters:
    path (click.Path): The root directory to start the search.

    Returns:
    None: The function does not return any value. It prints the audio file information.
    """
    for dirpath, _, filenames in os.walk(path):
        for filename in filenames:
            file_path = Path(os.path.join(dirpath, filename))
            ext = file_path.suffix.lower()
            if ext not in decoders:
                pass
            else:
                size = file_path.stat().st_size
                decoder = decoders.get(ext)
                if decoder is None:
                    pass
                else:
                    audio = decoder(file_path)
                    if ext == ".mp3":
                        click.echo(f"{file_path}:{audio.info.sample_rate}:16:{size}")
                    else:
                        click.echo(
                            f"{file_path}:{audio.info.sample_rate}:{audio.info.bits_per_sample}:{size}"
                        )
