import json
import os
import re
import sys
from enum import IntFlag, auto

VERSION_PATTERN = re.compile(r"version: (\S+)")
RUN_ID_PATTERN = re.compile(r"run_id: (\S+)")


class Errors(IntFlag):
    MISSING_VERSION = auto()
    MISSING_RUN_ID = auto()


def main():
    event_path = os.environ["GITHUB_EVENT_PATH"]
    errors = 0

    with open(event_path, "r") as fobj:
        event_data = json.load(fobj)
    first_comment = event_data["issue"]["body"]
    version_match = VERSION_PATTERN.search(first_comment)
    if not version_match:
        print("::error ::Could not find version in first comment!")
        errors = Errors.MISSING_VERSION
    run_id_match = RUN_ID_PATTERN.search(first_comment)
    if not run_id_match:
        print("::error ::Could not find run_id in first comment!")
        errors = errors | Errors.MISSING_RUN_ID
    if errors:
        return errors
    with open(os.getenv("GITHUB_OUTPUT"), "a") as fobj:
        print(f"version={version_match.group(1)}", file=fobj)
        print(f"run_id={run_id_match.group(1)}", file=fobj)


if __name__ == '__main__':
    sys.exit(main())
