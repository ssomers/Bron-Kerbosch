from collections import OrderedDict
from stats import SampleStatistics
from bisect import bisect_left
from enum import Enum, auto
import csv
import math
import os
import sys
from typing import Callable, cast, Dict, List, Mapping, Optional, Sequence, Tuple
from typing_extensions import Self

figsize_inline = (8, 6)  # in hectopixels
figsize_detail = (12, 9)  # in hectopixels


class Language(Enum):
    cpp = auto()
    csharp = auto()
    go = auto()
    java = auto()
    kotlin = auto()
    python = auto()
    python310 = auto()
    python311 = auto()
    python313 = auto()
    rust = auto()

    def long_name(self) -> str:
        return {
            Language.cpp: "C++",
            Language.csharp: "C# .NET 9",
            Language.go: "Go 1.25",
            Language.java: "Java 24",
            Language.kotlin: "Kotlin 2",
            Language.python: "Python",
            Language.python310: "Python 3.10",
            Language.python311: "Python 3.11",
            Language.python313: "Python 3.13",
            Language.rust: "Rust 1.90",
        }[self]

    def short_name(self) -> str:
        return {
            Language.cpp: "C++",
            Language.csharp: "C#",
            Language.go: "Go",
            Language.java: "Java",
            Language.kotlin: "Kotlin",
            Language.python: "Python",
            Language.python310: "Python",
            Language.python311: "Python",
            Language.python313: "Python",
            Language.rust: "Rust",
        }[self]


class LangLib:
    def __init__(self, language: Language, lib: Optional[str] = None):
        self.Language = language
        assert lib != ""
        self.Lib = lib

    def at_lib(self) -> str:
        return f"@{self.Lib}" if self.Lib else ""

    def __eq__(self, them: object) -> bool:
        assert isinstance(them, LangLib)
        return (self.Language, self.Lib) == (them.Language, them.Lib)


class Case:
    @classmethod
    def from_csv_header(cls, csv_header: str, language: Language) -> Self:
        parts = csv_header.split("@", 1) + [None]
        ver, lib = parts[0:2]
        assert ver
        return cls(LangLib(language, lib), ver)

    def __init__(self, langlib: LangLib, ver: str):
        self.LangLib = langlib
        self.Ver = ver

    def __str__(self) -> str:
        return f"{self.LangLib.Language.name}{self.LangLib.at_lib()} {self.Ver}"

    def __hash__(self) -> int:
        return hash((self.LangLib.Language, self.LangLib.Lib, self.Ver))

    def __eq__(self, them: object) -> bool:
        assert isinstance(them, Case)
        return (self.LangLib, self.Ver) == (them.LangLib, them.Ver)



def color_by_ver(case: Case) -> str:
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
        "Ver3½=GP2": "#996666",
        "Ver3½=GP3": "#CC9966",
        "Ver3½=GP4": "#CCCC66",
    }[case.Ver]


def color_by_language(case: Case) -> str:
    return {
        Language.python: "#000099",
        Language.python313: "#000099",
        Language.rust: "#CC0033",
        Language.java: "#009933",
        Language.kotlin: "#CC9933",
        Language.csharp: "#666600",
        Language.cpp: "#990000",
        Language.go: "#0000CC",
    }[case.LangLib.Language]


