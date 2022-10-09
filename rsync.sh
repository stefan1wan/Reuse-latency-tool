#!/bin/bash
rm *.data
rsync -avz  --exclude "target"  ./ ic-remote:/home/wan/rsuse-latency-tool