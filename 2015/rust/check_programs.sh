#!/bin/bash

RED='\e[0;31m'
GREEN='\e[0;32m'
RESET='\e[m'

if [ $# -eq 0 ]; then
    dirs=$(find -maxdepth 1 -type d -exec basename {} \; | sort)
else
    if [ $1 == "--help" ] || [ $1 == "-h" ] || [ $1 == "help" ]; then
        echo -e "Usage:\n$0 (Checks every project)"
        echo "or:"
        echo "$0 01 07 17 (checks only days 01, 07, and 17; in that order)"
        exit 0
    else
        dirs=$@
    fi
fi

# yes, I know, default-to-failure would be better but i'm too lazy :(
error=false

for dir in $dirs; do
    # skip . and 00/template.rs
    if [ "$dir" == "." ] || [ "$dir" == "00" ]; then
        continue
    fi

    pushd $dir &> /dev/null

    ls Cargo.toml &> /dev/null
    if [ $? -eq 0 ]; then
        cargo build --release &> /dev/null
        if [ $? -eq 0 ]; then
            echo "Day $dir: Cargo build successful. Running programs now."
            cargo run --release &> /dev/null
            if [ $? -eq 0 ]; then
                echo -e "${GREEN}Day $dir: Successful${RESET}"
            else
                echo -e "${RED}Day $dir: Failed!${RESET}"
                error=true
            fi
        else
            echo -e "${RED}Day $dir: Cargo build failed!${RESET}"
            error=true
        fi
    else
        echo "Day $dir: Compiling"
        compile1=0
        compile2=0
        if [ -f part1.rs ]; then
            rustc -O part1.rs &> /dev/null
            compile1=$?
        fi

        if [ -f part2.rs ]; then
            rustc -O part2.rs &> /dev/null
            compile2=$?
        fi

        if [ $compile1 -eq 0 ] && [ $compile2 -eq 0 ]; then
            echo "Day $dir: Build successful. Running programs now."
            run1=0
            run2=0
            if [ -f part1 ]; then
                ./part1 &> /dev/null
                run1=$?
            fi

            if [ -f part2 ]; then
                ./part2 &> /dev/null
                run2=$?
            fi

            if [ $run1 -eq 0 ] && [ $run2 -eq 0 ]; then
                echo -e "${GREEN}Day $dir: Successful${RESET}"
            else
                echo -e "${RED}Day $dir: Failed!${RESET}"
                error=true
            fi
        else
            echo -e "${RED}Day $dir: Build failed! $compile2 $run2 ${RESET}"
            error=true
        fi
    fi

    popd &> /dev/null

done

echo -e "\n"

if [ $error == false ]; then
    echo -e "\t${GREEN}Everything was successful!${RESET}"
else
    echo -e "\t${RED}There were errors!${RESET}"
fi

echo ""
