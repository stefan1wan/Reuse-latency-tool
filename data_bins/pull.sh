#!/bin/bash

scp ic:/mnt/wan/test/media_streaming/branch_bins.data  ./media_streaming_branch_bins.data
scp ic:/mnt/wan/test/media_streaming/bins.data  ./media_streaming_bins.data

scp ic:/mnt/wan/test/web_serving_db/branch_bins.data  ./web_serving_db_branch_bins.data
scp ic:/mnt/wan/test/web_serving_db/bins.data  ./web_serving_db_bins.data

scp ic:/mnt/wan/test/web_serving_web_server/branch_bins.data ./web_serving_web_server_branch_bins.data
scp ic:/mnt/wan/test/web_serving_web_server/bins.data ./web_serving_web_server_bins.data

scp ic:/mnt/wan/test/web_search/branch_bins.data   ./web_search_branch_bins.data
scp ic:/mnt/wan/test/web_search/bins.data   ./web_search_bins.data

scp ic:/mnt/wan/test/data_caching/branch_bins.data ./data_caching_branch_bins.data
scp ic:/mnt/wan/test/data_caching/bins.data ./data_caching_bins.data