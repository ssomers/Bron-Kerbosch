from stats import SampleStatistics
import csv
import math
import os
import sys
from typing import Callable, Dict, List, Mapping, Optional


def lang_name(case_name: str) -> str:
    return case_name.split(' ')[0]


def func_name(case_name: str) -> str:
    return case_name.split(' ')[-1].split('@')[0]


def color_by_func_name(case_name: str) -> str:
    return {
        "Ver0": "#000099",
        "Ver1": "#3333CC",
        "Ver2": "#990000",
        "Ver2-G": "#FF3300",
        "Ver2-GP": "#FF6666",
        "Ver2-GPX": "#FF9966",
        "Ver2-RP": "#CC00CC",
        "Ver3": "#006600",
        "Ver3-GP": "#339900",
        "Ver3-GPX": "#33CC00",
        "Ver3=GPc": "#669999",
        "Ver3=GPs": "#669966",
        "Ver3=GP0": "#3399CC",
        "Ver3=GP1": "#669999",
        "Ver3=GP2": "#669966",
        "Ver3=GP3": "#669933",
        "Ver3=GP4": "#669900",
    }[func_name(case_name)]


def color_by_language(case_name: str) -> str:
    return {
        "Python3": "#000099",
        "Rust": "#CC0033",
        "Java": "#009933",
        "Scala": "#006666",
        "C#": "#666600",
        "C++": "#990000",
        "Go": "#0000CC",
    }[lang_name(case_name)]


def dash_by_lib(case_name: str) -> str:
    if case_name.endswith("@BTreeSet") or case_name.endswith("@std_set"):
        return "dot"
    if case_name.endswith("@ord_vec"):
        return "dash"
    if case_name.endswith("@fnv"):
        return "dashdot"
    if case_name.endswith("@hashbrown"):
        return "longdash"
    return "solid"


class Measurement(object):
    def __init__(self, min, mean, max):
        self.min = min
        self.mean = mean
        self.max = max

    def isnan(self):
        return math.isnan(self.mean)

    def error_plus(self):
        return self.max - self.mean

    def error_minus(self):
        return self.mean - self.min


def publish(language: str, orderstr: str, case_names: List[str],
            stats_per_func_by_size: Mapping[int, List[SampleStatistics]]):
    num_cases = len(case_names)
    filename = f"bron_kerbosch_{language}_order_{orderstr}"
    path = os.path.join(os.pardir, filename + ".csv")
    with open(path, 'w', newline='') as csvfile:
        w = csv.writer(csvfile)
        w.writerow(["Size"] + [(f"{name} {t}") for name in case_names
                               for t in ["min", "mean", "max"]])
        for size, stats in stats_per_func_by_size.items():
            w.writerow([size] +
                       [f for s in stats
                        for f in [s.min, s.mean(), s.max]])
    publish_whole_csv(language=language, orderstr=orderstr)


