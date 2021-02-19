# Read from the file file.txt and output the tenth line to stdout.

cat file.txt | sed '10,1!d'
