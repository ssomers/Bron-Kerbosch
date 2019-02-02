from stats import SampleStatistics
import csv
import math
import os
import sys
from typing import List

COLORS = {
    "Ver1": "#000099",
    "Ver1+": "#0000FF",
    "Ver2": "#CC6600",
    "Ver2+": "#FF6600",
    "Ver2_RP": "#FF9900",
    "Ver2_GP": "#FF0099",
    "Ver2_GPX": "#FF3333",
    "Ver3": "#006600",
    "Ver3+": "#00CC00",
    "Ver3-": "#66CC66",
    "Ver3+MT": "#66FF66",
}


def publish(language: str, orderstr: str, func_names: List[str],
            sizes: List[int], stats_per_size: List[List[SampleStatistics]]):
    num_funcs = len(func_names)
    filename = f"bron_kerbosch_{language}_order_{orderstr}"
    path = os.path.join(os.pardir, filename + ".csv")
    with open(path, 'w', newline='') as csvfile:
        w = csv.writer(csvfile)
        w.writerow(["Size"] + [(name + " " + t) for name in func_names
                               for t in ["min", "mean", "max"]])
        for i, size in enumerate(sizes):
            stats = stats_per_size[i]
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
        num_funcs = (len(head) - 1) // 3
        if len(head) != 1 + num_funcs * 3:
            raise ImportError(f"Head: Found {len(row)} columns")
        if head[0] != "Size":
            raise ImportError("unexpected " + str(head[0]))
        if not all(h.endswith(" min") for h in head[1::3]):
            raise ImportError("unexpected " + str(head[1::3]))
        if not all(h.endswith(" mean") for h in head[2::3]):
            raise ImportError("unexpected " + str(head[2::3]))
        if not all(h.endswith(" max") for h in head[3::3]):
            raise ImportError("unexpected " + str(head[3::3]))
        func_names = [h.split()[0] for h in head[2::3]]

        assert all(func_names[f] in COLORS
                   for f in range(num_funcs)), f"Unknown in {func_names}"
        for i, row in enumerate(reader):
            if len(row) != 1 + num_funcs * 3:
                raise ImportError(f"Row {i+2}: Found {len(row)} columns")
            size = int(row[0])
            sizes.append(size)
            min_per_size.append([float(cell) for cell in row[1::3]])
            mean_per_size.append([float(cell) for cell in row[2::3]])
            max_per_size.append([float(cell) for cell in row[3::3]])

    try:
        from plotly import graph_objs, plotly
    except ImportError as e:
        print(f"{e}, not plotting until you pip install plotly")
    else:
        traces = [
            graph_objs.Scatter(
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
                    color=COLORS[func_names[f]],
                ),
                line=dict(color=COLORS[func_names[f]]),
                marker=dict(color=COLORS[func_names[f]]),
                mode="lines+markers",
                name=func_names[f],
            ) for f in range(num_funcs) if any(
                not math.isnan(mean_per_size[s][f]) for s in range(len(sizes)))
        ]
        layout = dict(
            title=('<a href="https://github.com/ssomers/Bron-Kerbosch">' +
                   f"{language.capitalize()} implementations of Bron-Kerbosch"
                   + "</a>" + f" on random graphs of order {orderstr}"),
            xaxis=dict(title="Size (#edges)"),
            yaxis=dict(title="Seconds spent"),
        )
        plotly.plot(
            figure_or_data=dict(
                data=traces,
                layout=layout,
            ),
            filename=filename)


if __name__ == '__main__':
    for orderstr in sys.argv[2:]:
        publish_csv(language=sys.argv[1], orderstr=orderstr)
