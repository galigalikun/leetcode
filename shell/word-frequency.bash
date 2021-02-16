# Read from the file words.txt and output the word frequency list to stdout.

woards=$(cat words.txt)
IFS=" "
list=(${woards})

IFS=$'\n'
s_list=($(echo "${list[*]}" | sort))
cnt=1
p_line=""
result=()
for line in ${s_list[@]}; do
    if [ "$p_line" = "$line" ]; then
      let cnt++
    elif [ "$p_line" != "" ]; then
      result+=($(echo "${cnt} ${p_line}"))
      cnt=1
    fi
    p_line=$line
done
result+=($(echo "${cnt} ${p_line}"))

s_result=($(echo "${result[*]}" | sort -r -n))

for res in ${s_result[@]}; do
  IFS=" "
  ret=(${res})
  echo "${ret[1]} ${ret[0]}"
done
