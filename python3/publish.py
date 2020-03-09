from stats import SampleStatistics
import csv
import math
import os
import sys
from typing import List, Mapping


def func_name(case_name: str) -> str:
    return case_name.split('@')[0]


def color(case_name: str) -> str:
    return {
        "Ver1": "#000099",
        "Ver1+": "#3333CC",
        "Ver2+": "#990000",
        "Ver2+G": "#FF3300",
        "Ver2+GP": "#FF6666",
        "Ver2+GPX": "#FF9966",
        "Ver2+RP": "#CC00CC",
        "Ver3+": "#006600",
        "Ver3+GP": "#339900",
        "Ver3+GPX": "#33CC00",
        "Ver3+MT": "#669999",
        "Ver3+ST": "#669966",
        "Ver3=GP2": "#669999",
        "Ver3=GP3": "#669966",
        "Ver3=GP4": "#669933",
        "Ver3=GP5": "#669900",
    }[func_name(case_name)]


def dash(case_name: str) -> str:
    if case_name.endswith("@BTreeSet") or case_name.endswith("@std_set"):
        return "dot"
    if case_name.endswith("@ord_vec"):
        return "dash"
    if case_name.endswith("@fnv"):
        return "dashdot"
    if case_name.endswith("@hashbrown"):
        return "longdash"
    return "solid"


def publish(language: str, orderstr: str, case_names: List[str],
            stats_per_func_by_size: Mapping[int, List[SampleStatistics]]):
    num_cases = len(case_names)
    filename = f"bron_kerbosch_{language}_order_{orderstr}"
    path = os.path.join(os.pardir, filename + ".csv")
    with open(path, 'w', newline='') as csvfile:
        w = csv.writer(csvfile)
        w.writerow(["Size"] + [(name + " " + t) for name in case_names
                               for t in ["min", "mean", "max"]])
        for size, stats in stats_per_func_by_size.items():
            w.writerow([size] +
                       [f for s in stats
                        for f in [s.min, s.mean(), s.max]])
    publish_csv(language=language, orderstr=orderstr)


def publish_csv(language: str, orderstr: str):
    filename = f"bron_kerbosch_{language}_order_{orderstr}"
    path = filename + ".csv"
    if not os.path.exists(path):
        path = os.path.join(os.pardir, path)
    sizes = []
    min_per_size = []
    max_per_size = []
    mean_per_size = []
    with open(path, newline='') as csvfile:
        reader = csv.reader(csvfile)
        head = next(reader)
        num_cases = (len(head) - 1) // 3
        if len(head) != 1 + num_cases * 3:
            raise ImportError(f"Head: Found {len(head)} columns")
        if head[0] != "Size":
            raise ImportError("unexpected " + str(head[0]))
        if not all(h.endswith(" min") for h in head[1::3]):
            raise ImportError("unexpected " + str(head[1::3]))
        if not all(h.endswith(" mean") for h in head[2::3]):
            raise ImportError("unexpected " + str(head[2::3]))
        if not all(h.endswith(" max") for h in head[3::3]):
            raise ImportError("unexpected " + str(head[3::3]))
        case_names = [h.split()[0] for h in head[2::3]]

        assert all(color(case_names[f])
                   for f in range(num_cases)), f"Unknown in {case_names}"
        for i, row in enumerate(reader):
            expected_cols = 1 + num_cases * 3
            if len(row) != expected_cols:
                raise ImportError(
                    f"Row {i+2}: Found {len(row)} columns, expected {expected_cols}"
                )
            size = int(row[0])
            sizes.append(size)
            min_per_size.append([float(cell) for cell in row[1::3]])
            mean_per_size.append([float(cell) for cell in row[2::3]])
            max_per_size.append([float(cell) for cell in row[3::3]])

    try:
        from chart_studio import plotly
    except ImportError as e:
        print(f"{e} (maybe you want to pip install chart-studio?)")
    else:
        indices = [
            f for f in range(num_cases) if any(
                not math.isnan(mean_per_size[s][f]) for s in range(len(sizes)))
        ]
        # Group traces in legend, unless every func_name is either unique or the same
        unique_func_names = len({func_name(case_names[f]) for f in indices})
        legendgroups = len(indices) > 5 and unique_func_names in range(
            2, len(indices))

        import plotly.io as pio
        pio.templates.default = "none"  # disable default 4.0 theme
        from plotly import graph_objects
        traces = [
            graph_objects.Scatter(
                x=sizes,
                y=[mean_per_size[s][f] for s in range(len(sizes))],
                error_y=dict(
                    type="data",
                    array=[
                        max_per_size[s][f] - mean_per_size[s][f]
                        for s in range(len(sizes))
                    ],
                    arrayminus=[
                        mean_per_size[s][f] - min_per_size[s][f]
                        for s in range(len(sizes))
                    ],
                ),
                hoverinfo="name",
                line=dict(color=color(case_names[f]),
                          dash=dash(case_names[f])),
                marker=dict(color=color(case_names[f])),
                mode="lines+markers",
                name=case_names[f],
                legendgroup=func_name(case_names[f]) if legendgroups else None,
            ) for f in indices
        ]
        layout = dict(
            title=(
                '<a href="https://github.com/ssomers/Bron-Kerbosch">' +
                f"{language.capitalize()} implementations of Bron-Kerbosch" +
                "</a>" + f" on random graphs of order {orderstr}"),
            xaxis=dict(title="Size (#edges)"),
            yaxis=dict(title="Seconds spent"),
        )
        plotly.plot(figure_or_data=dict(
            data=traces,
            layout=layout,
        ),
                    filename=filename)


if __name__ == '__main__':
    for orderstr in sys.argv[2:]:
        publish_csv(language=sys.argv[1], orderstr=orderstr)
