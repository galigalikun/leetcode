# Read from the file file.txt and print its transposed content to stdout.
IFS=$'\n'
list=($(cat file.txt))

declare -a array=()

for line in ${list[@]}; do
    IFS=" "
    data=($(echo $line))
    for ((i = 0; i < ${#data[@]}; i++)) {
        array[i]+="${data[i]}_"
    }
done

for result in ${array[@]}; do
    echo "${result//_/ }" | xargs
done

