#!/bin/bash

#xargs -P 3 -I {} sh -c 'eval "$1"' - {} <<'EOF'

#parallel --group --jobs=1 --verbose

parallel --line-buffer ::: "./target/release/pi 0 /run/media/jadson/e6d2b096-64e5-431c-91df-53e50c4b1676/100b-pi.txt" "./target/release/pi 1 /run/media/jadson/e6d2b096-64e5-431c-91df-53e50c4b1676/100b-pi.txt" "./target/release/pi 2 /run/media/jadson/e6d2b096-64e5-431c-91df-53e50c4b1676/100b-pi.txt" "./target/release/pi 3 /run/media/jadson/e6d2b096-64e5-431c-91df-53e50c4b1676/100b-pi.txt" "./target/release/pi 4 /run/media/jadson/e6d2b096-64e5-431c-91df-53e50c4b1676/100b-pi.txt" "./target/release/pi 5 /run/media/jadson/e6d2b096-64e5-431c-91df-53e50c4b1676/100b-pi.txt" "./target/release/pi 6 /run/media/jadson/e6d2b096-64e5-431c-91df-53e50c4b1676/100b-pi.txt" "./target/release/pi 7 /run/media/jadson/e6d2b096-64e5-431c-91df-53e50c4b1676/100b-pi.txt" "./target/release/pi 8 /run/media/jadson/e6d2b096-64e5-431c-91df-53e50c4b1676/100b-pi.txt" | tr -s ab
