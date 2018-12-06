from stats import SampleStatistics
import csv
import os
import sys
from typing import List


def publish(language: str, orderstr: str, num_funcs: int, sizes: List[int],
            stats_per_size: List[List[SampleStatistics]]):
    assert all([s.is_populated() for stats in stats_per_size for s in stats])
    filename = f"bron_kerbosch_{language}_order_{orderstr}"
    path = os.path.join(os.pardir, filename + ".csv")
    with open(path, 'w', newline='') as csvfile:
        w = csv.writer(csvfile)
        w.writerow(["Size"] + [f"Ver{i+1} seconds" for i in range(num_funcs)] +
                   [f"Ver{i+1} error" for i in range(num_funcs)])
        for i, size in enumerate(sizes):
            stats = stats_per_size[i]
            w.writerow([size] + [s.mean() for s in stats] +
                       [s.deviation() for s in stats])
    publish_csv(language=language, orderstr=orderstr)


def publish_csv(language: str, orderstr: str):
    filename = f"bron_kerbosch_{language}_order_{orderstr}"
    path = os.path.join(os.pardir, filename + ".csv")
    with open(path, newline='') as csvfile:
        reader = csv.reader(csvfile)
        head = next(reader)
        assert head[0] == "Size"
        num_funcs = (len(head) - 1) // 2
        assert len(head) == 1 + num_funcs * 2
        sizes = []
        times_per_size = []
        error_per_size = []
        for row in reader:
            assert len(row) == 1 + num_funcs * 2
            size = int(row[0])
            sizes.append(size)
            times_per_size.append(
                [float(cell) for cell in row[1:1 + num_funcs]])
            error_per_size.append(
                [float(cell) for cell in row[1 + num_funcs:]])

    try:
        from plotly import graph_objs, plotly
    except ImportError as e:
        print(f"{e}, not plotting until you pip install plotly")
    else:
        traces = [
            graph_objs.Scatter(
                x=sizes,
                y=[times_per_size[s][f] for s in range(len(sizes))],
                error_y={
                    'type': 'data',
                    'array': [error_per_size[s][f] for s in range(len(sizes))],
                    'visible': True
                },
                mode='lines+markers',
                name=f"Ver{f+1}") for f in range(num_funcs)
        ]
        layout = {
            'title': (f"{language.capitalize()} implementations" +
                      f" of Bron-Kerbosch on random graphs order {orderstr}"),
            'xaxis': {
                'title': "Size (#edges)"
            },
            'yaxis': {
                'title': "Seconds spent"
            },
        }
        plotly.plot(
            figure_or_data={
                'data': traces,
                'layout': layout,
            },
            filename=filename)


if __name__ == '__main__':
    publish_csv(language=sys.argv[1], orderstr=sys.argv[2])
