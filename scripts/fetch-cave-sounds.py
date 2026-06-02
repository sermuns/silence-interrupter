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
    filename = Path(url).name

    async with session.get(url) as resp:
        resp.raise_for_status()
        (OUT_DIR / filename).write_bytes(await resp.read())


async def main():
    OUT_DIR.mkdir(exist_ok=True)

    urls = [f"https://minecraft.wiki/images/Cave{i}.ogg" for i in range(1, 20)]

    async with aiohttp.ClientSession() as session:
        await asyncio.gather(*(download(session, url) for url in urls))


asyncio.run(main())
