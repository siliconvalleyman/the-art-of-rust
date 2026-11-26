"""Validate the curriculum, publication, and Rust 1.75 workspace."""

from __future__ import annotations

import argparse
import json
import re
import shutil
import subprocess
import sys
import tempfile
import tomllib
from collections import Counter
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "src"
FENCE = re.compile(r"^ {0,3}(`{3,}|~{3,})(.*)$")
HEADING = re.compile(r"^ {0,3}(#{1,6})\s+(.+?)\s*#*\s*$")
INLINE_LINK = re.compile(r"!?\[[^\]\n]+\]\((<[^>]+>|[^\s)]+)(?:\s+\"[^\"]*\")?\)")
INCLUDE = re.compile(r"{{#include\s+([^\s}]+)\s*}}")
PART = re.compile(r"^## Part ([IVX]+) — (.+)$")
CHAPTER = re.compile(r"^\| (C\d{3}) \| ([^|]+) \| ([^|]+) \| ([^|]+) \|$")
PROJECT = re.compile(
    r"^\| (P\d{2}) \| ([^|]+) \| ([^|]+) \| ([^|]+) \| ([^|]+) \|$"
)
EXERCISE = re.compile(r"^### (EX-C\d{3}-\d{2}) — ", re.M)
QUESTION = re.compile(r"\*\*(IQ-C\d{3}-\d{2})(?=[,.]|\*\*)")
NAVIGATION = re.compile(r"^\s*-\s+\[(\d{3})\. ([^\]]+)\]\(([^)]+)\)\s*$", re.M)


def lines(path: Path) -> list[str]:
    return path.read_text(encoding="utf-8").splitlines()


def identifiers(cell: str) -> list[str]:
    return [] if cell == "—" else cell.split(",")


def curriculum(errors: list[str]) -> dict:
    parts = []
    chapters = {}
    projects = {}
    for line in lines(ROOT / "CURRICULUM.md"):
        if part_match := PART.fullmatch(line):
            number, title = part_match.groups()
            parts.append({"number": number, "title": title, "chapters": []})
        if chapter_match := CHAPTER.fullmatch(line):
            chapter, title, needs, evidence = chapter_match.groups()
            if not parts or chapter in chapters:
                errors.append(f"CURRICULUM.md: duplicate or ungrouped chapter {chapter}")
                continue
            prerequisites = identifiers(needs)
            for need in prerequisites:
                if need not in chapters:
                    errors.append(f"CURRICULUM.md: {chapter} needs earlier chapter {need}")
            chapters[chapter] = {
                "title": title,
                "prerequisites": prerequisites,
                "evidence": evidence,
                "part": len(parts),
            }
            parts[-1]["chapters"].append(chapter)
        if project_match := PROJECT.fullmatch(line):
            project, title, needs, earlier_projects, evidence = project_match.groups()
            if project in projects:
                errors.append(f"CURRICULUM.md: duplicate project {project}")
                continue
            prereq_chapters = identifiers(needs)
            prereq_projects = identifiers(earlier_projects)
            for need in prereq_chapters:
                if need not in chapters:
                    errors.append(f"CURRICULUM.md: {project} needs unknown chapter {need}")
            for need in prereq_projects:
                if need not in projects:
                    errors.append(f"CURRICULUM.md: {project} needs earlier project {need}")
            projects[project] = {
                "title": title,
                "chapters": prereq_chapters,
                "projects": prereq_projects,
                "evidence": evidence,
            }
    roman_parts = (
        "I II III IV V VI VII VIII IX X XI XII XIII XIV XV XVI XVII XVIII XIX XX "
        "XXI XXII XXIII XXIV XXV"
    ).split()
    if [part["number"] for part in parts] != roman_parts or any(
        not part["chapters"] for part in parts
    ):
        errors.append("CURRICULUM.md: expected 25 consecutive, nonempty parts")
    if len(chapters) < 100 or list(chapters) != [
        f"C{number:03}" for number in range(1, len(chapters) + 1)
    ]:
        errors.append("CURRICULUM.md: expected at least 100 consecutive chapter IDs")
    if list(projects) != [f"P{number:02}" for number in range(1, 26)]:
        errors.append("CURRICULUM.md: expected 25 consecutive project IDs")
    return {"parts": parts, "chapters": chapters, "projects": projects}


