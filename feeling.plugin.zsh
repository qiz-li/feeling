#!/bin/zsh

zmodload zsh/datetime

FEELING_DATA_PATH="${FEELING_DATA_PATH:-$HOME/.config/feeling/feelings.csv}"
FEELING_FILLED_CHAR="${FEELING_FILLED_CHAR:-●}"
FEELING_EMPTY_CHAR="${FEELING_EMPTY_CHAR:-◯}"

# Atomic write - write to temp file, then rename (atomic on POSIX)
_feeling_write() {
    local tmp="$FEELING_DATA_PATH.tmp.$$"
    printf '%s\n' "$@" > "$tmp" && \mv -f "$tmp" "$FEELING_DATA_PATH"
}

# Rotate backups (keep last 3)
_feeling_backup() {
    [[ -f "$FEELING_DATA_PATH" ]] || return 0
    [[ -f "$FEELING_DATA_PATH.bak.2" ]] && \rm -f "$FEELING_DATA_PATH.bak.2"
    [[ -f "$FEELING_DATA_PATH.bak.1" ]] && \mv -f "$FEELING_DATA_PATH.bak.1" "$FEELING_DATA_PATH.bak.2"
    [[ -f "$FEELING_DATA_PATH.bak" ]] && \mv -f "$FEELING_DATA_PATH.bak" "$FEELING_DATA_PATH.bak.1"
    \cp "$FEELING_DATA_PATH" "$FEELING_DATA_PATH.bak"
}

