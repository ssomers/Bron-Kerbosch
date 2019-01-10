from stats import SampleStatistics
import csv
import math
import os
import sys
from typing import List

colors = [
    "#000099",
    "#CC6600",
    "#006600",
    "#0000FF",
    "#FF9900",
    "#FF0099",
    "#FF3333",
    "#00CC00",
    "#66FF66",
]


def publish(language: str, orderstr: str, func_names: List[str],
            sizes: List[int], stats_per_size: List[List[SampleStatistics]]):
    num_funcs = len(func_names)
    filename = f"bron_kerbosch_{language}_order_{orderstr}"
    path = os.path.join(os.pardir, filename + ".csv")
    with open(path, 'w', newline='') as csvfile:
        w = csv.writer(csvfile)
        w.writerow(["Size"] +
                   [f"{func_names[i]} min" for i in range(num_funcs)] +
                   [f"{func_names[i]} max" for i in range(num_funcs)] +
                   [f"{func_names[i]} mean" for i in range(num_funcs)])
        for i, size in enumerate(sizes):
            stats = stats_per_size[i]
            w.writerow([size] + [s.min for s in stats] +
                       [s.max for s in stats] + [s.mean() for s in stats])
    publish_csv(language=language, orderstr=orderstr)


def publish_csv(language: str, orderstr: str):
    filename = f"bron_kerbosch_{language}_order_{orderstr}"
    path = os.path.join(os.pardir, filename + ".csv")
    sizes = []
    min_per_size = []
    max_per_size = []
    mean_per_size = []
    with open(path, newline='') as csvfile:
        reader = csv.reader(csvfile)
        head = next(reader)
        num_funcs = (len(head) - 1) // 3
        assert len(head) == 1 + num_funcs * 3
        assert head[0] == "Size"
        for f in range(num_funcs):
            assert head[1 + num_funcs * 0 + f].endswith(" min")
            assert head[1 + num_funcs * 1 + f].endswith(" max")
            assert head[1 + num_funcs * 2 + f].endswith(" mean")
        func_names = [
            head[1 + num_funcs * 2 + f][:-5] for f in range(num_funcs)
        ]

        assert num_funcs <= len(colors)
        for row in reader:
            assert len(row) == 1 + num_funcs * 3
            size = int(row[0])
            sizes.append(size)
            min_per_size.append([
                float(cell)
                for cell in row[1 + num_funcs * 0:1 + num_funcs * 1]
            ])
            max_per_size.append([
                float(cell)
                for cell in row[1 + num_funcs * 1:1 + num_funcs * 2]
            ])
            mean_per_size.append([
                float(cell)
                for cell in row[1 + num_funcs * 2:1 + num_funcs * 3]
            ])

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
                    color=colors[f],
                ),
                line=dict(color=colors[f]),
                marker=dict(color=colors[f]),
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
    publish_csv(language=sys.argv[1], orderstr=sys.argv[2])
