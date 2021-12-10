from stats import SampleStatistics
import csv
import math
import os
import sys
from typing import Callable, Dict, List, Mapping, Optional, Sequence, Tuple

figsize = (8, 6)  # in hectopixels


def lang_name(case_name: str) -> str:
    return case_name.split(' ', 1)[0].split('@', 1)[0]


def func_name(case_name: str) -> str:
    return case_name.split(' ')[-1].split('@')[0]


def color_by_func_name(case_name: str) -> str:
    return {
        "Ver1": "#000099",
        "Ver1½": "#3333CC",
        "Ver2": "#660000",
        "Ver2½": "#990000",
        "Ver2-GP": "#996600",
        "Ver2½-GP": "#FF3300",
        "Ver2½-GPX": "#FF9933",
        "Ver2½-RP": "#FF00CC",
        "Ver3½": "#006600",
        "Ver3½-GP": "#339900",
        "Ver3½-GPX": "#33CC00",
        "Ver3½=GPc": "#669999",
        "Ver3½=GPs": "#669966",
        "Ver3½=GP0": "#3399CC",
        "Ver3½=GP1": "#669999",
        "Ver3½=GP2": "#669966",
        "Ver3½=GP3": "#669933",
        "Ver3½=GP4": "#669900",
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
    def __init__(self, min: float, mean: float, max: float) -> None:
        self.min = min
        self.mean = mean
        self.max = max

    def isnan(self) -> bool:
        return math.isnan(self.mean)

    def error_plus(self) -> float:
        return self.max - self.mean

    def error_minus(self) -> float:
        return self.mean - self.min


def publish(
        language: str, orderstr: str, case_names: Sequence[str],
        stats_per_func_by_size: Mapping[int, List[SampleStatistics]]) -> None:
    num_cases = len(case_names)
    filename = f"bron_kerbosch_{language}_order_{orderstr}"
    path = os.path.join(os.pardir, filename + ".csv")
    with open(path, 'w', newline='', encoding='utf-8') as csvfile:
        w = csv.writer(csvfile)
        w.writerow(["Size"] + [(f"{name} {t}") for name in case_names
                               for t in ["min", "mean", "max"]])
        for size, stats in stats_per_func_by_size.items():
            w.writerow(
                [str(size)] +
                [str(f) for s in stats for f in [s.min, s.mean(), s.max]])
    publish_whole_csv(language=language, orderstr=orderstr)


def read_csv(
    language: str,
    orderstr: str,
    case_name_selector: Mapping[str, str] = {}
) -> Tuple[str, List[int], Mapping[str, List[Measurement]]]:
    filename = f"bron_kerbosch_{language}_order_{orderstr}"
    path = filename + ".csv"
    if not os.path.exists(path):
        path = os.path.join(os.pardir, path)
    sizes = []
    measurement_per_size_by_case_name: Dict[str, List[Measurement]] = {}
    with open(path, newline='', encoding='utf-8') as csvfile:
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
                published_name = (case_name_selector.get(case_name)
                                  if case_name_selector else case_name)
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


def publish_whole_csv(language: str, orderstr: str) -> None:
    filename, sizes, measurement_per_size_by_case_name = read_csv(
        language, orderstr)
    assert sizes
    assert measurement_per_size_by_case_name
    try:
        from chart_studio import plotly  # type: ignore
        from plotly import io as plotly_io  # type: ignore
        from plotly import graph_objects
    except ImportError as e:
        print(f"{e} (maybe you want to pip install chart-studio?)")
    else:
        plotly_io.templates.default = "none"  # disable default 4.0 theme
        traces = [
            graph_objects.Scatter(
                x=sizes,
                y=[m.mean for m in m_per_size],
                error_y=dict(
                    type="data",
                    array=[m.error_plus() for m in m_per_size],
                    arrayminus=[m.error_minus() for m in m_per_size],
                ),
                hoverinfo="name",
                line=dict(color=color_by_func_name(case_name),
                          dash=dash_by_lib(case_name)),
                marker=dict(color=color_by_func_name(case_name)),
                mode="lines+markers",
                name=case_name,
            ) for case_name, m_per_size in
            measurement_per_size_by_case_name.items()
        ]
        layout = dict(
            title=(
                '<a href="https://github.com/ssomers/Bron-Kerbosch">' +
                f"{language.capitalize()} implementations of Bron-Kerbosch</a>"
                + f" on a random graph of order {orderstr}"),
            xaxis=dict(title="Size (#edges)"),
            yaxis=dict(title="Seconds spent"),
        )
        plotly.plot(figure_or_data=dict(data=traces, layout=layout),
                    filename=filename)


def publish_measurements(
        language: Optional[str],
        orderstr: str,
        filename: str,
        sizes: List[int],
        measurement_per_size_by_case_name: Mapping[str, List[Measurement]],
        version: Optional[str] = None,
        color_by_case: Optional[Callable[[str], str]] = None,
        dash_by_case: Optional[Callable[[str], str]] = None) -> None:
    assert sizes
    assert measurement_per_size_by_case_name, filename
    try:
        import matplotlib  # type: ignore
        from matplotlib import pyplot
    except ImportError as e:
        print(f"{e} (maybe you want to pip install matplotlib?)")
    else:
        matplotlib.rcParams['svg.hashsalt'] = "Bron-Kerbosch"
        fig, ax = pyplot.subplots(figsize=figsize)
        ax.set_title((f"{language.capitalize()} implementations"
                      if language else "Implementations") +
                     " of Bron-Kerbosch" + (f" {version}" if version else "") +
                     f" on a random graph of order {orderstr}",
                     loc="left")
        ax.set_xlabel("Size (#edges)")
        ax.set_ylabel("Seconds spent")
        for case_name, m_per_size in measurement_per_size_by_case_name.items():
            ax.errorbar(x=sizes[:len(m_per_size)],
                        y=[m.mean for m in m_per_size],
                        yerr=[[m.error_minus() for m in m_per_size],
                              [m.error_plus() for m in m_per_size]],
                        label=case_name,
                        capsize=3,
                        color=(None if color_by_case is None else
                               color_by_case(case_name)),
                        linestyle=(None if dash_by_case is None else
                                   dash_by_case(case_name)))
        ax.legend(loc="upper left")
        fig.tight_layout()
        fig.savefig(filename + ".svg", bbox_inches=0, pad_inches=0)
        pyplot.close(fig)


def publish_report(orderstr: str, filename: str, langlibs: Sequence[str],
                   versions: Sequence[str]) -> None:
    sizes: List[int] = []
    measurements: Dict[str, List[Measurement]] = {}
    languages = set(langlib.split('@', 1)[0] for langlib in langlibs)
    single_language = languages.pop() if len(languages) == 1 else None
    single_version = versions[0] if len(versions) == 1 else None
    for langlib in langlibs:
        lang_lib = langlib.split('@', 1)
        lang = lang_lib[0].capitalize()
        lib = ("@" + lang_lib[1]) if len(lang_lib) > 1 else ""
        filename1, sizes1, measurements1 = read_csv(
            language=lang,
            orderstr=orderstr,
            case_name_selector={
                f"{ver}{lib}":
                (f"{ver}{lib}" if single_language else
                 f"{lang}" if single_version else f"{lang}{lib} {ver}")
                for ver in versions
            })
        if sizes[:len(sizes1)] != sizes1[:len(sizes)]:
            raise ImportError(f"{sizes} != {sizes1} in {filename1}")
        if len(sizes) < len(sizes1):
            sizes = sizes1
        measurements.update(measurements1)

    dash_by_case = None
    if len(versions) == 2:

        def dash_by_case(case_name: str) -> str:
            return "dotted" if case_name.endswith(
                f" {versions[0]}") else "solid"

    publish_measurements(
        language=single_language,
        orderstr=orderstr,
        filename=filename,
        sizes=sizes,
        version=single_version,
        measurement_per_size_by_case_name=measurements,
        color_by_case=color_by_language if single_language is None else None,
        dash_by_case=dash_by_case)


def publish_reports() -> None:
    # 1. Ver1 vs. Ver1½
    publish_report(filename="report_1",
                   orderstr="100",
                   langlibs=["python3", "rust@Hash"],
                   versions=["Ver1", "Ver1½"])
    # 2. Ver1 vs. Ver2
    publish_report(filename="report_2",
                   orderstr="100",
                   langlibs=["java", "scala", "rust@Hash"],
                   versions=["Ver1½", "Ver2½"])
    # 3. Ver2 variants
    for orderstr in ["100", "10k"]:
        for langlib in ["rust@Hash", "java"]:
            lang = langlib.split('@', 1)[0]
            publish_report(
                filename=f"report_3_{lang}_{orderstr}",
                orderstr=orderstr,
                langlibs=[langlib],
                versions=["Ver2½", "Ver2½-RP", "Ver2½-GP", "Ver2½-GPX"])
    # 4. Ver2 vs. Ver3
    for orderstr in ["10k", "1M"]:
        for langlib in ["python3", "c#"]:
            lang = langlib.split('@', 1)[0]
            lang = langlib.split('@', 1)[0].replace("c#", "csharp")
            publish_report(
                filename=f"report_4_{lang}_{orderstr}",
                orderstr=orderstr,
                langlibs=[langlib],
                versions=["Ver2½-GP", "Ver2½-GPX", "Ver3½-GP", "Ver3½-GPX"])
    for orderstr in ["10k"]:
        for langlib in ["rust@Hash", "java"]:
            lang = langlib.split('@', 1)[0]
            publish_report(
                filename=f"report_4_{lang}_{orderstr}",
                orderstr=orderstr,
                langlibs=[langlib],
                versions=["Ver2½-GP", "Ver2½-GPX", "Ver3½-GP", "Ver3½-GPX"])
    # 5. Parallelism
    for orderstr in ["100", "10k", "1M"]:
        publish_report(filename=f"report_5_java_{orderstr}",
                       orderstr=orderstr,
                       langlibs=["java"],
                       versions=["Ver3½-GP", "Ver3½=GPs", "Ver3½=GPc"])
        publish_report(filename=f"report_5_go_{orderstr}",
                       orderstr=orderstr,
                       langlibs=["go"],
                       versions=[
                           "Ver3½-GP", "Ver3½=GP0", "Ver3½=GP1", "Ver3½=GP2",
                           "Ver3½=GP3", "Ver3½=GP4"
                       ])
    # 6. Languages
    for orderstr in ["100", "10k", "1M"]:
        publish_report(filename=f"report_6_{orderstr}",
                       orderstr=orderstr,
                       langlibs=[
                           "python3", "scala", "java", "go", "c#",
                           "c++@hashset", "rust@fnv"
                       ],
                       versions=["Ver3½-GP"])
        publish_report(
            filename=f"report_6_channels_{orderstr}",
            orderstr=orderstr,
            langlibs=["java", "go", "c#", "c++@hashset", "rust@fnv"],
            versions=["Ver3½=GPc"])
        publish_report(filename=f"report_6_parallel_{orderstr}",
                       orderstr=orderstr,
                       langlibs=["java", "scala"],
                       versions=["Ver3½=GPs"])
    # 7. Libraries
    for orderstr in ["100", "10k", "1M"]:
        publish_report(filename=f"report_7_rust_{orderstr}",
                       orderstr=orderstr,
                       langlibs=[
                           "rust@BTree",
                           "rust@Hash",
                           "rust@hashbrown",
                           "rust@fnv",
                           "rust@ord_vec",
                       ],
                       versions=["Ver3½-GP"])
    for orderstr in ["100", "10k"]:
        publish_report(filename=f"report_7_c++_{orderstr}",
                       orderstr=orderstr,
                       langlibs=[
                           "c++@hashset",
                           "c++@std_set",
                           "c++@ord_vec",
                       ],
                       versions=["Ver3½-GP"])


if __name__ == '__main__':
    if len(sys.argv) == 1:
        publish_reports()
    else:
        for orderstr in sys.argv[2:]:
            publish_whole_csv(language=sys.argv[1], orderstr=orderstr)
