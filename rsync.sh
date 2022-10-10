#!/bin/bash
rm *.data
rsync -avz  --exclude "target"  ./ ic:/home/wan/rsuse-latency-tool