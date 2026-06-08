from __future__ import annotations

import atexit
import time
from collections import defaultdict


_start_time = time.time()
_timings = defaultdict(lambda: {"calls": 0, "total": 0.0})


def record_timing(category: str, label: str, duration: float) -> None:
    key = (category, label)
    timing = _timings[key]
    timing["calls"] += 1
    timing["total"] += duration


def print_timing_report() -> None:
    if not _timings:
        return

    recorded_total = sum(timing["total"] for timing in _timings.values())
    wall_total = time.time() - _start_time
    category_totals = defaultdict(float)

    for (category, _), timing in _timings.items():
        category_totals[category] += timing["total"]

    print("[FLOW] Timing report:")
    print(
        f"[FLOW]   Recorded total: {recorded_total:.3f}s "
        f"({100 * recorded_total / wall_total:.1f}% of wall {wall_total:.3f}s)"
    )

    for category, total in sorted(
        category_totals.items(), key=lambda item: item[1], reverse=True
    ):
        percent = 100 * total / recorded_total if recorded_total else 0.0
        wall_percent = 100 * total / wall_total if wall_total else 0.0
        print(
            f"[FLOW]   {category}: {total:.3f}s "
            f"({percent:.1f}% recorded, {wall_percent:.1f}% wall)"
        )

        entries = [
            (label, timing)
            for (entry_category, label), timing in _timings.items()
            if entry_category == category
        ]
        for label, timing in sorted(
            entries, key=lambda item: item[1]["total"], reverse=True
        ):
            entry_percent = (
                100 * timing["total"] / recorded_total if recorded_total else 0.0
            )
            print(
                f"[FLOW]     {label}: calls={timing['calls']}, "
                f"total={timing['total']:.3f}s "
                f"({entry_percent:.1f}% recorded)"
            )


atexit.register(print_timing_report)
