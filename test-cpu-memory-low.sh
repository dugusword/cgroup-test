#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" 
cgexec -g cpu,memory:biye:lowmem $DIR/cgroup-test/target/release/cgroup-test
