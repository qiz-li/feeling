#!/bin/zsh

FEELING_DATA_PATH="$HOME/.config/feeling/feelings.csv"
FEELING_FILLED_CHAR="●"
FEELING_EMPTY_CHAR="◯"

feeling() {
    # Create feelings csv file if it doesn't exist
    if ! [[ -f $FEELING_DATA_PATH ]]; then
        mkdir -p "${FEELING_DATA_PATH%/*}" && touch "$FEELING_DATA_PATH"
        echo "date,feeling" >>"$FEELING_DATA_PATH"
    fi

    line_num=$(wc -l <"$FEELING_DATA_PATH" | sed 's/ //g')

    # Output feeling "calendar"
    if [[ $# -eq 0 ]]; then
        # CSV only has header
        # Temporarily add a new line to the end of the file
        # Needed for the script to work properly
        initial=false
        if [[ $line_num -eq 1 ]]; then
            initial=true
            echo >>"$FEELING_DATA_PATH"
        fi

        # Starting date is the first Monday more than three weeks before today
        # This will ensure the output is always four rows
        cur_date=$(date -d '-27 day' +%Y-%m-%d)
        while [[ $(date -d "$cur_date" +%w) != 1 ]]; do
            cur_date=$(date -d "$cur_date"'+1 day' +%Y-%m-%d)
        done

        no_color='\033[0m'
        week="\n"

        last_date=$(tail -1 "$FEELING_DATA_PATH" | cut -d "," -f1)
        tomorrow=$(date -d '+1 day' +%Y-%m-%d)

        while IFS="," read -r date feeling; do
            while ! [[ $cur_date > $date && $date != "$last_date" ]] &&
                ! [[ $cur_date = "$tomorrow" ]]; do
                if [[ $cur_date = "$date" ]]; then
                    char=$FEELING_FILLED_CHAR
                    # Choose color based on feeling
                    if [[ "$feeling" -ge 7 && "$feeling" -le 10 ]]; then
                        color='\033[0;32m'
                    elif [[ "$feeling" -ge 4 && "$feeling" -le 6 ]]; then
                        color='\033[0;33m'
                    elif [[ "$feeling" -ge 0 && "$feeling" -le 3 ]]; then
                        color='\033[0;31m'
                    else
                        echo "Invalid feeling: $feeling" >&2
                        return 1
                    fi
                # Date does not have a feeling
                else
                    color=$no_color
                    char=$FEELING_EMPTY_CHAR
                fi
                # Add day to week output
                week+=" ${color}${char}${no_color} "
                # Print the week
                if [ "$(date -d "$cur_date" +%w)" -eq 0 ]; then
                    echo -e "${week}\n"
                    week=""
                fi
                # Increment date
                cur_date=$(date -d "$cur_date"'+1 day' +%Y-%m-%d)
            done
        done < <(tail -n +2 <"$FEELING_DATA_PATH" | tail -28)

        # Print the last week
        if [[ $(date -d "$cur_date" +%w) != 1 ]]; then
            echo -e "${week}\n"
        fi

        # Remove the temporary line
        if [[ $initial = true ]]; then
            sed -i '$ d' "$FEELING_DATA_PATH"
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
                        # Validate date
                        if ! [[ $date =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]] ||
                            ! date -d "$date" >/dev/null 2>&1; then
                            echo "Invalid date: $date" >&2
                            return 1
                        fi
                        if [[ $date > $(date +%Y-%m-%d) ]]; then
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
            date=$(date +%Y-%m-%d)
        fi

        # Backup feeling data
        \cp "$FEELING_DATA_PATH" "$FEELING_DATA_PATH.bak"

        if [[ $remove = true ]]; then
            sed -i "/$date/d" "$FEELING_DATA_PATH"
            return 0
        fi

        # Find which line to insert/update
        if [[ $line_num -gt 1 ]]; then
            while read -r line; do
                cur_date=$(cut -d "," -f1 <<<"$line")
                # Found exact match, replace entire line
                if [[ $date = "$cur_date" ]]; then
                    echo "Date already has feeling: $date"
                    if read -rq "REPLY?Do you want to override? (y/n) "; then
                        sed -i "$line_num""s/.*/$date,$feeling/" "$FEELING_DATA_PATH"
                    fi
                    echo
                    return 0
                elif [[ $date > $cur_date ]]; then
                    break
                fi
                line_num=$((line_num - 1))
            done < <(tail -n +2 "$FEELING_DATA_PATH" | tac)
        fi

        sed -i "$line_num"'a\'"$date"','"$feeling" "$FEELING_DATA_PATH"
    fi
}
