#!/bin/zsh

RATE_DATA_PATH=$(dirname "$0")/ratings.csv
RATE_FILLED_CHAR="●"
RATE_EMPTY_CHAR="◯"

rate() {
    # Create ratings csv file if it doesn't exist
    if ! [[ -f $RATE_DATA_PATH ]]; then
        touch "$RATE_DATA_PATH"
        echo "date,rating" >>"$RATE_DATA_PATH"
    fi

    line_num=$(wc -l <"$RATE_DATA_PATH" | sed 's/ //g')

    # Output rating "calendar"
    if [[ $# -eq 0 ]]; then
        # CSV only has header
        # Temporarily add a new line to the end of the file
        # Needed for the script to work properly
        initial=false
        if [[ $line_num -eq 1 ]]; then
            initial=true
            echo >>"$RATE_DATA_PATH"
        fi

        # Starting date is the first Monday more than three weeks before today
        # This will ensure the output is always four rows
        cur_date=$(date -v -27d +"%Y-%m-%d")
        while [[ $(date -j -f "%Y-%m-%d" "$cur_date" +"%w") != 1 ]]; do
            cur_date=$(date -j -v +1d -f "%Y-%m-%d" "$cur_date" +"%Y-%m-%d")
        done

        no_color='\033[0m'
        week="\n"

        last_date=$(tail -1 "$RATE_DATA_PATH" | cut -d "," -f1)
        tomorrow=$(date -v +1d +%Y-%m-%d)

        while IFS="," read -r date rating; do
            while ! [[ $cur_date > $date && $date != "$last_date" ]] &&
                ! [[ $cur_date = "$tomorrow" ]]; do
                if [[ $cur_date = "$date" ]]; then
                    char=$RATE_FILLED_CHAR
                    # Choose color based on rating
                    if [[ "$rating" -gt 6 && "$rating" -le 11 ]]; then
                        color='\033[0;32m'
                    elif [[ "$rating" -gt 3 && "$rating" -le 7 ]]; then
                        color='\033[0;33m'
                    elif [[ "$rating" -gt 0 && "$rating" -le 4 ]]; then
                        color='\033[0;31m'
                    else
                        echo "Invalid rating: $rating" >&2
                        return 1
                    fi
                # Date does not have a rating
                else
                    color=$no_color
                    char=$RATE_EMPTY_CHAR
                fi
                # Add day to week output
                week+=" ${color}${char}${no_color} "
                # Print the week
                if [ "$(date -j -f "%Y-%m-%d" "$cur_date" +"%w")" -eq 0 ]; then
                    echo -e "${week}\n"
                    week=""
                fi
                # Increment date
                cur_date=$(date -j -v +1d -f "%Y-%m-%d" "$cur_date" +"%Y-%m-%d")
            done
        done < <(tail -n +2 <"$RATE_DATA_PATH" | tail -28)

        # Print the last week
        if [[ $(date -j -f "%Y-%m-%d" "$cur_date" +"%w") != 1 ]]; then
            echo -e "${week}\n"
        fi

        # Remove the temporary line
        if [[ $initial = true ]]; then
            sed -i '$ d' $RATE_DATA_PATH
        fi

    # Edit ratings data
    else
        if [[ $# -le 4 ]]; then
            date=false
            rating=false
            remove=false

            while [[ $# -gt 0 ]]; do
                # Get flags and arguments
                while getopts "d:rh" opt; do
                    case $opt in
                    h)
                        echo
                        echo "Usage: $0 [options] <rating>" >&2
                        echo
                        echo "Use script without arguments to see the current ratings"
                        echo
                        echo "   -r            Remove entry for specified date"
                        echo "   -d            Specify date, defaults to current date"
                        echo "   -h            Show help"
                        echo
                        echo "   <rating>      Rating of date, must be from 1 to 10"
                        echo
                        return 0
                        ;;
                    r)
                        if [[ $rating != false ]]; then
                            echo "Invalid arguments: cannot use -r with rating" >&2
                            return 1
                        fi
                        remove=true
                        ;;
                    d)
                        date=$OPTARG
                        # Validate date
                        if ! date -f "%Y-%m-%d" -j "$date" >/dev/null 2>&1; then
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

                # Get rating
                if [[ $# -gt 0 ]]; then
                    if [[ $1 -gt 0 && $1 -le 11 ]]; then
                        if [[ $rating != false ]]; then
                            echo "Rating set twice: $1" >&2
                            return 1
                        elif [[ $remove = true ]]; then
                            echo "Invalid arguments: cannot use -r with rating" >&2
                            return 1
                        else
                            rating=$1
                            shift
                        fi
                    else
                        echo "Invalid rating: $rating" >&2
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

        # Backup rating data
        \cp "$RATE_DATA_PATH" "$RATE_DATA_PATH.bak"

        if [[ $remove = true ]]; then
            sed -i "/$date/d" "$RATE_DATA_PATH"
            return 0
        fi

        # Find which line to insert/update
        if [[ $line_num -gt 1 ]]; then
            while read -r line; do
                cur_date=$(cut -d "," -f1 <<<"$line")
                # Found exact match, replace entire line
                if [[ $date = "$cur_date" ]]; then
                    echo "Date already rated: $date"
                    if read -rq "REPLY?Do you want to override? (y/n) "; then
                        sed -i "$line_num""s/.*/$date,$rating/" "$RATE_DATA_PATH"
                    fi
                    echo
                    return 0
                elif [[ $date > $cur_date ]]; then
                    break
                fi
                line_num=$((line_num - 1))
            done < <(tail -n +2 "$RATE_DATA_PATH" | tac)
        fi

        sed -i "$line_num"'a\'"$date"','"$rating" "$RATE_DATA_PATH"
    fi
}
