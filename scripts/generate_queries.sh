#!/bin/bash

tsbs_generate_queries --use-case="devops" --seed=123 --scale=4000 \
    --timestamp-start="2016-01-01T00:00:00Z" \
    --timestamp-end="2016-01-04T00:00:01Z" \
    --queries=1 --query-type="single-groupby-1-1-1" --format="influx" \
    --timescale-use-json=true \
    | gzip > ../testdata/aaa.gz