def read_csv(language: str,
             orderstr: str,
             case_name_selector: Mapping[str, str] = {}):
    filename = f"bron_kerbosch_{language}_order_{orderstr}"
    path = filename + ".csv"
    if not os.path.exists(path):
        path = os.path.join(os.pardir, path)
    sizes = []
    measurement_per_size_by_case_name: Dict[str, List[Measurement]] = {}
    with open(path, newline='') as csvfile:
        reader = csv.reader(csvfile)
        head = next(reader)
        num_cases = (len(head) - 1) // 3
        expected_cols = 1 + num_cases * 3
        if len(head) != expected_cols:
            raise ImportError(
                f"{filename}: Found {len(head)} columns, expected {expected_cols}"
            )
        if head[0] != "Size":
            raise ImportError("unexpected " + str(head[0]))
        if not all(h.endswith(" min") for h in head[1::3]):
            raise ImportError("unexpected " + str(head[1::3]))
        if not all(h.endswith(" mean") for h in head[2::3]):
            raise ImportError("unexpected " + str(head[2::3]))
        if not all(h.endswith(" max") for h in head[3::3]):
            raise ImportError("unexpected " + str(head[3::3]))
        case_names = [h.split()[0] for h in head[2::3]]
        for case_name in case_names:
            try:
                color_by_func_name(case_name)
            except KeyError as e:
                raise ImportError(
                    f'{filename}: unrecognized case name "{case_name}"') from e
        for i, row in enumerate(reader):
            if len(row) != expected_cols:
                raise ImportError(
                    f"{filename} row {i+2}: Found {len(row)} columns, expected {expected_cols}"
                )
            size = int(row[0])
            sizes.append(size)
            for c, case_name in enumerate(case_names):
                published_name = (case_name if case_name_selector is None else
                                  case_name_selector.get(case_name))
                if published_name is not None:
                    m = Measurement(min=float(row[c * 3 + 1]),
                                    mean=float(row[c * 3 + 2]),
                                    max=float(row[c * 3 + 3]))
                    measurement_per_size_by_case_name.setdefault(
                        published_name, []).append(m)

    for case_name in list(measurement_per_size_by_case_name.keys()):
        if all(measurement_per_size_by_case_name[case_name][s].isnan()
               for s in range(len(sizes))):
            print(f"{filename}: backing out on {case_name}")
            del measurement_per_size_by_case_name[case_name]
    return filename, sizes, measurement_per_size_by_case_name


def publish_whole_csv(language: str, orderstr: str):
    filename, sizes, measurements = read_csv(language, orderstr)
    publish_measurements(language=language,
                         orderstr=orderstr,
                         filename=filename,
                         sizes=sizes,
                         measurement_per_size_by_case_name=measurements,
                         color_by_case=color_by_func_name,
                         dash_by_case=dash_by_lib,
                         legendgroup=func_name)


def publish_measurements(
        language: str,
        orderstr: str,
        filename: str,
        sizes: List[int],
        measurement_per_size_by_case_name: Mapping[str, List[Measurement]],
        color_by_case: Optional[Callable[[str], str]] = None,
        dash_by_case: Optional[Callable[[str], str]] = None,
        legendgroup: Optional[Callable[[str], str]] = None):
    try:
        from chart_studio import plotly
    except ImportError as e:
        print(f"{e} (maybe you want to pip install chart-studio?)")
    else:
        # Group traces in legend, unless every func_name is either unique or the same
        case_names = measurement_per_size_by_case_name.keys()
        unique_func_names = len({func_name(n) for n in case_names})
        legendgroups = len(case_names) > 6 and unique_func_names in range(
            2, len(case_names))

        import plotly.io as pio
        pio.templates.default = "none"  # disable default 4.0 theme
        from plotly import graph_objects
        traces = [
            graph_objects.Scatter(
                x=sizes,
                y=[measurement_per_size[s].mean for s in range(len(sizes))],
                error_y=dict(
                    type="data",
                    array=[
                        measurement_per_size[s].error_plus()
                        for s in range(len(sizes))
                    ],
                    arrayminus=[
                        measurement_per_size[s].error_minus()
                        for s in range(len(sizes))
                    ],
                ),
                hoverinfo="name",
                line=None if color_by_case is None else dict(
                    color=color_by_case(case_name),
                    dash=None
                    if dash_by_case is None else dash_by_case(case_name)),
                marker=None if color_by_case is None else dict(
                    color=color_by_case(case_name)),
                mode="lines+markers",
                name=case_name,
                legendgroup=None
                if legendgroup is None else legendgroup(case_name),
            ) for case_name, measurement_per_size in
            measurement_per_size_by_case_name.items()
        ]
        layout = dict(
            title=(
                '<a href="https://github.com/ssomers/Bron-Kerbosch">' +
                f"{language.capitalize()} implementations of Bron-Kerbosch" +
                "</a>" + f" on random graphs of order {orderstr}"),
            xaxis=dict(title="Size (#edges)"),
            yaxis=dict(title="Seconds spent"),
        )
        plotly.plot(figure_or_data=dict(data=traces, layout=layout),
                    filename=filename)