def markdown_blocks(path: Path, errors: list[str]):
    """Yield lines with fence context and report unclosed or empty headings."""
    fence = None
    for number, line in enumerate(lines(path), start=1):
        if marker := FENCE.match(line):
            delimiter, info = marker.groups()
            if fence is None:
                fence = (delimiter, info.strip())
                yield number, line, fence[1], True
                continue
            if delimiter[0] == fence[0][0] and len(delimiter) >= len(fence[0]) and not info.strip():
                fence = None
                continue
        if fence is None and re.fullmatch(r" {0,3}#{1,6}\s*", line):
            errors.append(f"{path.relative_to(ROOT)}:{number}: empty heading")
        yield number, line, fence[1] if fence else None, False
    if fence is not None:
        errors.append(f"{path.relative_to(ROOT)}: unclosed code fence")


def includes_from(path: Path, errors: list[str]) -> set[Path]:
    included = set()
    for line_number, line, _, _ in markdown_blocks(path, errors):
        for directive in INCLUDE.findall(line):
            filename, _, selector = directive.partition(":")
            target = (path.parent / filename).resolve()
            if not target.is_relative_to(ROOT) or not target.is_file():
                errors.append(f"{path.relative_to(ROOT)}:{line_number}: missing include {filename}")
                continue
            included.add(target)
            if selector and not re.fullmatch(r"\d*:\d*|\d+", selector):
                content = lines(target)
                start = f"ANCHOR: {selector}"
                stop = f"ANCHOR_END: {selector}"
                anchors = [i for i, row in enumerate(content) if row.strip().endswith(start)]
                ends = [i for i, row in enumerate(content) if row.strip().endswith(stop)]
                if len(anchors) != 1 or len(ends) != 1 or anchors[0] >= ends[0]:
                    errors.append(f"{path.relative_to(ROOT)}:{line_number}: invalid anchor {directive}")
    return included


def heading_ids(path: Path, includes: dict[Path, set[Path]], seen: set[Path]) -> set[str]:
    if path in seen:
        return set()
    seen = seen | {path}
    result = set()
    for _, line, fence, _ in markdown_blocks(path, []):
        if fence is None and (heading := HEADING.match(line)):
            text = re.sub(r"[^\w\s-]", "", heading[2].lower())
            result.add(re.sub(r"\s+", "-", text))
            result.add(text.replace(" ", "-"))
    for target in includes.get(path, set()):
        if target.suffix == ".md":
            result.update(heading_ids(target, includes, seen))
    return result


def inspect_markdown(paths: list[Path], errors: list[str]) -> dict[Path, set[Path]]:
    includes = {path: includes_from(path, errors) for path in paths}
    for path in paths:
        prose = ""
        for line_number, line, fence, opening in markdown_blocks(path, []):
            location = f"{path.relative_to(ROOT)}:{line_number}"
            if opening and fence is not None:
                tags = fence.split(",")
                if tags[0] == "rust" and "ignore" in tags and not any(
                    label in prose
                    for label in ("Illustrative fragment", "Platform-specific", "Toolchain-specific")
                ):
                    errors.append(f"{location}: ignored Rust fence needs an explicit classification")
            if fence is not None:
                continue
            if line.strip():
                prose = line
            for destination in INLINE_LINK.findall(re.sub(r"`+[^`]*`+", "", line)):
                target = urlsplit(destination.strip("<>"))
                if target.scheme or target.netloc:
                    continue
                linked = (path.parent / unquote(target.path)).resolve() if target.path else path
                if not linked.is_relative_to(ROOT) or not linked.exists():
                    errors.append(f"{location}: missing link {destination}")
                elif target.fragment and linked.suffix == ".md" and unquote(
                    target.fragment
                ) not in heading_ids(linked, includes, set()):
                    errors.append(f"{location}: missing anchor {destination}")
    return includes


