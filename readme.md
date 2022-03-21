<h3 align="center">
  <code>Feeling</code>
</h3>
<p align="center">A Zsh script for your <i>feelings</i>
</p>
<p align="center">
  <img width="700" src="feeling.svg" />
</p>
<p align="center">
  <sub>Demo made with
    <code> <a href="https://github.com/marionebl/svg-term-cli">svg-term-cli</a></code>
      using my
    <code>
      <a href="https://github.com/qiz-li/dotfiles">
        <b>dotfiles</b>
      </a>
    </code>
  </sub>
</p>

## Overview

The script is inspired by this Reddit [post](https://github.com/qiz-li/dotfiles).
The idea is to enter your feeling each day on a 1-10 scale.
This gives time to reflect upon your day to create more meaningful goals and improve wellness.
By displaying your feelings in a beautiful calendar,
you can also gain insight into trends in your feelings while prettifying your terminal.

## Installation

> Note for **macOS** users.
> As the script makes use of GNU versions of tools,
> please link [`gsed`](https://formulae.brew.sh/formula/gnu-sed) to `sed` and `gdate` to `date`.

### [Oh My Zsh](http://ohmyz.sh)

Clone this repository into the custom plugins folder:

```shell
git clone https://github.com/qiz-li/feeling.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/feeling
```

Update Oh My Zsh plugins list in `.zshrc`:

```shell
plugins=(
    # Other plugins...
    feeling
)
```

### [Antigen](https://github.com/zsh-users/antigen)

Add the following to your `.zshrc`:

```shell
antigen bundle qiz-li/feeling@main
```

## Configuration

#### `FEELING_DATA_PATH`

Path to file in which your feelings and corresponding dates will be stored.
Defaults to `~/.config/feeling/feelings.csv`.

#### `FEELING_FILLED_CHAR`

Character to represent days you have rated.
Defaults to `●`.

#### `FEELING_EMPTY_CHAR`

Character to represent days that are unrated.
Defaults to `◯`.

## Usage

Use the `-h` flag for help.

### Show Calendar

Run without any arguments:

```shell
feeling
```

### Add/Change Feeling

Add your feeling as an argument.

To add/change today's feeling:

```shell
feeling 8
```

Or specify a date using the `-d` flag:

```shell
feeling -d 2022-02-22 3
```

### Remove Feeling

Add the `-r` flag to remove a feeling.

To remove today's feeling:

```shell
feeling -r
```

Or a specific date:

```shell
feeling -d 2022-03-20 -r
```
