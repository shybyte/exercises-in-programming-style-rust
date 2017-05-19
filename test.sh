#!/usr/bin/env bash

cargo build --release --all

for exe in $(find . -regex "./target/release/[0-9]+.*" ); do
  if [ -x $exe ]; then
    for input in test-data/input/*.txt ; do
      echo -n "Testing  $exe with input $input ... "
      /usr/bin/time -f "%Us %Mkb" -o time.tmp.txt $exe $input >output.tmp.txt
      diff output.tmp.txt ${input/input/output}
      result=$?
      if [ $result -ne 0 ]; then
        echo failed
        exit 1
      else
        echo passed in `cat time.tmp.txt`
      fi
    done
  fi
done