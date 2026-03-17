#!/bin/sh

for i in {1..10}
do
   curl -s http://127.0.0.1:8080/announcements > /dev/null &
done