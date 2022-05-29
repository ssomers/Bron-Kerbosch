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


def clean_language(language: str) -> str:
    return language.replace("c#", "csharp")


def csv_basename(language: str, orderstr: str) -> str:
    return f"bron_kerbosch_{clean_language(language)}_order_{orderstr}.csv"


def publish(
        language: str, orderstr: str, case_names: Sequence[str],
        stats_per_func_by_size: Mapping[int, List[SampleStatistics]]) -> None:
    num_cases = len(case_names)
    filename = csv_basename(language, orderstr)
    path = os.path.join(os.pardir, filename)
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
    filename = csv_basename(language, orderstr)
    path = os.path.join(os.pardir, filename)
    if not os.path.exists(path):
        path = filename
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
                    f"{filename} row {i+2}: found {len(row)} columns, expected {expected_cols}"
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
    assert len(sizes)
    assert len(m_per_size_by_case_name)
    assert all(
        len(m_per_size) == len(sizes)
        for m_per_size in m_per_size_by_case_name.values())
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
    sizes, m_per_size_by_case_name = read_csv(language, orderstr)
    if import_matplotlib():
        if cutoff := {
            ("c++", "10k"): 10_000,
            ("rust", "10k"): 50_000,
            ("rust", "1M"): 500_000
        }.get((language, orderstr)):
            idx = bisect_left(sizes, cutoff)
            sizes_1 = sizes[:idx]
            sizes = sizes[idx:]
            m_per_size_by_case_name_1 = {
                case_name: m_per_size[:idx]
                for case_name, m_per_size in m_per_size_by_case_name.items()
            }
            m_per_size_by_case_name = {
                case_name: m_per_size[idx:]
                for case_name, m_per_size in m_per_size_by_case_name.items()
                if not all(m.isnan() for m in m_per_size[idx:])
            }
            publish_details(language,
                            orderstr,
                            sizes_1,
                            m_per_size_by_case_name_1,
                            basename_variant="_initial")
        publish_details(language, orderstr, sizes, m_per_size_by_case_name)


def publish_details(language: str,
                    orderstr: str,
                    sizes: List[int],
                    m_per_size_by_case_name: Mapping[str, List[Measurement]],
                    basename_variant: str = "") -> None:
    assert len(sizes)
    assert len(m_per_size_by_case_name)
    assert all(
        len(m_per_size) == len(sizes)
        for m_per_size in m_per_size_by_case_name.values())
    basename = f'details_{clean_language(language)}_{orderstr}{basename_variant}'
    from matplotlib import pyplot
    fig, axes = pyplot.subplots(figsize=figsize_detail)
    axes.set_title(f"{language.capitalize()} implementations of " +
                   f"Bron-Kerbosch on a random graph of order {orderstr}")
    axes.set_xlabel("Size (#edges)")
    axes.set_ylabel("Seconds spent")
    if not basename_variant:
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
            twin.plot([], linestyle=linestyle, label=lib, color="black")
        twin.legend(loc="lower right")
    fig.tight_layout()
    fig.savefig(basename + ".svg", bbox_inches=0, pad_inches=0)


def publish_measurements(
        basename: str,
        orderstr: str,
        sizes: List[int],
        measurement_per_size_by_case_name: Mapping[str, List[Measurement]],
        suffix: str = "",
        language: Optional[str] = None,
        color_by_case: Optional[Callable[[str], str]] = None,
        dash_by_case: Optional[Callable[[str], str]] = None) -> None:
    assert sizes
    assert measurement_per_size_by_case_name, basename
    print("Generating", basename)
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
        fig.savefig(basename + ".svg", bbox_inches=0, pad_inches=0)
        pyplot.close(fig)


def publish_report(
        basename: str,
        orderstr: str,
        langlibs: Sequence[str],
        versions: Sequence[str],
        single_version: Optional[str] = None,
        dash_by_case: Optional[Callable[[str], str]] = None) -> None:
    sizes: List[int] = []
    measurements: Dict[str, List[Measurement]] = {}
    languages = set(langlib.split('@', 1)[0] for langlib in langlibs)
    assert len(languages) > 1
    for langlib in langlibs:
        lang_lib = langlib.split('@', 1)
        lang = lang_lib[0]
        at_lib = ("@" + lang_lib[1]) if len(lang_lib) > 1 else ""
        sizes1, measurements1 = read_csv(
            language=lang,
            orderstr=orderstr,
            case_name_selector={
                f"{ver}{at_lib}": lang.capitalize() +
                ("" if single_version else f"{at_lib} {ver}")
                for ver in versions
            })
        if cutoff := {
                "10k": 10_000,
                "1M": 500_000,
        }.get(orderstr):
            idx = bisect_left(sizes1, cutoff)
            sizes1 = sizes1[idx:]
            measurements1 = {n: m[idx:] for n, m in measurements1.items()}
        if sizes[:len(sizes1)] != sizes1[:len(sizes)]:
            raise ImportError(f"{sizes} != {sizes1} for {lang} {orderstr}")
        if len(sizes) < len(sizes1):
            sizes = sizes1
        measurements.update(measurements1)

    publish_measurements(
        basename=basename,
        orderstr=orderstr,
        sizes=sizes,
        suffix="" if single_version is None else f" {single_version}",
        measurement_per_size_by_case_name=measurements,
        color_by_case=color_by_language,
        dash_by_case=dash_by_case)