def check_publication(manifest: dict, includes: dict[Path, set[Path]], errors: list[str]) -> dict:
    summary = (SOURCE / "SUMMARY.md").read_text(encoding="utf-8")
    nav = NAVIGATION.findall(summary)
    numbers = [int(number) for number, _, _ in nav]
    if numbers != sorted(set(numbers)):
        errors.append("src/SUMMARY.md: numbered chapters must be unique and in sequence order")
    on_disk = set(SOURCE.glob("part-*/[0-9][0-9][0-9]-*.md"))
    in_summary = {(SOURCE / target).resolve() for _, _, target in nav}
    if on_disk != in_summary:
        errors.append("src/SUMMARY.md: numbered chapter paths differ from the files on disk")
    published = {f"C{number}" for number, _, _ in nav}
    exercises_seen = Counter()
    questions_seen = Counter()
    published_solutions = set().union(
        *(
            targets
            for path, targets in includes.items()
            if path.is_relative_to(SOURCE) and path != SOURCE / "SUMMARY.md"
        )
    )
    for number, title, target in nav:
        chapter = f"C{number}"
        chapter_plan = manifest["chapters"].get(chapter)
        path = (SOURCE / target).resolve()
        if chapter_plan is None:
            errors.append(f"src/SUMMARY.md: unknown chapter {chapter}")
            continue
        if set(chapter_plan["prerequisites"]) - published:
            errors.append(f"{chapter}: prerequisites missing from book")
        expected_dir = SOURCE / f"part-{chapter_plan['part']:02}"
        if path.parent != expected_dir or title != chapter_plan["title"]:
            errors.append(f"{chapter}: navigation must match the curriculum title and part")
        if not path.is_file() or lines(path)[0:1] != [f"# {number}. {title}"]:
            errors.append(f"{chapter}: expected chapter heading with matching ID and title")
        statement = ROOT / "exercises" / f"{chapter}.md"
        answer = ROOT / "solutions" / f"{chapter}.md"
        if not statement.is_file() or not answer.is_file():
            errors.append(f"{chapter}: missing exercise or solution file")
            continue
        if statement not in includes.get(path, set()) or answer not in published_solutions:
            errors.append(f"{chapter}: exercise or solution file missing from rendered chapters")
        for pattern, seen, prefix, minimum in (
            (EXERCISE, exercises_seen, "EX", 5),
            (QUESTION, questions_seen, "IQ", 2),
        ):
            ids = pattern.findall(statement.read_text(encoding="utf-8"))
            matched = pattern.findall(answer.read_text(encoding="utf-8"))
            if len(ids) < minimum or ids != matched or any(
                key != f"{prefix}-{chapter}-{index:02}"
                for index, key in enumerate(ids, start=1)
            ):
                errors.append(f"{chapter}: invalid {prefix} IDs or missing matching answers")
            seen.update(ids)
    for stem in ("exercises", "solutions"):
        for path in (ROOT / stem).glob("C*.md"):
            if path.stem not in published:
                errors.append(f"{path.relative_to(ROOT)}: no matching chapter")
    return {"chapters": len(published), "exercises": len(exercises_seen), "questions": len(questions_seen)}


class RenderedPage(HTMLParser):
    def __init__(self):
        super().__init__()
        self.in_main = False
        self.anchors = set()
        self.references = []

    def handle_starttag(self, tag, attributes):
        attrs = dict(attributes)
        if tag == "main":
            self.in_main = True
        if attrs.get("id"):
            self.anchors.add(attrs["id"])
        if self.in_main and tag in ("a", "img"):
            reference = attrs.get("href" if tag == "a" else "src")
            if reference:
                self.references.append(reference)

    def handle_endtag(self, tag):
        if tag == "main":
            self.in_main = False


def check_rendered() -> list[str]:
    book = ROOT / "book"
    pages = {}
    for path in book.rglob("*.html"):
        parser = RenderedPage()
        parser.feed(path.read_text(encoding="utf-8"))
        pages[path] = parser
    errors = []
    for path, page in pages.items():
        for reference in page.references:
            target = urlsplit(reference)
            if target.scheme or target.netloc:
                continue
            linked = (path.parent / unquote(target.path)).resolve() if target.path else path
            if linked.is_dir():
                linked /= "index.html"
            if not linked.is_relative_to(book) or not linked.is_file():
                errors.append(f"{path.relative_to(ROOT)}: missing rendered target {reference}")
            elif linked in pages and target.fragment and unquote(
                target.fragment
            ) not in pages[linked].anchors:
                errors.append(f"{path.relative_to(ROOT)}: missing rendered anchor {reference}")
    return errors


