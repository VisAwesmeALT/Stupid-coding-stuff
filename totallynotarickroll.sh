#!/bin/bash

# Countdown from 10 to 1
for i in {10..1}; do
  echo "Counting down: $i"
  sleep 1
done

# Redirect to Rickroll
xdg-open "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
