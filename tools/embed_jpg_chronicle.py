#!/usr/bin/env python3
"""Make a self-contained story HTML embedding the given JPEG byte-for-byte.

Use for each CERN H0xx iteration:
  python3 tools/embed_jpg_chronicle.py docs/images/h034-story.jpg \
    docs/h034-story.html --title "H034 — Simultaneous Ring Disagreements"

No external libraries or network requests. The JPG is represented as a
data:image/jpeg;base64 URI so one HTML file preserves the visual offline.
"""
import argparse
import base64
import hashlib
import html
import pathlib


def make_page(jpeg_bytes, title, caption, repo_url):
    if not jpeg_bytes.startswith(b"\xff\xd8\xff") or not jpeg_bytes.rstrip().endswith(b"\xff\xd9"):
        raise ValueError("Input must be a complete JPEG image")
    b64 = base64.b64encode(jpeg_bytes).decode("ascii")
    safe_title = html.escape(title, quote=True)
    safe_caption = html.escape(caption, quote=True)
    safe_url = html.escape(repo_url, quote=True)
    sha = hashlib.sha256(jpeg_bytes).hexdigest()
    return f"""<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>{safe_title}</title>
<style>
:root{{color-scheme:dark}}*{{box-sizing:border-box}}body{{margin:0;background:#071423;color:#e6f6ff;font:16px/1.5 system-ui}}
main{{max-width:1400px;margin:auto;padding:24px}}h1{{font-size:clamp(1.7rem,3.4vw,2.6rem);margin:.2em 0}}
img{{width:100%;height:auto;display:block;border-radius:14px;border:1px solid #2b556a;box-shadow:0 10px 42px #0008;margin:16px 0}}
a{{color:#8fe0ec}}small{{color:#adc9d8}}
</style></head><body><main>
<small>DAVIDWISE01 / CERN · VISUAL RESEARCH CHRONICLE</small>
<h1>{safe_title}</h1>
<p>{safe_caption}</p>
<img alt="{safe_caption}" src="data:image/jpeg;base64,{b64}">
<p><small>Original JPEG SHA-256: <code>{sha}</code> · Embedded without modification.</small></p>
<p><a href="{safe_url}">Open project repository</a></p>
</main></body></html>
"""


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("jpg", type=pathlib.Path)
    p.add_argument("html", type=pathlib.Path)
    p.add_argument("--title", required=True)
    p.add_argument("--caption", default="Research visualization: conceptual model, not CERN detector telemetry.")
    p.add_argument("--repo-url", default="https://github.com/DavidWise01/cern")
    args = p.parse_args()
    image = args.jpg.read_bytes()
    page = make_page(image, args.title, args.caption, args.repo_url)
    args.html.parent.mkdir(parents=True, exist_ok=True)
    args.html.write_text(page, encoding="utf-8")
    print(f"Embedded JPEG {len(image):,} bytes into {args.html}; HTML {len(page):,} characters")


if __name__ == "__main__":
    main()
