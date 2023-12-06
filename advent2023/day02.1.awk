
BEGIN { FS="[: ]" }

/[2-9][0-9] / {next}
/1[3-9] red/ {next}
/1[4-9] green/ {next}
/1[5-9] blue/ {next}

{s+=$2}

END {print "Part 1:", s}
