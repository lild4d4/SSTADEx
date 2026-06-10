#!/usr/bin/env python3

import argparse
import re
from pathlib import Path


def parse_args():
    parser = argparse.ArgumentParser(
        description=(
            "Read a CSV file, filter it, extract unique values, "
            "get column extrema, or plot two columns."
        )
    )

    parser.add_argument(
        "csv_file",
        type=Path,
        help="Path to the input CSV file",
    )

    parser.add_argument(
        "-o",
        "--output",
        type=Path,
        default=Path("output.csv"),
        help="Path to save the output file",
    )

    parser.add_argument(
        "--column",
        type=str,
        help="Column name to filter or extract unique values from",
    )

    parser.add_argument(
        "--min",
        type=float,
        help="Minimum value for numeric filtering",
    )

    parser.add_argument(
        "--max",
        type=float,
        help="Maximum value for numeric filtering",
    )

    parser.add_argument(
        "--equals",
        type=str,
        help="Exact value to filter by",
    )

    parser.add_argument(
        "--unique",
        action="store_true",
        help="Extract unique values from the selected column",
    )

    parser.add_argument(
        "--minimum",
        action="store_true",
        help="Get the minimum numeric value from the selected column",
    )

    parser.add_argument(
        "--maximum",
        action="store_true",
        help="Get the maximum numeric value from the selected column",
    )

    parser.add_argument(
        "--gen-column",
        type=str,
        help="Name of a new column to generate from an equation",
    )

    parser.add_argument(
        "--eq",
        type=str,
        help="Equation used with --gen-column. Reference columns as $column_name.",
    )

    parser.add_argument(
        "--plot",
        action="store_true",
        help="Generate a plot using two columns",
    )

    parser.add_argument(
        "--x-column",
        type=str,
        help="Column to use as x-axis",
    )

    parser.add_argument(
        "--y-column",
        type=str,
        help="Column to use as y-axis",
    )

    parser.add_argument(
        "--plot-type",
        choices=["line", "scatter"],
        default="scatter",
        help="Type of plot to generate",
    )

    parser.add_argument(
        "--x-log",
        action="store_true",
        help="Use a logarithmic x-axis when plotting",
    )

    parser.add_argument(
        "--y-log",
        action="store_true",
        help="Use a logarithmic y-axis when plotting",
    )

    return parser.parse_args()


def check_column_exists(df, column):
    if column not in df.columns:
        raise ValueError(f"Column '{column}' not found in CSV.")


def filter_dataframe(df, column=None, min_value=None, max_value=None, equals=None):
    filtered_df = df.copy()

    if column is None:
        return filtered_df

    check_column_exists(filtered_df, column)

    if equals is not None:
        filtered_df = filtered_df[filtered_df[column].astype(str) == equals]

    if min_value is not None:
        filtered_df = filtered_df[filtered_df[column] >= min_value]

    if max_value is not None:
        filtered_df = filtered_df[filtered_df[column] <= max_value]

    return filtered_df


def extract_unique_values(df, column):
    import pandas as pd

    if column is None:
        raise ValueError("You must provide --column when using --unique")

    check_column_exists(df, column)

    unique_values = sorted(df[column].dropna().unique())
    unique_df = pd.DataFrame({column: unique_values})

    return unique_df


def extract_extrema(df, column, include_minimum=False, include_maximum=False):
    import pandas as pd

    if not include_minimum and not include_maximum:
        return None

    if column is None:
        raise ValueError("You must provide --column when using --minimum or --maximum")

    check_column_exists(df, column)

    numeric_values = pd.to_numeric(df[column], errors="coerce").dropna()
    if numeric_values.empty:
        raise ValueError(f"Column '{column}' does not contain numeric values.")

    extrema = []
    if include_minimum:
        extrema.append(
            {"column": column, "statistic": "minimum", "value": numeric_values.min()}
        )
    if include_maximum:
        extrema.append(
            {"column": column, "statistic": "maximum", "value": numeric_values.max()}
        )

    return pd.DataFrame(extrema)


