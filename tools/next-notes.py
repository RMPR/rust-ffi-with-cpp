#!/usr/bin/env python3
"""Keep a "Next: ..." line in every slide's speaker note.

presenterm's notes window shows only the current slide's note, with no preview
of what comes next. This script writes that preview into the notes themselves:
each slide's note ends with the title of the following slide and a hint of
what is on it (code file, DEMO, diagram, table, bullets).

It is idempotent: run it again after editing slides.md and the "Next:" lines
are refreshed. Your own note text is kept. Nothing visible on the slides
changes; the script refuses to write if it would.

    tools/next-notes.py [slides.md]
"""
import re
import sys

path = sys.argv[1] if len(sys.argv) > 1 else "slides.md"
lines = open(path).read().split("\n")

# --- split into: preamble (front matter etc.) and titled slides -------------
in_fence = False
title_rows = []
for i, line in enumerate(lines):
    if line.startswith("```"):
        in_fence = not in_fence
    elif not in_fence and line == "===" and i > 0 and lines[i - 1].strip():
        title_rows.append(i - 1)
if not title_rows:
    sys.exit("no slides found")
bounds = title_rows + [len(lines)]
preamble = lines[: title_rows[0]]
slides = [lines[bounds[k] : bounds[k + 1]] for k in range(len(title_rows))]

ONE_LINE = re.compile(r"^\s*<!--\s*speaker_note:\s*(.*?)\s*-->\s*$")


def split_notes(body):
    """Return (body without speaker notes, list of note text lines)."""
    kept, notes, i, fence = [], [], 0, False
    while i < len(body):
        line = body[i]
        if line.startswith("```"):
            fence = not fence
        if not fence:
            m = ONE_LINE.match(line)
            if m:
                notes.append(m.group(1))
                i += 1
                continue
            if line.strip() == "<!--" and i + 1 < len(body) and body[i + 1].strip().startswith("speaker_note:"):
                j = i + 1
                first = body[j].strip()[len("speaker_note:"):].strip()
                if first and first != "|":
                    notes.append(first)
                j += 1
                while j < len(body) and body[j].strip() != "-->":
                    notes.append(body[j].strip())
                    j += 1
                i = j + 1
                continue
        kept.append(line)
        i += 1
    notes = [n for n in notes if n and not n.startswith("Next:")]
    while kept and not kept[-1].strip():
        kept.pop()
    return kept, notes


def describe(body):
    """A short hint of what a slide contains."""
    hints, bullets, fence, comment = [], [], False, False
    for k, line in enumerate(body):
        if not fence and line.strip() == "<!--":
            comment = True  # a commented-out block: not on the slide
        if comment:
            comment = line.strip() != "-->"
            continue
        if line.startswith("```"):
            if not fence:
                info = line[3:].strip()
                if info.startswith("file"):
                    for nxt in body[k + 1 : k + 4]:
                        if nxt.startswith("path:"):
                            hints.append("code: " + nxt.split("/")[-1].strip())
                elif "+exec" in info:
                    hints.append("DEMO, Ctrl+E")
                elif info:
                    hints.append("code")
            fence = not fence
            continue
        if fence:
            continue
        if line.startswith("!["):
            hints.append("diagram")
        elif line.startswith("|") and "table" not in hints:
            hints.append("table")
        elif re.match(r"^(\*|-|\d+\.) ", line):
            m = re.search(r"\*\*(.+?)\*\*", line)
            bullets.append(m.group(1) if m else None)
        elif "snippet_output" in line:
            hints.append("shows earlier output")
    if bullets:
        named = [b for b in bullets if b]
        hints.append("bullets: " + ", ".join(named) if len(named) == len(bullets) else f"{len(bullets)} bullets")
    seen, out = set(), []
    for h in hints:
        if h not in seen:
            seen.add(h)
            out.append(h)
    return " | ".join(out)


titles = [s[0].replace("`", "") for s in slides]
result = list(preamble)
visible_before, visible_after = [], []
for k, slide in enumerate(slides):
    body, notes = split_notes(slide)
    visible_before.append(body)
    if k + 1 < len(slides):
        hint = describe(split_notes(slides[k + 1])[0][2:])
        nxt = f"Next: {titles[k + 1]}" + (f"  [{hint}]" if hint else "")
    else:
        nxt = "Next: (end of talk)"
    block = ["", "<!--", "speaker_note: |"] + [f"  {n}" for n in notes + [nxt]] + ["-->", ""]
    result += body + block
    visible_after.append(split_notes(body + block)[0])

# --- safety: only speaker notes may differ ----------------------------------
if visible_before != visible_after:
    sys.exit("refusing to write: visible slide content would change")
text = "\n".join(result).rstrip("\n") + "\n"
if text != "\n".join(lines).rstrip("\n") + "\n":
    open(path, "w").write(text)
    print(f"{path}: refreshed 'Next:' notes on {len(slides)} slides")
else:
    print(f"{path}: 'Next:' notes already up to date")
