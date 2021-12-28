from collections import OrderedDict
from stats import SampleStatistics
from bisect import bisect_left
import csv
import math
import os
import sys
from typing import Callable, Dict, List, Mapping, Optional, Sequence, Tuple

figsize_inline = (8, 6)  # in hectopixels
figsize_detail = (12, 9)  # in hectopixels


def lang_name(case_name: str) -> str:
    return case_name.split(' ', 1)[0].split('@', 1)[0]


def func_name(case_name: str) -> str:
    return case_name.split(' ')[-1].split('@')[0]


def color_by_func_name(case_name: str) -> str:
    return {
        "Ver1": "#000099",
        "Ver1½": "#6666FF",
        "Ver2": "#660000",
        "Ver2½": "#CC0000",
        "Ver2-GP": "#666633",
        "Ver2½-GP": "#FF6600",
        "Ver2½-GPX": "#FFCC33",
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


def linestyle_by_lib(lib: str) -> object:
    return {
        "BTree": "dotted",
        "std_set": "dotted",
        "ord_vec": "dashed",
        "hashbrown": "dashdot",
        "fnv": (0, (4, 2, 1, 1, 1, 2)),
    }.get(lib)


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
) -> Tuple[List[int], Mapping[str, List[Measurement]]]:
    filename = f"bron_kerbosch_{language}_order_{orderstr}"
    path = filename + ".csv"
    if not os.path.exists(path):
        path = os.path.join(os.pardir, path)
    sizes = []
    m_per_size_by_case_name: Dict[str, List[Measurement]] = {}
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
                    m_per_size_by_case_name.setdefault(published_name,
                                                       []).append(m)

    for case_name in list(m_per_size_by_case_name.keys()):
        if all(m_per_size_by_case_name[case_name][s].isnan()
               for s in range(len(sizes))):
            print(f"{filename}: backing out on {case_name}")
            del m_per_size_by_case_name[case_name]
    return sizes, m_per_size_by_case_name


def import_matplotlib() -> bool:
    try:
        import matplotlib  # type: ignore
    except ImportError as e:
        print(f"{e} (maybe you want to `pip install matplotlib`?)")
        return False
    else:
        matplotlib.rcParams['svg.hashsalt'] = "Bron-Kerbosch"
        return True


def publish_whole_csv(language: str, orderstr: str) -> None:
    m_per_size_by_case_name: Mapping[str, List[Measurement]]
    sizes, m_per_size_by_case_name = read_csv(language, orderstr)
    filename = f'details_{language.replace("c#", "csharp")}_{orderstr}'
    assert sizes
    assert m_per_size_by_case_name
    assert all(
        len(m_per_size) == len(sizes)
        for m_per_size in m_per_size_by_case_name.values())
    if import_matplotlib():
        if sizes[-1] > 1_000_000:
            cutoff = bisect_left(sizes, 500_000)
            publish_details(
                language, orderstr, filename + "_initial", sizes[:cutoff], {
                    case_name: m_per_size[:cutoff]
                    for case_name, m_per_size in
                    m_per_size_by_case_name.items()
                })
            publish_details(
                language, orderstr, filename, sizes[cutoff:], {
                    case_name: m_per_size[cutoff:]
                    for case_name, m_per_size in m_per_size_by_case_name.items(
                    ) if not all(m.isnan() for m in m_per_size[cutoff:])
                })
        else:
            publish_details(language, orderstr, filename, sizes,
                            m_per_size_by_case_name)


def publish_details(
        language: str, orderstr: str, filename: str, sizes: List[int],
        m_per_size_by_case_name: Mapping[str, List[Measurement]]) -> None:
    assert sizes
    assert m_per_size_by_case_name
    assert all(
        len(m_per_size) == len(sizes)
        for m_per_size in m_per_size_by_case_name.values())
    from matplotlib import pyplot
    fig, axes = pyplot.subplots(figsize=figsize_detail)
    axes.set_title(f"{language.capitalize()} implementations of " +
                   f"Bron-Kerbosch on a random graph of order {orderstr}")
    axes.set_xlabel("Size (#edges)")
    axes.set_ylabel("Seconds spent")
    axes.set_yscale("log")
    linestyles: Dict[str, object] = OrderedDict()
    for case_name, m_per_size in m_per_size_by_case_name.items():
        names = case_name.split('@')
        func_name = names[0]
        lib = (names + [""])[1]
        linestyle = linestyle_by_lib(lib)
        linestyles.setdefault(lib, linestyle)
        axes.errorbar(x=sizes[:len(m_per_size)],
                      y=[m.mean for m in m_per_size],
                      yerr=[[m.error_minus() for m in m_per_size],
                            [m.error_plus() for m in m_per_size]],
                      label=func_name if linestyle is None else None,
                      capsize=3,
                      color=color_by_func_name(case_name),
                      linestyle=linestyle)
    axes.legend(loc="upper left")
    if len(linestyles) > 1:
        twin = axes.twinx()
        twin.get_yaxis().set_visible(False)
        for lib, linestyle in linestyles.items():
            twin.plot([], [], linestyle=linestyle, label=lib, color="black")
        twin.legend(loc="lower right")
    fig.tight_layout()
    fig.savefig(filename + ".svg", bbox_inches=0, pad_inches=0)