def run(*command: str) -> bool:
    print("+", " ".join(command), flush=True)
    return subprocess.run(command, cwd=ROOT).returncode == 0


def check_byte_diagnostic() -> bool:
    fixture = "tests/fixtures/out_of_range_byte.rs"
    print(f"+ rustc diagnostic check for {fixture}", flush=True)
    with tempfile.TemporaryDirectory() as scratch:
        result = subprocess.run(
            ("rustc", "--edition=2021", "--emit=metadata", "--error-format=json",
             "--out-dir", scratch, fixture),
            capture_output=True, text=True, encoding="utf-8", cwd=ROOT,
        )
    diagnostics = [
        json.loads(line) for line in result.stderr.splitlines() if line.startswith("{")
    ]
    return result.returncode != 0 and any(
        diagnostic.get("level") == "error"
        and (diagnostic.get("code") or {}).get("code") == "overflowing_literals"
        for diagnostic in diagnostics
    )


def validate_tools() -> list[str]:
    versions = {"rustc": "1.75.0", "mdbook": "v0.4.36"}
    errors = []
    for name, version in versions.items():
        if not shutil.which(name):
            errors.append(f"{name} is unavailable. See README.md for the historical toolchain.")
            continue
        actual = subprocess.run(
            (name, "--version"), capture_output=True, text=True, cwd=ROOT
        )
        tokens = actual.stdout.split()
        if actual.returncode or tokens[:2] != [name, version]:
            errors.append(f"expected {name} {version}, got {actual.stdout.strip() or actual.stderr.strip()}")
    for name in ("cargo", "rustdoc"):
        if not shutil.which(name):
            errors.append(f"{name} is unavailable.")
    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--docs-only", action="store_true")
    mode.add_argument("--manifest", action="store_true")
    args = parser.parse_args()

    errors = []
    manifest = curriculum(errors)
    if args.manifest and not errors:
        print(json.dumps(manifest, indent=2, ensure_ascii=True))
        return 0
    paths = sorted(
        set(ROOT.glob("*.md"))
        | {
            path
            for folder in (
                ".github", "src", "examples", "exercises", "solutions", "proofs",
                "mathematics", "algorithms", "data-structures", "projects", "benchmarks",
                "appendices", "assets", "resources",
            )
            for path in (ROOT / folder).rglob("*.md")
        }
    )
    includes = inspect_markdown(paths, errors)
    counts = check_publication(manifest, includes, errors)
    with (ROOT / "book.toml").open("rb") as book_config:
        book = tomllib.load(book_config)
    if book["book"]["authors"] != ["Zavqiddin Hasanov"] or book["rust"]["edition"] != "2021":
        errors.append("book.toml: incorrect author or Rust edition")
    if not args.docs_only and not errors:
        errors.extend(validate_tools())
        if not errors:
            commands = [
                ("cargo", "fmt", "--all", "--", "--check"),
                ("cargo", "check", "--workspace", "--all-targets", "--locked"),
                ("cargo", "clippy", "--workspace", "--all-targets", "--locked", "--", "-D", "warnings"),
                ("cargo", "test", "--workspace", "--all-targets", "--locked"),
                ("cargo", "test", "--workspace", "--doc", "--locked"),
                ("cargo", "build", "--workspace", "--release", "--locked"),
                ("mdbook", "build"),
                ("mdbook", "test"),
            ]
            for command in commands:
                if not run(*command):
                    errors.append(f"command failed: {' '.join(command)}")
                elif command == ("mdbook", "build"):
                    errors.extend(check_rendered())
            if not check_byte_diagnostic():
                errors.append("rustc did not emit overflowing_literals for the out-of-range byte")
    for error in errors:
        print(error, file=sys.stderr)
    if errors:
        return 1
    print(
        f"Validated {counts['chapters']} published chapters, "
        f"{counts['exercises']} exercise pairs, and {counts['questions']} interview pairs."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
