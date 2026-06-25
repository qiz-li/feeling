<h3 align="center">
  <code>Feeling</code>
</h3>
<p align="center">A beautiful terminal mood tracker for your <i>feelings</i>
</p>
<p align="center">
  <img width="700" src="feeling.svg" />
</p>
<p align="center">
  <sub>Demo made with
    <code><a href="https://github.com/marionebl/svg-term-cli">svg-term-cli</a></code>
      using my
    <code><a href="https://github.com/qiz-li/dotfiles"><b>dotfiles</b></a></code>
  </sub>
</p>

## Overview

A simple, blazing-fast mood tracker that lives in your terminal. Research suggests that tracking your mood can improve emotional awareness and well-being by recognizing patterns over time (Kauer et al., 2012). Log how you feel each day on a 1–10 scale and watch the patterns emerge.

Data is stored locally as a plain CSV file.

## Installation

```shell
curl -sSf https://raw.githubusercontent.com/qiz-li/feeling/main/install.sh | sh
```

## Usage

```
feeling 8                     # log today's feeling (1-10)
feeling 6 -d 2024-03-15       # log a specific date
feeling                       # show default view (month)
feeling week                  # show current week
feeling month                 # show last 4 weeks
feeling year                  # full-year heatmap
feeling prompt                # single colored glyph (for starship/p10k)
feeling remove                # remove today's entry
feeling remove -d 2024-03-15  # remove a specific date
feeling export                # dump raw CSV to stdout
```

### CLI flags

| Flag | Description |
|---|---|
| `--data-path <path>` | Override data file location |
| `--view <week\|month\|year>` | Override default view |

### Prompt integration

Add to your `starship.toml`:

```toml
[custom.feeling]
command = "feeling prompt"
when = true
```

## Configuration

Create `~/.config/feeling/config.toml` (see [`config.example.toml`](config.example.toml)):

```toml
view = "month"           # default view: week, month, year
sunday_start = false     # start weeks on Sunday
# data_path = "~/path/to/feelings.csv"

[chars]
filled = "●"
empty = "◯"

[chars.year]
filled = "●"
empty = "·"
```

All options can also be set via environment variables.
> Precedence: **CLI flags > env vars > config file > defaults**.


| Variable | Description |
|---|---|
| `FEELING_DATA_PATH` | Override data file location |
| `FEELING_CONFIG_PATH` | Override config file location |
| `FEELING_VIEW` | Default view (`week`, `month`, `year`) |
| `FEELING_SUNDAY_START` | `1` or `true` for Sunday start |
| `FEELING_FILLED_CHAR` | Filled character |
| `FEELING_EMPTY_CHAR` | Empty character |
| `FEELING_YEAR_FILLED_CHAR` | Filled character for year heatmap |
| `FEELING_YEAR_EMPTY_CHAR` | Empty character for year heatmap |

## Data

Stored as plain CSV at `$XDG_DATA_HOME/feeling/feeling.csv` (defaults to `~/.local/share/feeling/feeling.csv`):

```
date,feeling
2024-03-15,7
2024-03-16,4
```

Atomic writes, file locking, rotating backups, and sha256 integrity checks to keep data safe.
