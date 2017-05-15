#!/usr/bin/env bash

cargo build --all

for exe in $(find . -regex "./target/debug/[0-9]+.*" ); do
  if [ -x $exe ]; then
    for input in test-data/input/*.txt ; do
      echo -n "Testing  $exe with input $input ... "
      $exe $input >output.tmp.txt
      diff output.tmp.txt ${input/input/output}
      result=$?
      if [ $result -ne 0 ]; then
        echo failed
        exit 1
      else
        echo passed
      fi
    done
  fi
done