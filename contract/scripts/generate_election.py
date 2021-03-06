from datetime import datetime, timedelta
import json


def main():
    now = datetime.now()
    election = {
        "start": f'{nanoseconds(now + timedelta(minutes=1))}',
        "end": f'{nanoseconds(now + timedelta(3))}',
        "title": "My Election",
        "description": "Some short description",
        "candidates": ["Alice", "Bob"],
    }
    print(json.dumps(election))


def nanoseconds(t: datetime) -> int:
    return int(t.timestamp() * 1_000_000) * 1_000


if __name__ == '__main__':
    main()
