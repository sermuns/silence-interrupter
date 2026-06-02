# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "aiohttp>=3.14.0",
#     "asyncio>=4.0.0",
# ]
# ///
import asyncio
import aiohttp
from pathlib import Path

OUT_DIR = Path("audio")


async def download(session, url):
    file_name = Path(url).name
    file_path = OUT_DIR / file_name

    if file_path.exists():
        print(f"'{file_name}' already exists, skipping")
        return

    print(f"Downloading '{file_name}'")

    async with session.get(url) as resp:
        resp.raise_for_status()
        (OUT_DIR / file_name).write_bytes(await resp.read())


async def main():
    OUT_DIR.mkdir(exist_ok=True)

    urls = [f"https://minecraft.wiki/images/Cave{i}.ogg" for i in range(1, 20)] + [
        f"https://minecraft.wiki/images/Villager_idle{i}.ogg" for i in range(1, 4)
    ]

    async with aiohttp.ClientSession() as session:
        await asyncio.gather(*(download(session, url) for url in urls))


asyncio.run(main())
