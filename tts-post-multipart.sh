#!/bin/bash

curl -X POST \
     --output output.multipart.wav \
     -F "INPUT_TEXT='Close the windows, its warmer outside than inside'" \
     -F "INPUT_TYPE=TEXT" \
     -F "OUTPUT_TYPE=AUDIO" \
     -F "LOCALE=en_GB" \
     -F "AUDIO=WAVE_FILE" \
     "http://localhost:8080/process"