def generate_column(df, column, equation):
    import numpy as np

    if column is None and equation is None:
        return df

    if column is None or equation is None:
        raise ValueError("You must provide both --gen-column and --eq")

    generated_df = df.copy()
    referenced_columns = set(re.findall(r"\$([A-Za-z_][A-Za-z0-9_]*)", equation))

    for referenced_column in referenced_columns:
        check_column_exists(generated_df, referenced_column)

    python_expression = re.sub(
        r"\$([A-Za-z_][A-Za-z0-9_]*)",
        r'generated_df["\1"]',
        equation,
    )

    allowed_globals = {"__builtins__": {}, "np": np}
    allowed_locals = {"generated_df": generated_df}

    try:
        generated_df[column] = eval(python_expression, allowed_globals, allowed_locals)
    except Exception as exc:
        raise ValueError(f"Could not evaluate equation '{equation}': {exc}") from exc

    return generated_df


def check_positive_for_log_scale(df, column):
    if (df[column] <= 0).any():
        raise ValueError(
            f"Column '{column}' contains zero or negative values, "
            "which cannot be plotted on a log scale."
        )


def plot_columns(
    df,
    x_column,
    y_column,
    output_path,
    plot_type="scatter",
    x_log=False,
    y_log=False,
):
    import matplotlib.pyplot as plt

    if x_column is None or y_column is None:
        raise ValueError("You must provide --x-column and --y-column when using --plot")

    check_column_exists(df, x_column)
    check_column_exists(df, y_column)

    if x_log:
        check_positive_for_log_scale(df, x_column)

    if y_log:
        check_positive_for_log_scale(df, y_column)

    output_path.parent.mkdir(parents=True, exist_ok=True)

    plt.figure()

    if plot_type == "scatter":
        plt.scatter(df[x_column], df[y_column])
    elif plot_type == "line":
        plt.plot(df[x_column], df[y_column])

    plt.xlabel(x_column)
    plt.ylabel(y_column)
    plt.title(f"{y_column} vs {x_column}")
    if x_log:
        plt.xscale("log")
    if y_log:
        plt.yscale("log")
    plt.grid(True)
    plt.tight_layout()

    plt.savefig(output_path, dpi=300)
    plt.close()


def main():
    args = parse_args()

    if not args.csv_file.exists():
        raise FileNotFoundError(f"File not found: {args.csv_file}")

    import pandas as pd

    df = pd.read_csv(args.csv_file)
    df = generate_column(df, args.gen_column, args.eq)

    if args.unique:
        output_df = extract_unique_values(df, args.column)

        args.output.parent.mkdir(parents=True, exist_ok=True)
        output_df.to_csv(args.output, index=False)

        print(f"Input file: {args.csv_file}")
        print(f"Original rows: {len(df)}")
        print(f"Unique values: {len(output_df)}")
        print(f"Saved unique values to: {args.output}")

    elif args.minimum or args.maximum:
        output_df = extract_extrema(
            df,
            args.column,
            include_minimum=args.minimum,
            include_maximum=args.maximum,
        )

        args.output.parent.mkdir(parents=True, exist_ok=True)
        output_df.to_csv(args.output, index=False)

        print(f"Input file: {args.csv_file}")
        print(f"Original rows: {len(df)}")
        for _, row in output_df.iterrows():
            print(f"{row['statistic'].capitalize()} {row['column']}: {row['value']}")
        print(f"Saved extrema to: {args.output}")

    elif args.plot:
        plot_columns(
            df,
            x_column=args.x_column,
            y_column=args.y_column,
            output_path=args.output,
            plot_type=args.plot_type,
            x_log=args.x_log,
            y_log=args.y_log,
        )

        print(f"Input file: {args.csv_file}")
        print(f"Saved plot to: {args.output}")

    else:
        output_df = filter_dataframe(
            df,
            column=args.column,
            min_value=args.min,
            max_value=args.max,
            equals=args.equals,
        )

        args.output.parent.mkdir(parents=True, exist_ok=True)
        output_df.to_csv(args.output, index=False)

        print(f"Input file: {args.csv_file}")
        print(f"Original rows: {len(df)}")
        print(f"Filtered rows: {len(output_df)}")
        print(f"Saved filtered CSV to: {args.output}")


if __name__ == "__main__":
    main()