def publish_measurements(
        language: Optional[str],
        orderstr: str,
        filename: str,
        sizes: List[int],
        measurement_per_size_by_case_name: Mapping[str, List[Measurement]],
        suffix: str,
        color_by_case: Optional[Callable[[str], str]] = None,
        dash_by_case: Optional[Callable[[str], str]] = None) -> None:
    assert sizes
    assert measurement_per_size_by_case_name, filename
    if import_matplotlib():
        from matplotlib import pyplot
        fig, axes = pyplot.subplots(figsize=figsize_inline)
        axes.set_title(
            (f"{language.capitalize()} implementations of "
             if language else "") +
            f"Bron-Kerbosch{suffix} on a random graph of order {orderstr}",
            loc="left")
        axes.set_xlabel("Size (#edges)")
        axes.set_ylabel("Seconds spent")
        for case_name, m_per_size in measurement_per_size_by_case_name.items():
            axes.errorbar(x=sizes[:len(m_per_size)],
                          y=[m.mean for m in m_per_size],
                          yerr=[[m.error_minus() for m in m_per_size],
                                [m.error_plus() for m in m_per_size]],
                          label=case_name,
                          capsize=3,
                          color=(None if color_by_case is None else
                                 color_by_case(case_name)),
                          linestyle=(None if dash_by_case is None else
                                     dash_by_case(case_name)))
        axes.legend(loc="upper left")
        fig.tight_layout()
        fig.savefig(filename + ".svg", bbox_inches=0, pad_inches=0)
        pyplot.close(fig)


def publish_report(orderstr: str,
                   filename: str,
                   langlibs: Sequence[str],
                   versions: Sequence[str],
                   single_version: Optional[str] = None) -> None:
    sizes: List[int] = []
    measurements: Dict[str, List[Measurement]] = {}
    languages = set(langlib.split('@', 1)[0] for langlib in langlibs)
    single_language = languages.pop() if len(languages) == 1 else None
    for langlib in langlibs:
        lang_lib = langlib.split('@', 1)
        lang = lang_lib[0].capitalize()
        lib = ("@" + lang_lib[1]) if len(lang_lib) > 1 else ""
        sizes1, measurements1 = read_csv(
            language=lang,
            orderstr=orderstr,
            case_name_selector={
                f"{ver}{lib}":
                (f"{lib}" if single_language and single_version else
                 f"{ver}{lib}" if single_language else
                 f"{lang}" if single_version else f"{lang}{lib} {ver}")
                for ver in versions
            })
        if orderstr == "1M":
            cutoff = bisect_left(sizes1, 250_000)
            sizes1 = sizes1[cutoff:]
            measurements1 = {n: m[cutoff:] for n, m in measurements1.items()}
        if sizes[:len(sizes1)] != sizes1[:len(sizes)]:
            raise ImportError(f"{sizes} != {sizes1} for {lang} {orderstr}")
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
        suffix="" if single_version is None else " " + single_version,
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
                           "c++@hashset", "rust@Hash"
                       ],
                       versions=["Ver3½-GP"],
                       single_version="Ver3½-GP")
        publish_report(
            filename=f"report_6_channels_{orderstr}",
            orderstr=orderstr,
            langlibs=["java", "go", "c#", "c++@hashset", "rust@Hash"],
            versions=["Ver3½=GPc", "Ver3½=GP3"],
            single_version="parallel Ver3½=GP using channels")
        publish_report(filename=f"report_6_parallel_{orderstr}",
                       orderstr=orderstr,
                       langlibs=["java", "scala"],
                       versions=["Ver3½=GPs"],
                       single_version="simple parallel Ver3½=GP")
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
                       versions=["Ver3½-GP"],
                       single_version="Ver3½-GP")
    for orderstr in ["100", "10k"]:
        publish_report(filename=f"report_7_c++_{orderstr}",
                       orderstr=orderstr,
                       langlibs=[
                           "c++@hashset",
                           "c++@std_set",
                           "c++@ord_vec",
                       ],
                       versions=["Ver3½-GP"],
                       single_version="Ver3½-GP")


if __name__ == '__main__':
    if len(sys.argv) == 1:
        publish_reports()
    else:
        for orderstr in sys.argv[2:]:
            publish_whole_csv(language=sys.argv[1], orderstr=orderstr)