def publish_result(orderstr: str, filename: str, langlibs: List[str],
                   versions: List[str]):
    sizes = []
    measurements = {}
    languages = set(langlib.split('@', 1)[0] for langlib in langlibs)
    language = languages.pop() if len(languages) == 1 else None
    for i, langlib in enumerate(langlibs):
        lang_lib = langlib.split('@', 1)
        lang = lang_lib[0].capitalize()
        lib = ("@" + lang_lib[1]) if len(lang_lib) > 1 else ""
        _, sizes1, measurements1 = read_csv(
            language=lang,
            orderstr=orderstr,
            case_name_selector={
                f"{ver}{lib}":
                f"{ver}{lib}" if language else f"{lang} {ver}{lib}"
                for ver in versions
            })
        if i == 0:
            sizes = sizes1
            if len(langlibs) == 1:
                language = lang
        else:
            assert sizes == sizes1, "f{sizes} != {sizes1}"
        measurements.update(measurements1)

    dash_by_case = None
    if len(versions) == 2:
        dash_by_case = lambda case_name: "dot" if f" {versions[0]}" in case_name else "solid"
    publish_measurements(
        language=language or "various",
        orderstr=orderstr,
        filename=filename,
        sizes=sizes,
        measurement_per_size_by_case_name=measurements,
        legendgroup=lang_name if len(langlibs) > 1 else None,
        color_by_case=color_by_language if len(langlibs) > 1 else None,
        dash_by_case=dash_by_case)


if __name__ == '__main__':
    if len(sys.argv) == 1:
        publish_result(filename="bron_kerbosch_result_1",
                       orderstr="100",
                       langlibs=["python3", "rust@fnv"],
                       versions=["Ver0", "Ver1"])
        publish_result(
            filename="bron_kerbosch_result_2",
            orderstr="100",
            langlibs=["c++@hashset", "scala", "python3", "java", "rust@fnv"],
            versions=["Ver1", "Ver2"])
        for orderstr in ["100", "10k"]:
            publish_result(
                filename=f"bron_kerbosch_result_3_python3_{orderstr}",
                orderstr=orderstr,
                langlibs=["python3"],
                versions=["Ver2", "Ver2-G", "Ver2-GP", "Ver2-GPX", "Ver2-RP"])
            publish_result(filename=f"bron_kerbosch_result_3_java_{orderstr}",
                           orderstr=orderstr,
                           langlibs=["java"],
                           versions=["Ver2", "Ver2-G", "Ver2-GP", "Ver2-GPX"])
        for orderstr in ["100", "10k", "1M"]:
            publish_result(
                filename=f"bron_kerbosch_result_4_python3_{orderstr}",
                orderstr=orderstr,
                langlibs=["python3"],
                versions=["Ver2-GP", "Ver2-GPX", "Ver3-GP", "Ver3-GPX"])
            publish_result(
                filename=f"bron_kerbosch_result_4_c#_{orderstr}",
                orderstr=orderstr,
                langlibs=["c#"],
                versions=["Ver2-GP", "Ver2-GPX", "Ver3-GP", "Ver3-GPX"])
        for orderstr in ["100", "10k"]:
            publish_result(filename=f"bron_kerbosch_result_4_rust_{orderstr}",
                           orderstr=orderstr,
                           langlibs=["rust"],
                           versions=["Ver2-GP@fnv", "Ver2-GPX@fnv", "Ver3-GP@fnv", "Ver3-GPX@fnv"])
            publish_result(
                filename=f"bron_kerbosch_result_4_java_{orderstr}",
                orderstr=orderstr,
                langlibs=["java"],
                versions=["Ver2-GP", "Ver2-GPX", "Ver3-GP", "Ver3-GPX"])
            publish_result(
                filename=f"bron_kerbosch_result_4_go_{orderstr}",
                orderstr=orderstr,
                langlibs=["go"],
                versions=["Ver2-GP", "Ver2-GPX", "Ver3-GP", "Ver3-GPX"])
    else:
        for orderstr in sys.argv[2:]:
            publish_whole_csv(language=sys.argv[1], orderstr=orderstr)
