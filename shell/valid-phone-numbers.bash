# Read from the file file.txt and output all valid phone numbers to stdout.

IFS=$'\n'
list=($(cat file.txt))


for phone in ${list[@]}; do
    if [[ ${phone} =~ ^([0-9]{3})-([0-9]{3})-([0-9]{4})$ ]]; then
        echo $phone
    elif [[ ${phone} =~ ^\(([0-9]{3})\)\ ([0-9]{3})-([0-9]{4})$ ]]; then
        echo $phone
    fi
done
