#!/bin/bash

curl -X POST \
     --output output.form.wav \
     -d "INPUT_TEXT='Close the windows, its warmer outside than inside is'" \
     -d "INPUT_TYPE=TEXT" \
     -d "OUTPUT_TYPE=AUDIO" \
     -d "LOCALE=en_US" \
     -d "AUDIO=WAVE_FILE" \
     -H "Content-Type:" \
     "http://localhost:8080/process"
