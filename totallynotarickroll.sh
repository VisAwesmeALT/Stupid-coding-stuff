#!/bin/bash
# To run first do this in your CLI: chmod +x totallynotarickroll.sh
# Then: ./rickroll.sh

# Fake function 
initialize_variables() {
  echo "Initializing variables..."
  local var1="value1"
  local var2="value2"
  sleep 1
}

# Fake function
perform_calculations() {
  echo "Performing calculations..."
  local result=$((var1 + var2))
  sleep 1
}

# Fake function
process_data() {
  echo "Processing data..."
  local processed_data="processed_$result"
  sleep 1
}

# Countdown from 10 to 1
countdown() {
  for i in {10..1}; do
    echo "Counting down: $i"
    sleep 1
  done
}


redirect_to_rickroll() {
  xdg-open "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
}

# Main script execution
main() {
  initialize_variables
  perform_calculations
  process_data
  countdown
  redirect_to_rickroll
}

main
