#!/bin/bash
find $1 -type f | xargs -l sed -i -E "s/day[[:digit:]]+/$1/"