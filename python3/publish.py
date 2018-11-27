import sys
from typing import List


def publish(language: str, num_funcs: int, order: int, sizes: List[int],
            times_per_size: List[List[float]]):
    try:
        from plotly import graph_objs, plotly
    except ImportError as e:
        print(f"{e}, not plotting until you pip install plotly")
    else:
        traces = [
            graph_objs.Scatter(
                x=sizes,
                y=[times_per_size[s][f] for s in range(len(sizes))],
                mode='lines+markers',
                name=f"Ver{f+1}") for f in range(num_funcs)
        ]
        layout = {
            'title': ("{language} implementations of Bron-Kerbosch on " +
                      f"random graphs order (#nodes) {order}"),
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
            filename=f'Bron-Kerbosch_order_{order}')


if __name__ == '__main__':
    a = 1
    language = sys.argv[a]
    a += 1
    num_funcs = int(sys.argv[a])
    a += 1
    order = int(sys.argv[a])
    a += 1
    num_sizes = int(sys.argv[a])
    a += 1
    sizes = [int(sys.argv[a + i]) for i in range(num_sizes)]
    a += num_sizes
    times_per_size = [[
        float(sys.argv[a + j * num_funcs + i]) for i in range(num_funcs)
    ] for j in range(num_sizes)]
    publish(
        language=language,
        num_funcs=num_funcs,
        order=order,
        sizes=sizes,
        times_per_size=times_per_size)
