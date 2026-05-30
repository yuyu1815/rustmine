#!/usr/bin/env python3
import argparse
import json
import os
import sys
import urllib.request
from pathlib import Path


def is_native_library(name: str) -> bool:
    return ":natives-" in name


def download(url: str, out: Path) -> None:
    out.parent.mkdir(parents=True, exist_ok=True)
    if out.exists():
        return
    tmp = out.with_suffix(out.suffix + ".tmp")
    with urllib.request.urlopen(url) as response, tmp.open("wb") as handle:
        handle.write(response.read())
    tmp.replace(out)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--version-json", required=True)
    parser.add_argument("--client-jar", required=True)
    parser.add_argument("--cache-dir", required=True)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    version = json.loads(Path(args.version_json).read_text())
    entries = [str(Path(args.client_jar))]

    cache_dir = Path(args.cache_dir)
    for lib in version.get("libraries", []):
        name = lib.get("name", "")
        if is_native_library(name):
            continue
        artifact = lib.get("downloads", {}).get("artifact")
        if not artifact:
            continue
        path = cache_dir / artifact["path"]
        download(artifact["url"], path)
        entries.append(str(path))

    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(os.pathsep.join(entries))
    print(out)
    return 0


if __name__ == "__main__":
    sys.exit(main())