def publish_version_report(basebasename: str, orderstr: str, langlib: str,
                           versions: Sequence[str]) -> None:
    sep = (langlib + '@').index('@')
    lang, at_lib = langlib[:sep], langlib[sep:]
    sizes, measurements = read_csv(
        language=lang,
        orderstr=orderstr,
        case_name_selector={f"{ver}{at_lib}": f"{ver}"
                            for ver in versions})
    publish_measurements(basename=basebasename +
                         f"_{clean_language(lang)}_{orderstr}",
                         language=lang,
                         orderstr=orderstr,
                         sizes=sizes,
                         measurement_per_size_by_case_name=measurements)


def publish_library_report(basename: str, orderstr: str, language: str,
                           ver: str, libs: Sequence[str]) -> None:
    sizes, measurements = read_csv(
        language=language,
        orderstr=orderstr,
        case_name_selector={f"{ver}@{lib}": f"{lib}"
                            for lib in libs})
    publish_measurements(basename=basename,
                         language=language,
                         orderstr=orderstr,
                         sizes=sizes,
                         suffix=" " + ver,
                         measurement_per_size_by_case_name=measurements)


def publish_reports() -> None:
    # 1. Ver1 vs. Ver1½
    publish_report(basename="report_1",
                   orderstr="100",
                   langlibs=["python3", "rust@Hash"],
                   versions=["Ver1", "Ver1½"],
                   dash_by_case=lambda name: "solid"
                   if name.endswith("½") else "dotted")
    # 2. Ver1 vs. Ver2
    publish_report(basename="report_2",
                   orderstr="100",
                   langlibs=["java", "scala", "rust@Hash"],
                   versions=["Ver1½", "Ver2½"],
                   dash_by_case=lambda name: "solid"
                   if name.endswith("2½") else "dotted")
    # 3. Ver2 variants
    for orderstr in ["100", "10k"]:
        for langlib in ["rust@Hash", "java"]:
            publish_version_report(
                basebasename="report_3",
                orderstr=orderstr,
                langlib=langlib,
                versions=["Ver2½", "Ver2½-RP", "Ver2½-GP", "Ver2½-GPX"])
    # 4. Ver2 vs. Ver3
    for orderstr in ["10k", "1M"]:
        for langlib in ["python3", "c#"]:
            publish_version_report(basebasename="report_4",
                                   orderstr=orderstr,
                                   langlib=langlib,
                                   versions=["Ver2½-GP", "Ver3½-GP"])
    for orderstr in ["10k"]:
        for langlib in ["rust@Hash", "java"]:
            publish_version_report(basebasename="report_4",
                                   orderstr=orderstr,
                                   langlib=langlib,
                                   versions=["Ver2½-GP", "Ver3½-GP"])
    # 5. Ver3 variants
    for orderstr in ["10k", "1M"]:
        for langlib in ["python3", "c#"]:
            publish_version_report(basebasename="report_5",
                                   orderstr=orderstr,
                                   langlib=langlib,
                                   versions=["Ver3½-GP", "Ver3½-GPX"])
    for orderstr in ["10k"]:
        for langlib in ["rust@Hash", "java"]:
            publish_version_report(basebasename="report_5",
                                   orderstr=orderstr,
                                   langlib=langlib,
                                   versions=["Ver3½-GP", "Ver3½-GPX"])
    # 6. Parallelism
    for orderstr in ["100", "10k", "1M"]:
        publish_version_report(basebasename="report_6",
                               orderstr=orderstr,
                               langlib="java",
                               versions=["Ver3½-GP", "Ver3½=GPs", "Ver3½=GPc"])
        publish_version_report(basebasename="report_6",
                               orderstr=orderstr,
                               langlib="go",
                               versions=[
                                   "Ver3½-GP", "Ver3½=GP0", "Ver3½=GP1",
                                   "Ver3½=GP2", "Ver3½=GP3", "Ver3½=GP4"
                               ])
    # 7. Languages
    for orderstr in ["100", "10k", "1M"]:
        publish_report(basename=f"report_7_sequential_{orderstr}",
                       orderstr=orderstr,
                       langlibs=[
                           "python3", "scala", "java", "go", "c#",
                           "c++@hashset", "rust@Hash"
                       ],
                       versions=["Ver3½-GP"],
                       single_version="Ver3½-GP")
        publish_report(
            basename=f"report_7_channels_{orderstr}",
            orderstr=orderstr,
            langlibs=["java", "go", "c#", "c++@hashset", "rust@Hash"],
            versions=["Ver3½=GPc", "Ver3½=GP3"],
            single_version="parallel Ver3½=GP using channels")
        publish_report(basename=f"report_7_parallel_{orderstr}",
                       orderstr=orderstr,
                       langlibs=["java", "scala"],
                       versions=["Ver3½=GPs"],
                       single_version="simple parallel Ver3½=GP")
    # 8. Libraries
    for orderstr in ["100", "10k", "1M"]:
        publish_library_report(
            basename=f"report_8_rust_{orderstr}",
            orderstr=orderstr,
            language="rust",
            ver="Ver3½-GP",
            libs=["BTree", "Hash", "hashbrown", "fnv", "ord_vec"],
        )
    for orderstr in ["100", "10k"]:
        publish_library_report(basename=f"report_8_c++_{orderstr}",
                               orderstr=orderstr,
                               language="c++",
                               ver="Ver3½-GP",
                               libs=["hashset", "std_set", "ord_vec"])


if __name__ == '__main__':
    if len(sys.argv) == 1:
        publish_reports()
    else:
        for orderstr in sys.argv[2:]:
            publish_whole_csv(language=sys.argv[1], orderstr=orderstr)
