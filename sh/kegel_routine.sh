#!/bin/bash

set -e

START=$SECONDS
CLEAR_LINE="\e[K\r"
GREEN="\e[32m"
CYAN="\e[96m"
RED="\e[31m"
DEFAULT="\e[39m"

function exercise_countdown() {
    for i in {1..60}; do
        if [[ $(bc <<< "$i % 2") != 0 ]]; then
            echo -en "\n${DEFAULT}KEGEL\n"
        else
            echo -en "\n${DEFAULT}REVERSE KEGEL\n"
        fi

        echo -en "${GREEN}TIME LEFT FOR EXERCISE:\n"

        for j in {30..1}; do
            echo -en "${CLEAR_LINE}${CYAN}$j seconds ${DEFAULT} | \
${RED}$(elapsed_time $START)${CLEAR_LINE}"
            sleep 1
        done

        relax_countdown
    done
}

function relax_countdown() {
    echo -en "\n${GREEN}TIME TO RELAX BEFORE EXERCISE:\n"

    for i in {15..1}; do
        echo -en "${CLEAR_LINE}${CYAN}$i seconds ${DEFAULT} | \
${RED}$(elapsed_time $START)${CLEAR_LINE}"
        sleep 1
    done
}

function elapsed_time() {
    ELAPSED=$((SECONDS - $1))
    # 2700 = 45 minutes in seconds
    # n/27 = n/2700 * 100 (percentage)
    PERCENTAGE=$(bc <<< "scale=1; $ELAPSED/27")

    echo -en "$(date -ud "@$ELAPSED" +'%M:%S') - 45:00 ($PERCENTAGE% TOTAL)\n"
}

exercise_countdown

exit 0