feeling() {
    # Create feelings csv file if it doesn't exist
    if ! [[ -f $FEELING_DATA_PATH ]]; then
        mkdir -p "${FEELING_DATA_PATH%/*}" && touch "$FEELING_DATA_PATH"
        echo "date,feeling" >>"$FEELING_DATA_PATH"
    fi

    # Cache file contents in array (with validation)
    local -a _lines=()
    local _line
    while IFS= read -r _line || [[ -n $_line ]]; do
        # Only keep valid lines: header or date,feeling format
        if [[ $_line == "date,feeling" ]] || [[ $_line =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2},[0-9]+$ ]]; then
            _lines+=("$_line")
        fi
    done < "$FEELING_DATA_PATH"
    local line_num=${#_lines}

    # Output feeling "calendar"
    if [[ $# -eq 0 ]]; then
        # Get data lines (skip header), last 28 entries
        local -a data_lines=("${_lines[@]:1}")
        local -a recent_entries=("${data_lines[@]: -28}")

        # Get last date from data
        local last_date=""
        if [[ ${#data_lines} -gt 0 && -n "${data_lines[-1]}" ]]; then
            last_date="${data_lines[-1]%%,*}"
        fi

        # Calculate start epoch: 27 days ago, find next Monday
        local start_epoch=$((EPOCHSECONDS - 86400 * 27))
        local dow
        strftime -s dow '%w' $start_epoch
        # dow: 0=Sun, 1=Mon, ..., 6=Sat - find days until Monday
        local days_to_monday=$(( dow == 0 ? 1 : (dow == 1 ? 0 : 8 - dow) ))
        start_epoch=$((start_epoch + 86400 * days_to_monday))

        local tomorrow_epoch=$((EPOCHSECONDS + 86400))
        local tomorrow
        strftime -s tomorrow '%Y-%m-%d' $tomorrow_epoch

        local no_color=$'\033[0m'
        local week="\n"
        local cur_epoch=$start_epoch
        local cur_date entry_idx=1 entry_date entry_feeling
        local color char

        # Get first entry
        if [[ ${#recent_entries} -gt 0 && -n "${recent_entries[1]}" ]]; then
            entry_date="${recent_entries[1]%%,*}"
            entry_feeling="${recent_entries[1]#*,}"
        else
            entry_date=""
            entry_feeling=""
        fi

        while true; do
            strftime -s cur_date '%Y-%m-%d' $cur_epoch
            [[ $cur_date == "$tomorrow" ]] && break

            if [[ $cur_date == "$entry_date" ]]; then
                char=$FEELING_FILLED_CHAR
                # Choose color based on feeling
                if [[ "$entry_feeling" -ge 7 && "$entry_feeling" -le 10 ]]; then
                    color=$'\033[0;32m'
                elif [[ "$entry_feeling" -ge 4 && "$entry_feeling" -le 6 ]]; then
                    color=$'\033[0;33m'
                elif [[ "$entry_feeling" -ge 0 && "$entry_feeling" -le 3 ]]; then
                    color=$'\033[0;31m'
                else
                    echo "Invalid feeling: $entry_feeling" >&2
                    return 1
                fi
                # Move to next entry
                ((entry_idx++))
                if [[ $entry_idx -le ${#recent_entries} && -n "${recent_entries[$entry_idx]}" ]]; then
                    entry_date="${recent_entries[$entry_idx]%%,*}"
                    entry_feeling="${recent_entries[$entry_idx]#*,}"
                else
                    entry_date=""
                fi
            else
                color=$no_color
                char=$FEELING_EMPTY_CHAR
            fi

            # Add day to week output
            week+=" ${color}${char}${no_color} "

            # Print the week on Sunday
            strftime -s dow '%w' $cur_epoch
            if [[ $dow -eq 0 ]]; then
                print "${week}\n"
                week=""
            fi

            # Increment date
            cur_epoch=$((cur_epoch + 86400))
        done

        # Print the last week if not empty
        strftime -s dow '%w' $cur_epoch
        if [[ $dow -ne 1 ]]; then
            print "${week}\n"
        fi

    # Edit feelings data
    else
        if [[ $# -le 3 ]]; then
            date=false
            feeling=false
            remove=false

            while [[ $# -gt 0 ]]; do
                # Get flags and arguments
                while getopts "d:rh" opt; do
                    case $opt in
                    h)
                        echo
                        echo "Usage: $0 [options] <feeling>" >&2
                        echo
                        echo "Use script without arguments to see the current feelings"
                        echo
                        echo "   -r            Remove entry for specified date"
                        echo "   -d            Specify date, defaults to current date"
                        echo "   -h            Show help"
                        echo
                        echo "   <feeling>     How you felt on the date, must be from 1 to 10"
                        echo
                        return 0
                        ;;
                    r)
                        if [[ $feeling != false ]]; then
                            echo "Invalid arguments: cannot use -r with $feeling" >&2
                            return 1
                        fi
                        remove=true
                        ;;
                    d)
                        date=$OPTARG
                        # Validate date format
                        if ! [[ $date =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]]; then
                            echo "Invalid date: $date" >&2
                            return 1
                        fi
                        # Check date is not in the future
                        local today
                        strftime -s today '%Y-%m-%d' $EPOCHSECONDS
                        if [[ $date > $today ]]; then
                            echo "Date is in the future: $date" >&2
                            return 1
                        fi
                        ;;
                    \?)
                        echo "Invalid option: -$OPTARG" >&2
                        return 1
                        ;;
                    esac
                done
                shift $((OPTIND - 1))

                # Get feeling
                if [[ $# -gt 0 ]]; then
                    if [[ $1 -ge 1 && $1 -le 10 ]]; then
                        if [[ $feeling != false ]]; then
                            echo "Feeling set twice: $1" >&2
                            return 1
                        elif [[ $remove = true ]]; then
                            echo "Invalid arguments: cannot use -r with $1" >&2
                            return 1
                        else
                            feeling=$1
                            shift
                        fi
                    else
                        echo "Invalid feeling: $1" >&2
                        return 1
                    fi
                fi
            done

        else
            echo "Too many arguments: $*" >&2
            return 1
        fi

        # Date defaults to today
        if [[ $date = false ]]; then
            strftime -s date '%Y-%m-%d' $EPOCHSECONDS
        fi

        # Backup feeling data (rotating backups)
        _feeling_backup

        if [[ $remove = true ]]; then
            # Remove line matching date using array filtering
            local -a new_lines=("${_lines[1]}")
            local -a data_entries=("${_lines[@]:1}")
            local entry
            for entry in "${data_entries[@]}"; do
                [[ "${entry%%,*}" != "$date" ]] && new_lines+=("$entry")
            done
            _feeling_write "${new_lines[@]}"
            return 0
        fi

        # Find which line to insert/update
        if [[ $line_num -gt 1 ]]; then
            # Iterate through data lines in reverse order
            local -a data_lines=("${_lines[@]:1}")
            local -a reversed=("${(Oa)data_lines[@]}")
            local entry cur_date
            for entry in "${reversed[@]}"; do
                cur_date="${entry%%,*}"
                # Found exact match, replace entire line
                if [[ $date == "$cur_date" ]]; then
                    echo "Date already has feeling: $date"
                    if read -rq "REPLY?Do you want to override? (y/n) "; then
                        _lines[$line_num]="$date,$feeling"
                        _feeling_write "${_lines[@]}"
                    fi
                    echo
                    return 0
                elif [[ $date > $cur_date ]]; then
                    break
                fi
                line_num=$((line_num - 1))
            done
        fi

        # Insert new entry after line_num
        local -a new_lines=("${_lines[@]:0:$line_num}" "$date,$feeling" "${_lines[@]:$line_num}")
        _feeling_write "${new_lines[@]}"
    fi
}