def linestyle_by_lib(lib: str) -> object:
    return {
        "BTree": "dotted",
        "std_set": "dotted",
        "SortedSet": "dotted",
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


def csv_basename(language: Language, orderstr: str) -> str:
    return f"bron_kerbosch_{language.name}_order_{orderstr}.csv"


def publish(
    languagestr: str,
    orderstr: str,
    case_names: Sequence[str],
    stats_per_func_by_size: Mapping[int, List[SampleStatistics]],
) -> None:
    language = Language[languagestr]
    filename = csv_basename(language, orderstr)
    path = os.path.join(os.pardir, filename)
    with open(path, "w", newline="", encoding="utf-8") as csvfile:
        w = csv.writer(csvfile)
        w.writerow(
            ["Size"]
            + [(f"{name} {t}") for name in case_names for t in ["min", "mean", "max"]]
        )
        for size, stats in stats_per_func_by_size.items():
            w.writerow(
                [str(size)] + [str(f) for s in stats for f in [s.min, s.mean(), s.max]]
            )
    publish_whole_csv(language=language, orderstr=orderstr)


def read_seconds(s: str) -> float:
    return float(s) if s else math.nan


def read_csv(
    language: Language,
    orderstr: str,
    case_selector: Callable[[Case], bool] = lambda _: True,
) -> Tuple[List[int], Mapping[Case, List[Measurement]]]:
    filename = csv_basename(language, orderstr)
    path = os.path.join(os.pardir, filename)
    if not os.path.exists(path):
        path = filename
    print("Reading", filename)
    sizes = []
    m_per_size_by_case: Dict[Case, List[Measurement]] = {}
    with open(path, newline="", encoding="utf-8") as csvfile:
        reader = csv.reader(csvfile)
        try:
            head = next(reader)
        except StopIteration:
            raise ImportError(f"{filename}: seems to be empty")
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
        cases: list[Case] = []
        for case_name in [h.split()[0] for h in head[2::3]]:
            try:
                case = Case.from_csv_header(case_name, language=language)
            except KeyError as e:
                raise ImportError(
                    f'{filename}: unrecognized case name "{case_name}"'
                ) from e
            cases.append(case)
        for i, row in enumerate(reader):
            if len(row) != expected_cols:
                raise ImportError(
                    f"{filename} row {i+2}: found {len(row)} columns,"
                    f"expected {expected_cols}"
                )
            size = int(row[0])
            sizes.append(size)
            for c, case in enumerate(cases):
                if case_selector(case):
                    m = Measurement(
                        min=read_seconds(row[c * 3 + 1]),
                        mean=read_seconds(row[c * 3 + 2]),
                        max=read_seconds(row[c * 3 + 3]),
                    )
                    m_per_size_by_case.setdefault(case, []).append(m)

    for case in list(m_per_size_by_case.keys()):
        if all(m_per_size_by_case[case][s].isnan() for s in range(len(sizes))):
            print(f"{filename}: backing out on {case}")
            del m_per_size_by_case[case]
    assert len(sizes)
    assert len(m_per_size_by_case)
    for m_per_size in m_per_size_by_case.values():
        if len(m_per_size) != len(sizes):
            breakpoint()
    return sizes, m_per_size_by_case


def import_matplotlib() -> bool:
    try:
        import matplotlib
    except ImportError as e:
        print(f"{e} (maybe you want to `pip install matplotlib`?)")
        return False
    else:
        matplotlib.rcParams["svg.hashsalt"] = "Bron-Kerbosch"
        return True


def publish_whole_csv(language: Language, orderstr: str) -> None:
    sizes, m_per_size_by_case = read_csv(language, orderstr)
    if import_matplotlib():
        if cutoff := {
            (Language.cpp, "10k"): 10_000,
            (Language.rust, "10k"): 50_000,
            (Language.rust, "1M"): 500_000,
        }.get((language, orderstr)):
            idx = bisect_left(sizes, cutoff)
            sizes_1 = sizes[:idx]
            sizes = sizes[idx:]
            m_per_size_by_case_1 = {
                case: m_per_size[:idx]
                for case, m_per_size in m_per_size_by_case.items()
            }
            m_per_size_by_case = {
                case: m_per_size[idx:]
                for case, m_per_size in m_per_size_by_case.items()
                if not all(m.isnan() for m in m_per_size[idx:])
            }
            publish_details(
                language,
                orderstr,
                sizes_1,
                m_per_size_by_case_1,
                basename_variant="_initial",
            )
        publish_details(language, orderstr, sizes, m_per_size_by_case)


def publish_details(
    language: Language,
    orderstr: str,
    sizes: List[int],
    m_per_size_by_case: Mapping[Case, List[Measurement]],
    basename_variant: str = "",
) -> None:
    assert len(sizes)
    assert len(m_per_size_by_case)
    assert all(
        len(m_per_size) == len(sizes) for m_per_size in m_per_size_by_case.values()
    )
    filename = f"details_{language.name}_{orderstr}{basename_variant}.svg"
    if import_matplotlib():
        from matplotlib import pyplot

        fig, axes = pyplot.subplots(figsize=figsize_detail)
        axes.set_title(
            f"{language.long_name()} implementations of "
            + f"Bron-Kerbosch on a random graph of order {orderstr}"
        )
        axes.set_xlabel("Size (#edges)")
        axes.set_ylabel("Seconds spent")
        if not basename_variant:
            axes.set_yscale("log")
        linestyles: Dict[str, object] = OrderedDict()
        for case, m_per_size in m_per_size_by_case.items():
            lib = case.LangLib.Lib
            if lib is None:
                linestyle = None
            else:
                linestyle = linestyles.setdefault(lib, linestyle_by_lib(lib))
            axes.errorbar(
                x=sizes[: len(m_per_size)],
                y=[m.mean for m in m_per_size],
                yerr=[
                    [m.error_minus() for m in m_per_size],
                    [m.error_plus() for m in m_per_size],
                ],
                label=case.Ver if linestyle is None else None,
                capsize=3,
                color=color_by_ver(case),
                linestyle=linestyle,
            )
        axes.legend(loc="upper left")
        if linestyles:
            twin = axes.twinx()
            twin.get_yaxis().set_visible(False)
            for lib, linestyle in linestyles.items():
                twin.plot([], linestyle=linestyle, label=lib, color="black")
            twin.legend(loc="lower right")
        fig.tight_layout()
        print("Writing", filename)
        fig.savefig(filename, bbox_inches=0, pad_inches=0)


def publish_measurements(
    basename: str,
    orderstr: str,
    sizes: List[int],
    measurement_per_size_by_case: Mapping[Case, List[Measurement]],
    label_by_case: Callable[[Case], str],
    suffix: str = "",
    language: Optional[Language] = None,
    color_by_case: Callable[[Case], Optional[str]] = lambda _: None,
    linestyle_by_case: Callable[[Case], Optional[str]] = lambda _: None,
) -> None:
    assert sizes
    assert measurement_per_size_by_case, basename
    filename = basename + ".svg"
    if import_matplotlib():
        from matplotlib import pyplot

        fig, axes = pyplot.subplots(figsize=figsize_inline)
        axes.set_title(
            (f"{language.short_name()} implementations of " if language else "")
            + f"Bron-Kerbosch{suffix} on a random graph of order {orderstr}",
            loc="left",
        )
        axes.set_xlabel("Size (#edges)")
        axes.set_ylabel("Seconds spent")
        for case, m_per_size in measurement_per_size_by_case.items():
            axes.errorbar(
                x=sizes[: len(m_per_size)],
                y=[m.mean for m in m_per_size],
                yerr=[
                    [m.error_minus() for m in m_per_size],
                    [m.error_plus() for m in m_per_size],
                ],
                label=label_by_case(case),
                capsize=3,
                color=color_by_case(case),
                linestyle=linestyle_by_case(case),
            )
        axes.legend(loc="upper left")
        fig.tight_layout()
        print("Writing", filename)
        fig.savefig(filename, bbox_inches=0, pad_inches=0)
        pyplot.close(fig)


def publish_report(
    basename: str,
    orderstr: str,
    langlibs: Sequence[LangLib],
    versions: Sequence[str],
    linestyle_per_version: Sequence[Optional[str]],
    label_by_case: Callable[[Case], str],
    single_version_suffix: Optional[str] = None,
) -> None:
    sizes: List[int] = []
    measurements: Dict[Case, List[Measurement]] = {}
    assert len(versions) == len(linestyle_per_version)
    for langlib in langlibs:
        sizes1, measurements1 = read_csv(
            language=langlib.Language,
            orderstr=orderstr,
            case_selector=lambda case: case.LangLib == langlib and case.Ver in versions,
        )
        if cutoff := {
            "10k": 10_000,
            "1M": 500_000,
        }.get(orderstr):
            idx = bisect_left(sizes1, cutoff)
            sizes1 = sizes1[idx:]
            measurements1 = {n: m[idx:] for n, m in measurements1.items()}
        if sizes[: len(sizes1)] != sizes1[: len(sizes)]:
            raise ImportError(f"{sizes} != {sizes1} for {langlib.Language} {orderstr}")
        if len(sizes) < len(sizes1):
            sizes = sizes1
        measurements.update(measurements1)

    publish_measurements(
        basename=basename,
        orderstr=orderstr,
        sizes=sizes,
        suffix=f" {single_version_suffix}" if single_version_suffix else "",
        measurement_per_size_by_case=measurements,
        label_by_case=label_by_case,
        color_by_case=color_by_language,
        linestyle_by_case=lambda case: linestyle_per_version[versions.index(case.Ver)],
    )


def publish_version_report(
    basebasename: str, orderstr: str, langlib: LangLib, versions: Sequence[str]
) -> None:
    sizes, measurements = read_csv(
        language=langlib.Language,
        orderstr=orderstr,
        case_selector=lambda case: case.LangLib == langlib and case.Ver in versions,
    )
    publish_measurements(
        basename=basebasename + f"_{langlib.Language.name}_{orderstr}",
        language=langlib.Language,
        orderstr=orderstr,
        sizes=sizes,
        measurement_per_size_by_case=measurements,
        label_by_case=lambda case: case.Ver,
    )


def publish_library_report(
    basename: str, orderstr: str, language: Language, ver: str, libs: Sequence[str]
) -> None:
    sizes, measurements = read_csv(
        language=language,
        orderstr=orderstr,
        case_selector=lambda case: case.LangLib.Lib in libs and case.Ver == ver,
    )
    publish_measurements(
        basename=basename,
        language=language,
        orderstr=orderstr,
        sizes=sizes,
        suffix=f" {ver}",
        measurement_per_size_by_case=measurements,
        label_by_case=lambda case: cast(str, case.LangLib.Lib),
    )


def publish_langver_report(
    basename: str,
    orderstr: str,
    ver: str,
    language: Language,
    languages: list[Language],
) -> None:
    sizes: List[int] = []
    measurements: Dict[Case, List[Measurement]] = {}
    for language in languages:
        sizes1, measurements1 = read_csv(
            language=language,
            orderstr=orderstr,
            case_selector=lambda case: case.Ver == ver,
        )
        assert not sizes or sizes1 == sizes
        sizes = sizes1
        measurements.update(measurements1)
    publish_measurements(
        basename=basename,
        language=language,
        orderstr=orderstr,
        sizes=sizes,
        suffix=f" {ver}",
        measurement_per_size_by_case=measurements,
        label_by_case=lambda case: case.LangLib.Language.long_name(),
    )


def publish_reports() -> None:
    # 1. Ver1 vs. Ver1½
    publish_report(
        basename="report_1",
        orderstr="100",
        langlibs=[LangLib(Language.python313), LangLib(Language.rust, "Hash")],
        versions=["Ver1", "Ver1½"],
        linestyle_per_version=["dotted", None],
        label_by_case=lambda case: f"{case.LangLib.Language.short_name()} {case.Ver}",
    )
    # 2. Ver1 vs. Ver2
    publish_report(
        basename="report_2",
        orderstr="100",
        langlibs=[
            LangLib(Language.java),
            LangLib(Language.rust, "Hash"),
        ],
        versions=["Ver1½", "Ver2½"],
        linestyle_per_version=["dotted", None],
        label_by_case=lambda case: f"{case.LangLib.Language.short_name()} {case.Ver}",
    )
    # 3. Ver2 variants
    for orderstr in ["100", "10k"]:
        for langlib in [LangLib(Language.rust, "Hash"), LangLib(Language.java)]:
            publish_version_report(
                basebasename="report_3",
                orderstr=orderstr,
                langlib=langlib,
                versions=["Ver2½", "Ver2½-RP", "Ver2½-GP", "Ver2½-GPX"],
            )
    # 4. Ver2 vs. Ver3
    for orderstr in ["10k", "1M"]:
        for langlib in [
            LangLib(Language.python313),
            LangLib(Language.csharp, "HashSet"),
        ]:
            publish_version_report(
                basebasename="report_4",
                orderstr=orderstr,
                langlib=langlib,
                versions=["Ver2½-GP", "Ver3½-GP"],
            )
    for orderstr in ["10k"]:
        for langlib in [LangLib(Language.rust, "Hash"), LangLib(Language.java)]:
            publish_version_report(
                basebasename="report_4",
                orderstr=orderstr,
                langlib=langlib,
                versions=["Ver2½-GP", "Ver3½-GP"],
            )
    # 5. Ver3 variants
    for orderstr in ["10k", "1M"]:
        for langlib in [
            LangLib(Language.python313),
            LangLib(Language.csharp, "HashSet"),
        ]:
            publish_version_report(
                basebasename="report_5",
                orderstr=orderstr,
                langlib=langlib,
                versions=["Ver3½-GP", "Ver3½-GPX"],
            )
    for orderstr in ["10k"]:
        for langlib in [LangLib(Language.rust, "Hash"), LangLib(Language.java)]:
            publish_version_report(
                basebasename="report_5",
                orderstr=orderstr,
                langlib=langlib,
                versions=["Ver3½-GP", "Ver3½-GPX"],
            )
    # 6. Parallelism
    for orderstr in ["100", "10k", "1M"]:
        publish_version_report(
            basebasename="report_6",
            orderstr=orderstr,
            langlib=LangLib(Language.java),
            versions=["Ver3½-GP", "Ver3½=GPs", "Ver3½=GPc"],
        )
        publish_version_report(
            basebasename="report_6",
            orderstr=orderstr,
            langlib=LangLib(Language.go),
            versions=[
                "Ver3½-GP",
                "Ver3½=GP0",
                "Ver3½=GP1",
                "Ver3½=GP2",
                "Ver3½=GP3",
                "Ver3½=GP4",
            ],
        )
    # 7. Languages
    for orderstr in ["100", "10k", "1M"]:
        publish_report(
            basename=f"report_7_sequential_{orderstr}",
            orderstr=orderstr,
            langlibs=[
                LangLib(Language.python313),
                LangLib(Language.kotlin),
                LangLib(Language.go),
                LangLib(Language.java),
                LangLib(Language.cpp, "hashset"),
                LangLib(Language.csharp, "HashSet"),
                LangLib(Language.rust, "Hash"),
            ],
            versions=["Ver3½-GP"],
            linestyle_per_version=[None],
            label_by_case=lambda case: case.LangLib.Language.short_name(),
            single_version_suffix="Ver3½-GP",
        )
        publish_report(
            basename=f"report_7_channels_{orderstr}",
            orderstr=orderstr,
            langlibs=[
                LangLib(Language.java),
                LangLib(Language.kotlin),
                LangLib(Language.go),
                LangLib(Language.csharp, "HashSet"),
                LangLib(Language.cpp, "hashset"),
                LangLib(Language.rust, "Hash"),
            ],
            versions=["Ver3½=GPc", "Ver3½=GP3"],
            linestyle_per_version=[None, None],
            label_by_case=lambda case: case.LangLib.Language.short_name(),
            single_version_suffix="parallel Ver3½=GP using channels",
        )
    # 8. Libraries
    for orderstr in ["100", "10k", "1M"]:
        publish_library_report(
            basename=f"report_8_rust_{orderstr}",
            orderstr=orderstr,
            language=Language.rust,
            ver="Ver3½-GP",
            libs=["BTree", "Hash", "hashbrown", "fnv", "ord_vec"],
        )
    for orderstr in ["100", "10k"]:
        publish_library_report(
            basename=f"report_8_csharp_{orderstr}",
            orderstr=orderstr,
            language=Language.csharp,
            ver="Ver3½-GP",
            libs=["HashSet", "SortedSet"],
        )
        publish_library_report(
            basename=f"report_8_cpp_{orderstr}",
            orderstr=orderstr,
            language=Language.cpp,
            ver="Ver3½-GP",
            libs=["hashset", "std_set", "ord_vec"],
        )
    # 9. Language versions
    for orderstr in ["100", "10k", "1M"]:
        publish_langver_report(
            basename=f"report_9_python_{orderstr}",
            orderstr=orderstr,
            language=Language.python,
            languages=[Language.python310, Language.python311, Language.python313],
            ver="Ver3½-GP",
        )


if __name__ == "__main__":
    if len(sys.argv) == 1:
        publish_reports()
    else:
        language = Language[sys.argv[1]]
        for orderstr in sys.argv[2:]:
            publish_whole_csv(language=language, orderstr=orderstr)
